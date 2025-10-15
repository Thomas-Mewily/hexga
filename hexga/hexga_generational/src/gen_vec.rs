use super::*;


pub mod prelude
{
    pub use super::{GenVec, CollectToGenVecExtension};
}


pub type GenVec<T> = GenVecOf<T,Generation>;


#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryValue<T>
{
    Used(T),
    // Next free
    Free(usize),
}
impl<T> EntryValue<T>
{
    pub fn get(&self) -> Option<&T> { if let Self::Used(v) = self { Some(v) } else { None }}
    pub fn get_mut(&mut self) -> Option<&mut T> { if let Self::Used(v) = self { Some(v) } else { None }}

    /// Panic is the entry is free
    pub fn take_and_free(&mut self, free_index: usize) -> T {
        match std::mem::replace(self, EntryValue::Free(free_index)) {
            EntryValue::Used(value) => value,
            EntryValue::Free(_) => panic!("Entry was already free"),
        }
    }

    pub fn is_free(&self) -> bool { matches!(self, Self::Free(_))}
    pub fn is_used(&self) -> bool { matches!(self, Self::Used(_))}
}

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entry<T,Gen:IGeneration=Generation>
{
    pub(crate) value: EntryValue<T>,
    #[cfg_attr(feature = "serde", serde(rename = "gen"))]
    generation : Gen,
}
impl <T,Gen:IGeneration> Entry<T,Gen>
{
    pub fn new(value : EntryValue<T>, generation : Gen) -> Self { Self { value, generation }}
    pub fn generation(&self) -> Gen { self.generation }

    pub fn have_value(&self) -> bool { self.value().is_some() }

    pub fn value(&self) -> Option<&T> { self.value.get() }
    pub fn value_mut(&mut self) -> Option<&mut T> { self.value.get_mut() }

    pub fn get_id(&self, index: usize) -> GenIDOf<T,Gen> { GenIDOf::from_index_and_generation(index, self.generation) }

    pub fn increment_generation(&mut self) -> bool { if self.can_increment_generation() { self.generation.increment(); true } else { false } }
    pub fn can_increment_generation(&self) -> bool { self.generation.can_increment() }

    pub fn decrement_generation(&mut self) -> bool { if self.can_decrement_generation() { self.generation.decrement(); true } else { false } }
    pub fn can_decrement_generation(&self) -> bool { self.generation.can_decrement() }

    pub fn is_generation_saturated(&self) -> bool { !self.can_increment_generation() }
}

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Eq)]
pub struct GenVecOf<T,Gen:IGeneration=Generation>
{
    pub(crate) values: Vec<Entry<T,Gen>>,
    head: usize,
    len: usize,
}

impl<T, Gen:IGeneration> Hash for GenVecOf<T,Gen> where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.len.hash(state);

        if !Gen::OVERFLOW_BEHAVIOR.is_wrapping()
        {
            self.values.hash(state);
            self.head.hash(state);
        }else
        {
            for (id, value) in self.iter()
            {
                id.hash(state);
                value.hash(state);
            }
        }
    }
}

impl<T, Gen:IGeneration> PartialEq for GenVecOf<T,Gen> where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool
    {
        if !Gen::OVERFLOW_BEHAVIOR.is_wrapping()
        {
            self.len == other.len && self.values == other.values && self.head == other.head
        }else
        {
            /*
                We can't know if the gen vec is new or if the gen vec just wrapped arround.

                Those two are equal: (Assuming Gen::MIN value is 0)
                A: GenVecOf { entry: [Entry { value: Free(18446744073709551615), generation: 0 }], head: 0, len: 0 }
                B: GenVecOf { entry: [], head: 18446744073709551615, len: 0 }

                Both can represent unused wrapped gen vec.

                doing :

                let id = B.insert(10);
                B.rollback_insert(id);

                will put A in the same equal state/representation as B.


                But these 2 are different, because that generation was already used.

                X: GenVecOf { entry: [Entry { value: Free(18446744073709551615), generation: 1 }], head: 0, len: 0 }
                Y: GenVecOf { entry: [], head: 18446744073709551615, len: 0 }
            */

            if self.len != other.len { return false; }
            if self.head == other.head { return self.values == other.values; }
            if !(self.head.is_max_value() ^ other.head.is_max_value()) { return false; }

            if self.head.is_max_value()
            {
                if self.values.len() + 1 != other.values.len() { return false; }
                let mid = other.head;
                debug_assert!(!mid.is_max_value());

                let entry = other.get_entry_from_index(mid).unwrap();
                let EntryValue::Free(f) = entry.value else { return false; };
                if !f.is_max_value() || !entry.generation().is_min_value() { return false; }

                let self_left = &self.values[0..mid];
                let self_right = &self.values[mid..];

                let other_left = &other.values[0..mid];
                let other_right = &other.values[mid+1..];

                self_left == other_left && self_right == other_right
            }else if other.head.is_max_value()
            {
                if other.values.len() + 1 != self.values.len() { return false; }
                let mid = self.head;
                debug_assert!(!mid.is_max_value());

                let entry = self.get_entry_from_index(mid).unwrap();
                let EntryValue::Free(f) = entry.value else { return false; };
                if !f.is_max_value() || !entry.generation().is_min_value() { return false; }

                let other_left = &other.values[0..mid];
                let other_right = &other.values[mid..];

                let self_left = &self.values[0..mid];
                let self_right = &self.values[mid+1..];

                other_left == self_left && other_right == self_right
            }else
            {
                unreachable!()
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<T, Gen:IGeneration> Serialize for GenVecOf<T,Gen> where T:Serialize, Gen: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {

        let mut state = serializer.serialize_struct("GenVec", 1 + self.head.is_not_max_value() as usize)?;
        state.serialize_field("values", &self.values)?;
        if self.head.is_not_max_value()
        {
            // need to be in the same order on all machine for determinist
            state.serialize_field("free", &Some(self.head))?;
        }
        state.end()
    }
}


impl<T, Gen:IGeneration> GenVecOf<T,Gen>
{
    #[allow(dead_code)]
    pub(crate) fn from_entries_and_head(values: Vec<Entry<T, Gen>>, head: usize) -> Result<Self, String>
    {
        let len = values.iter().filter(|s| s.have_value()).count();

        if values.len() == usize::MAX
        {
            return Err("GenVec : the last usize value is used for null in a GenVec and cannot be used".to_owned());
        }

        let mut nb_use = len;
        let mut cur_head = head;

        while nb_use != 0
        {
            let Some(next_entry) = values.get(cur_head) else { return Err(format!("GenVec : entry {:?} is out of range", cur_head)); };
            let EntryValue::Free(f) = next_entry.value else { return Err(format!("GenVec : entry {:?} was not free", cur_head)); };
            if f == usize::MAX { return Err(format!("GenVec : invalid free head {:?} at {:?}", f, cur_head));}
            cur_head = f;
            nb_use -= 1;
        }

        Ok(Self{ values, head, len})
    }
}


#[cfg(feature = "serde")]
impl<'de,T, Gen> Deserialize<'de> for GenVecOf<T, Gen>
where
    Gen: IGeneration + Deserialize<'de>,
    Entry<T, Gen>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct GenVecVisitor<T, Gen> {
            marker: std::marker::PhantomData<(T, Gen)>,
        }

        impl<'de, T, Gen> Visitor<'de> for GenVecVisitor<T, Gen>
        where
            Gen: IGeneration + Deserialize<'de>,
            Entry<T, Gen>: Deserialize<'de>,
        {
            type Value = GenVecOf<T, Gen>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a struct representing GenVec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values : Option<Vec<Entry<T, Gen>>> = None;
                let mut free_index: Option<Option<usize>> = None;

                while let Some(key) = map.next_key::<&'de str>()?
                {
                    match key
                    {
                        "values" => {
                            if values.is_some() {
                                return Err(de::Error::duplicate_field("values"));
                            }
                            values = Some(map.next_value()?);
                        }
                        "free" => {
                            if free_index.is_some() {
                                return Err(de::Error::duplicate_field("free"));
                            }
                            free_index = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(
                                &key,
                                &["values", "free"],
                            ));
                        }
                    }
                }

                let entry = values.ok_or_else(|| de::Error::missing_field("values"))?;
                let free = free_index.flatten().unwrap_or(usize::MAX);
                GenVecOf::<T,Gen>::from_entries_and_head(entry, free).map_err(|e| de::Error::custom(e))
            }
        }

        const FIELDS: &[&str] = &["values", "free"];
        deserializer.deserialize_struct(
            "GenVec",
            FIELDS,
            GenVecVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}


impl<T,Gen:IGeneration> Default for GenVecOf<T,Gen>
{
    fn default() -> Self { Self::new() }
}

impl<T,Gen:IGeneration> GenVecOf<T,Gen>
{
    pub const fn new() -> Self { Self { values: Vec::new(), head : usize::MAX, len : 0 }}
    pub fn with_capacity(capacity : usize) -> Self { Self { values: Vec::with_capacity(capacity), head : usize::MAX, len : 0 }}

    pub fn capacity(&self) -> usize { self.values.capacity() }
    pub fn shrink_to_fit(mut self) { self.values.shrink_to_fit(); }


    /// Clears the [`GenVec`], removing all elements and resetting all [`GenID`] values.
    ///
    /// After calling this method, any previous [`GenID`] is no longer valid (not enforced) and
    /// **must** not be used, as doing so may lead to undefined behavior.
    pub fn clear(&mut self)
    {
        self.head = usize::MAX;
        self.len = 0;
        self.values.clear();
    }

    /// Removes all elements from the [`GenVec`] and invalidates all existing [`GenID`] (enforced).
    pub fn remove_all(&mut self)
    {
        for (index, v) in self.values.iter_mut().enumerate()
        {
            if v.have_value()
            {
                if v.increment_generation()
                {
                    v.value = EntryValue::Free(self.head);
                    self.head = index;
                }else
                {
                    v.value = EntryValue::Free(usize::MAX);
                }
            }
        }
        self.len = 0;
    }

    pub fn rollback_insert(&mut self, id: GenIDOf<T,Gen>) -> Result<T,()>
    {
        let index = id.index();
        let head = self.head;

        let entry_len = self.values.len();

        let Some(entry) = self.get_entry_mut_from_index(index) else { return Err(()); };
        if entry.value.is_free() { return Err(()); }

        if head.is_max_value()
        {
            if index + 1 != entry_len { return Err(()); }
        }

        let can_not_decrease = !entry.can_decrement_generation();
        let val = entry.value.take_and_free(head);
        self.len -= 1;

        if head.is_max_value() && can_not_decrease
        {
            self.values.pop().ok_or(())?;
        }else
        {
            self.head = index;
        }

        Ok(val)
    }
    pub fn insert(&mut self, value: T) ->  GenIDOf<T,Gen>
    {
        self.len += 1;

        if self.head == usize::MAX
        {
            let index = self.values.len();

            // The last index is used for the null() key
            assert!(index != usize::MAX, "How you didn't run out of memory before ?");

            let generation = Gen::MIN;
            self.values.push(Entry { value: EntryValue::Used(value), generation });
            return GenIDOf::from_index_and_generation(index, generation);
        }

        let EntryValue::Free(next_free_index) = self.values[self.head].value else { unreachable!(); };
        let head = self.head;
        self.head = next_free_index;
        self.values[head].value = EntryValue::Used(value);
        return GenIDOf::from_index_and_generation(head, self.values[head].generation);
    }

    #[inline(always)]
    pub fn get_entry_from_index(&self, index: usize) -> Option<&Entry<T,Gen>> { self.values.get(index) }
    #[inline(always)]
    pub(crate) fn get_entry_mut_from_index(&mut self, index: usize) -> Option<&mut Entry<T,Gen>> { self.values.get_mut(index) }

    #[inline(always)]
    pub fn get_from_index(&self, index: usize) -> Option<&T> { self.get_entry_from_index(index).and_then(|s| s.value()) }
    #[inline(always)]
    pub fn get_mut_from_index(&mut self, index: usize) -> Option<&mut T> { self.get_entry_mut_from_index(index).and_then(|s| s.value_mut()) }

    #[inline(always)]
    pub fn get_entry(&self, id: GenIDOf<T,Gen>) -> Option<&Entry<T,Gen>> { self.get_entry_from_index(id.index()).filter(|v| v.generation() == id.generation()) }
    #[inline(always)]
    pub(crate) fn get_entry_mut(&mut self, id: GenIDOf<T,Gen>) -> Option<&mut Entry<T,Gen>> { self.get_entry_mut_from_index(id.index()).filter(|v| v.generation() == id.generation()) }

    #[inline(always)]
    pub fn get(&self, id: GenIDOf<T,Gen>) -> Option<&T> { self.get_entry(id).and_then(|v| v.value()) }
    #[inline(always)]
    pub fn get_mut(&mut self, id: GenIDOf<T,Gen>) -> Option<&mut T> { self.get_entry_mut(id).and_then(|v| v.value_mut()) }

    /// Return a valid [`GenID`] to the current index or return [`GenIDOf::NULL`] if the index is outside the range
    pub fn index_to_id(&self, index: usize) -> GenIDOf<T, Gen>
    {
        self.get_entry_from_index(index).map(|v| v.get_id(index)).unwrap_or(GenIDOf::NULL)
    }

    /// The operation that once done just after an [`Self::remove_from_index`], put this data structure in the same state as before
    pub fn rollback_remove_index(&mut self, index: usize, value: T) -> Result<(), ()>
    {
        let mut head = self.head;
        let entry = self.get_entry_mut_from_index(index).ok_or(())?;
        let EntryValue::Free(f) = entry.value else { return Err(()); };
        let free = f;

        if f.is_not_max_value()
        {
            if head != index { return Err(()); }
            head = free;
            if !entry.decrement_generation() { return Err(()); }
        }else
        {
            // Entry don't have a next free entry
            if head == index
            {
                head = usize::MAX;
                if !entry.decrement_generation() { return Err(()); }
            }else if !entry.is_generation_saturated()
            {
                return Err(());
            }
        }

        entry.value = EntryValue::Used(value);

        self.head = head;
        self.len += 1;

        Ok(())
    }

    pub fn remove_from_index(&mut self, index: usize) -> Option<T>
    {
        let head = self.head;

        let Some(entry) = self.get_entry_mut_from_index(index) else { return None; };
        if entry.value.is_free() { return None; }

        let val = entry.value.take_and_free(head);

        if entry.increment_generation()
        {
            self.head = index;
        }else
        {
            entry.value = EntryValue::Free(usize::MAX);
        }
        self.len -= 1;

        Some(val)
    }

    pub fn rollback_remove(&mut self, id: GenIDOf<T,Gen>, value: T) -> Result<(), ()>
    {
        // Todo : missing some check to see if the last operation removal was done with id
        self.rollback_remove_index(id.index(), value)
    }

    pub fn remove(&mut self, id: GenIDOf<T,Gen>) -> Option<T>
    {
        if self.get(id).is_none() { return None; }
        self.remove_from_index(id.index())
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry<T,Gen>> { self.values.iter() }

    pub const fn len(&self) -> usize { self.len }

    pub fn iter(&self) -> Iter<'_, T, Gen> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_, T, Gen> { self.into_iter() }

    pub fn ids(&self) -> impl Iterator<Item = GenIDOf<T,Gen>> { self.into_iter().map(|(id, _val)| id) }

    pub fn values(&self) -> impl Iterator<Item = &T> { self.iter().map(|(_,val)| val) }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> { self.iter_mut().map(|(_,val)| val) }

    pub fn into_ids(self) -> impl Iterator<Item = GenIDOf<T,Gen>> { self.into_iter().map(|(id, _val)| id) }
    pub fn into_values(self) -> impl Iterator<Item = T> { self.into_iter().map(|(_id, val)| val) }

    /// Iter over all entry index, including the free/unused one.
    ///
    /// The correct way to iterate over all entry index.
    ///
    /// Use this instead of `0..genvec.len()`.
    pub fn iter_index(&self) -> impl Iterator<Item = usize> + use<T, Gen> { 0..self.values.len() }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(id, v)` for which `f(id, &v)` returns `false`. The elements are visited in unsorted (and unspecified) order.
    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(GenIDOf<T,Gen>, &T) -> bool
    {
        self.retain_mut(|id,elem| f(id,elem));
    }

        /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(id, v)` for which `f(id, &mut v)` returns `false`. The elements are visited in unsorted (and unspecified) order.
    pub fn retain_mut<F>(&mut self, mut f: F) where F: FnMut(GenIDOf<T,Gen>, &mut T) -> bool
    {
        for index in self.iter_index()
        {
            let entry = self.get_entry_mut_from_index(index).unwrap();
            let id = entry.get_id(index);
            let Some(v) = entry.value_mut() else { continue; };
            if !f(id, v)
            {
                self.remove_from_index(index);
            }
        }
    }
}

impl<T, Gen:IGeneration> Index<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    type Output=T;
    fn index(&self, index: GenIDOf<T,Gen>) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Gen:IGeneration> IndexMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    fn index_mut(&mut self, index: GenIDOf<T,Gen>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Gen:IGeneration> Index<usize> for GenVecOf<T,Gen>
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_from_index(index).unwrap() }
}
impl<T, Gen:IGeneration> IndexMut<usize> for GenVecOf<T,Gen>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut_from_index(index).unwrap() }
}

impl<T, Gen:IGeneration> FromIterator<T> for GenVecOf<T, Gen>
{
    fn from_iter<K: IntoIterator<Item = T>>(iter: K) -> Self {
        let values : Vec<Entry<T,Gen>> = iter.into_iter().map(|v| Entry::new(EntryValue::Used(v), Gen::MIN)).collect();
        let len = values.len();
        Self{ values, head: usize::MAX, len }
    }
}

impl<T, Gen: IGeneration> IntoIterator for GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, T);
    type IntoIter = IntoIter<T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter
        {
            iter: self.values.into_iter().enumerate(),
            len_remaining: self.len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter<T, Gen: IGeneration>
{
    iter: std::iter::Enumerate<std::vec::IntoIter<Entry<T, Gen>>>,
    len_remaining : usize,
}

impl<T, Gen: IGeneration> Iterator for IntoIter<T, Gen> {
    type Item = (GenIDOf<T, Gen>, T);

    fn next(&mut self) -> Option<Self::Item>
    {
        while let Some((index, entry)) = self.iter.next()
        {
            if let EntryValue::Used(value) = entry.value
            {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, entry.generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<T, Gen: IGeneration> FusedIterator for IntoIter<T, Gen> {}
impl<T, Gen: IGeneration> ExactSizeIterator for IntoIter<T, Gen> { fn len(&self) -> usize { self.len_remaining } }

impl<'a, T, Gen: IGeneration> IntoIterator for &'a GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a T);
    type IntoIter = Iter<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.values.iter().enumerate(),
            len_remaining : self.len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T, Gen: IGeneration>
{
    iter: std::iter::Enumerate<std::slice::Iter<'a, Entry<T, Gen>>>,
    len_remaining : usize,
}

impl<'a, T, Gen: IGeneration> Iterator for Iter<'a, T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.iter.next() {
            if let Some(value) = entry.value() {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, entry.generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<'a, T, Gen: IGeneration> FusedIterator for Iter<'a, T, Gen> {}
impl<'a, T, Gen: IGeneration> ExactSizeIterator for Iter<'a, T, Gen> { fn len(&self) -> usize { self.len_remaining } }



impl<'a, T, Gen: IGeneration> IntoIterator for &'a mut GenVecOf<T, Gen>
{
    type Item = (GenIDOf<T, Gen>, &'a mut T);
    type IntoIter = IterMut<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.values.iter_mut().enumerate(),
            len_remaining : self.len,
        }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T, Gen: IGeneration>
{
    iter: std::iter::Enumerate<std::slice::IterMut<'a, Entry<T, Gen>>>,
    len_remaining : usize,
}

impl<'a, T, Gen: IGeneration> Iterator for IterMut<'a, T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.iter.next() {
            let generation = entry.generation();
            if let Some(value) = entry.value_mut() {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<'a, T, Gen: IGeneration> FusedIterator for IterMut<'a, T, Gen> {}
impl<'a, T, Gen: IGeneration> ExactSizeIterator for IterMut<'a, T, Gen> { fn len(&self) -> usize { self.len_remaining } }


impl<T,Gen:IGeneration> Length for GenVecOf<T,Gen> { #[inline(always)] fn len(&self) -> usize { self.len() } }
impl<T,Gen:IGeneration> Clearable for GenVecOf<T,Gen> { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T,Gen:IGeneration> Capacity for GenVecOf<T,Gen>
{
    type Param=();

    #[inline(always)]
    fn capacity(&self) -> usize { self.values.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.values.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.values.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve_exact(additional) }
}

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenVecError<T,Gen:IGeneration>
{
    IndexOutOfRange(IndexOutOfRange),
    WrongGeneration(GenVecWrongGeneration<T,Gen>),
    /// The entry at this index is saturated
    Saturated(usize),
}
impl<T,Gen:IGeneration> Eq for GenVecError<T,Gen> {}
impl<T,Gen:IGeneration> PartialEq for GenVecError<T,Gen>
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IndexOutOfRange(l0), Self::IndexOutOfRange(r0)) => l0 == r0,
            (Self::WrongGeneration(l0), Self::WrongGeneration(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<T,Gen:IGeneration> Hash for GenVecError<T,Gen>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self
        {
            GenVecError::IndexOutOfRange(v) => v.hash(state),
            GenVecError::WrongGeneration(v) => v.hash(state),
            GenVecError::Saturated(v) => v.hash(state),
        }
    }
}

// error : the `Copy` impl for `hexga_core::collections::IndexOutOfRange` requires that `std::ops::Range<usize>: Copy`
// impl<T,Gen:IGeneration> Copy for GenVecError<T,Gen> {}
impl<T,Gen:IGeneration> Clone for GenVecError<T,Gen>
{
    fn clone(&self) -> Self {
        match self {
            GenVecError::IndexOutOfRange(v) => Self::IndexOutOfRange(v.clone()),
            GenVecError::WrongGeneration(v) => Self::WrongGeneration(v.clone()),
            GenVecError::Saturated(v) => Self::Saturated(v.clone()),
        }
    }
}

impl<T,Gen:IGeneration> Debug for GenVecError<T,Gen>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenVecError::IndexOutOfRange(arg0) => f.debug_tuple("IndexOutOfRange").field(arg0).finish(),
            GenVecError::WrongGeneration(arg0) => f.debug_tuple("WrongGeneration").field(arg0).finish(),
            GenVecError::Saturated(arg0) => f.debug_tuple("Saturated").field(arg0).finish(),
        }
    }
}

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct GenVecWrongGeneration<T,Gen:IGeneration>
{
    pub got : Gen,
    pub expected : Gen,
    phantom : PhantomData<T>,
}


#[cfg(feature = "serde")]
impl<T, Gen: IGeneration> Serialize for GenVecWrongGeneration<T, Gen>
where
    Gen: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("GenVecWrongGeneration", 2)?;
        state.serialize_field("got", &self.got)?;
        state.serialize_field("expected", &self.expected)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T, Gen: IGeneration> Deserialize<'de> for GenVecWrongGeneration<T, Gen>
where
    Gen: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Helper<Gen> {
            got: Gen,
            expected: Gen,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(GenVecWrongGeneration::new(helper.got, helper.expected))
    }
}

impl<T,Gen:IGeneration> GenVecWrongGeneration<T,Gen>
{
    pub fn new(got : Gen, expected : Gen) -> Self { Self{ got, expected, phantom: PhantomData }}
}
impl<T,Gen:IGeneration> Eq for GenVecWrongGeneration<T,Gen> {}
impl<T,Gen:IGeneration> PartialEq for GenVecWrongGeneration<T,Gen>
{
    fn eq(&self, other: &Self) -> bool { self.got == other.got && self.expected == other.expected }
}

impl<T,Gen:IGeneration> Hash for GenVecWrongGeneration<T,Gen>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.got.hash(state);
        self.expected.hash(state);
    }
}


impl<T,Gen:IGeneration> Copy for GenVecWrongGeneration<T,Gen> {}
impl<T,Gen:IGeneration> Clone for GenVecWrongGeneration<T,Gen>
{
    fn clone(&self) -> Self {
        Self { got: self.got.clone(), expected: self.expected.clone(), phantom: PhantomData }
    }
}

impl<T,Gen:IGeneration> Debug for GenVecWrongGeneration<T,Gen>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenVecWrongGeneration").field("got", &self.got).field("expected", &self.expected).field("phantom", &self.phantom).finish()
    }
}


impl<T,Gen:IGeneration> TryGet<usize> for GenVecOf<T,Gen>
{
    type Error=IndexOutOfRange;
    fn try_get(&self, index: usize) -> Result<&Self::Output, Self::Error>
    {
        self.get_from_index(index).ok_or_else(|| IndexOutOfRange::new(index, 0..self.len()))
    }
}
impl<T,Gen:IGeneration> Get<usize> for GenVecOf<T,Gen>
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn get(&self, index: usize) -> Option<&Self::Output> { self.get_from_index(index) }
}

impl<T,Gen:IGeneration> TryGet<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    type Error=GenVecError<T,Gen>;
    fn try_get(&self, id :  GenIDOf<T,Gen>) -> Result<&Self::Output, Self::Error>
    {
        match self.get_entry_from_index(id.index())
        {
            Some(s) => match s.value()
            {
                Some(v) => Ok(v),
                None => if s.is_generation_saturated()
                {
                    Err(GenVecError::Saturated(id.index()))
                }else
                {
                    Err(GenVecError::WrongGeneration(GenVecWrongGeneration::new(id.generation(), s.generation())))
                },
            },
            None => Err(GenVecError::IndexOutOfRange(IndexOutOfRange::new(id.index(), 0..self.len()))),
        }
    }
}
impl<T,Gen:IGeneration> Get<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    type Output = <Self as Index<GenIDOf<T,Gen>>>::Output;
    #[inline(always)]
    fn get(&self, index: GenIDOf<T,Gen>) -> Option<&Self::Output> { self.get(index) }
}

impl<T,Gen:IGeneration> TryGetMut<usize> for GenVecOf<T,Gen>
{
    fn try_get_mut(&mut self, index: usize) -> Result<&mut Self::Output, Self::Error>
    {
        let len = self.len();
        self.get_mut_from_index(index).ok_or_else(|| IndexOutOfRange::new(index, 0..len))
    }
}
impl<T,Gen:IGeneration> GetMut<usize> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> { self.get_mut_from_index(index) }
}

impl<T,Gen:IGeneration> GetManyMut<usize> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        // TODO: check the SlotMap crate for a better implementation of any overlapping indices
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.try_get_many_mut(indices).map(|entries| entries.map(|v| v.value_mut()))
        {
            Ok(values) => if values.iter().any(|v| v.is_none()) { Err(ManyMutError::IndexOutOfBounds) } else { Ok(values.map(|v| v.unwrap())) },
            Err(e) => Err(e),
        }
    }

    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut Self::Output;N]> {
        // TODO: check the SlotMap crate for a better implementation of any overlapping indices
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.get_many_mut(indices).map(|entries| entries.map(|v| v.value_mut()))
        {
            Some(values) => if values.iter().any(|v| v.is_none()) { None } else { Some(values.map(|v| v.unwrap())) },
            None => None,
        }
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut Self::Output;N] {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        unsafe { self.values.get_many_unchecked_mut(indices).map(|v| v.value_mut().unwrap()) }
    }
}

impl<T,Gen:IGeneration> TryGetMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    fn try_get_mut(&mut self, id :  GenIDOf<T,Gen>) -> Result<&mut Self::Output, Self::Error>
    {
        let len = self.len();
        match self.get_entry_mut_from_index(id.index())
        {
            Some(s) =>
            {
                let generation = s.generation();
                let is_saturated = s.is_generation_saturated();
                match s.value_mut()
                {
                    Some(v) => Ok(v),
                    None => if is_saturated
                    {
                        Err(GenVecError::Saturated(id.index()))
                    }else
                    {
                        Err(GenVecError::WrongGeneration(GenVecWrongGeneration::new(id.generation(), generation)))
                    }
                }
            }
            None => Err(GenVecError::IndexOutOfRange(IndexOutOfRange::new(id.index(), 0..len))),
        }
    }
}
impl<T,Gen:IGeneration> GetMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn get_mut(&mut self, index: GenIDOf<T,Gen>) -> Option<&mut Self::Output> { self.get_mut(index) }
}

impl<T,Gen:IGeneration> GetManyMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [GenIDOf<T,Gen>; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        // Todo: use O(N) complexity to check the overlaping
        // Check SlotMap imply that put tmp Free slot/entry in the current indices to

        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.try_get_many_mut(indices.map(|id| id.index()))
        {
            Ok(values) => if values.iter().enumerate().any(|(index,v)| !v.have_value() || v.generation() != indices[index].generation())
            { Err(std::slice::GetDisjointMutError::OverlappingIndices) } else { Ok(values.map(|v| v.value_mut().unwrap())) },
            Err(e) => Err(e),
        }
    }

    fn get_many_mut<const N: usize>(&mut self, indices: [GenIDOf<T,Gen>; N]) -> Option<[&mut Self::Output;N]>
    {
        // Todo: use O(N) complexity to check the overlaping
        // Check SlotMap imply that put tmp Free slot/entry in the current indices to

        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.get_many_mut(indices.map(|id| id.index()))
        {
            Some(values) =>
                if values.iter().enumerate().any(|(index,v)| !v.have_value() || v.generation() != indices[index].generation())
                { None } else { Some(values.map(|v| v.value_mut().unwrap())) },
            None => None,
        }
    }
}
impl<T,Gen:IGeneration> Remove<usize> for GenVecOf<T,Gen>
{
    type Output=T;
    fn remove(&mut self, index: usize) -> Option<Self::Output> {
        self.remove_from_index(index)
    }
}

impl<T,Gen:IGeneration> Remove<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    type Output=T;
    fn remove(&mut self, index: GenIDOf<T,Gen>) -> Option<Self::Output> {
        self.remove(index)
    }
}


impl<T,Gen:IGeneration> GenVecOf<T,Gen>
{
    /// Moves all the elements of `other` into `self`, leaving `other` empty by clearing it (don't invalidate all previous [GenID]).
    pub fn append(&mut self, other: &mut GenVecOf<T,Gen>) -> impl GenIDUpdater<T,Gen> + use<T,Gen> where T: GenIDUpdatable<T,Gen>
    {
        let capacity = other.len();
        let mut h = HashMap::with_capacity(capacity);

        for (index, entry) in other.values.iter_mut().enumerate().filter(|(_,s)| s.have_value())
        {
            let val = entry.value.take_and_free(usize::MAX);
            let old_id = entry.get_id(index);
            let new_id = self.insert(val);
            h.insert(old_id, new_id);
        }
        other.clear();

        for new_id in h.values()
        {
            unsafe { self.get_unchecked_mut(*new_id) }.update_id(&h);
        }
        h
    }
}

impl<A,Gen:IGeneration> Extend<A> for GenVecOf<A,Gen>
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T)
    {
        for val in iter.into_iter()
        {
            self.insert(val);
        }
    }
}

pub trait GenIDUpdater<T,Gen:IGeneration>
{
    fn update(&self, dest : &mut GenIDOf<T,Gen>);
}
impl<T,Gen:IGeneration> GenIDUpdater<T,Gen> for HashMap<GenIDOf<T,Gen>,GenIDOf<T,Gen>>
{
    fn update(&self, dest : &mut GenIDOf<T,Gen>) {
        debug_assert!(dest.is_null() || self.get(&dest).is_some());
        *dest = self.get(&dest).copied().unwrap_or(GenIDOf::NULL);
    }
}


pub trait GenIDUpdatable<T=Self,Gen:IGeneration=Generation> : Sized
{
    fn update_id<U : GenIDUpdater<T,Gen>>(&mut self, updater : &U);
}
impl<T,Gen:IGeneration> GenIDUpdatable<T,Gen> for GenIDOf<T,Gen>
{
    fn update_id<U : GenIDUpdater<T,Gen>>(&mut self, updater : &U) {
        updater.update(self);
    }
}

impl<A,Gen:IGeneration> Extend<(GenIDOf<A,Gen>, A)> for GenVecOf<A,Gen> where A : GenIDUpdatable<A,Gen>
{
    fn extend<T: IntoIterator<Item = (GenIDOf<A,Gen>, A)>>(&mut self, iter: T)
    {
        let it = iter.into_iter();
        let mut h = HashMap::with_capacity(it.size_hint().0);

        for (old_id, val) in it
        {
            let new_id = self.insert(val);
            h.insert(old_id, new_id);
        }

        for new_id in h.values()
        {
            unsafe { self.get_unchecked_mut(*new_id) }.update_id(&h);
        }
    }
}

pub trait CollectToGenVecExtension<T,Gen:IGeneration=Generation> : Sized + IntoIterator<Item = T>
{
    fn to_genvec(self) -> GenVecOf<T,Generation>
    {
        GenVecOf::from_iter(self)
    }
}
impl<I,T1> CollectToGenVecExtension<T1> for I where I : IntoIterator<Item = T1> {}

/*
pub trait CollectToGenVecWithIDExtension<T,Gen:IGeneration=Generation> : Sized + IntoIterator<Item = (GenIDOf<T,Gen>, T)>
{
    fn to_genvec(self) -> GenVecOf<T,Generation>
    {
        GenVecOf::from_iter(self)
    }
}
impl<I,T> CollectToGenVecWithIDExtension<T> for I where I : IntoIterator<Item = (GenIDOf<T,Gen>, T)> {}

impl<I,T> CollectToGenVecWithIndexExtension<T> for I where I : IntoIterator<Item = (usize, T)> {}
*/

#[allow(dead_code)]
#[cfg(test)]
mod tests
{
    use std::num::Wrapping;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Cell
    {
        next : GenID<Cell>,
        value : i32,
    }

    impl GenIDUpdatable for Cell
    {
        fn update_id<U : GenIDUpdater<Self,u32>>(&mut self, updater : &U) {
            self.next.update_id(updater);
        }
    }

    #[test]
    fn extend_complexe_struct()
    {
        let mut src = GenVec::new();
        let first = src.insert(Cell{ next: GenID::NULL, value: 1 });
        src.insert(Cell{ next: first, value: 2 });

        let mut dest = GenVec::new();
        let first = dest.insert(Cell{ next: GenID::NULL, value: 3 });
        dest.insert(Cell{ next: first, value: 4 });

        src.extend(dest.into_iter());

        let ids = src.iter().map(|(_,v)| v.value).collect::<std::collections::HashSet<_>>();
        assert_eq!(ids.len(), 4);
    }


    #[test]
    fn append_complexe_struct()
    {
        let mut src = GenVec::new();
        let first = src.insert(Cell{ next: GenID::NULL, value: 1 });
        src.insert(Cell{ next: first, value: 2 });

        let mut dest = GenVec::new();
        let mut first = dest.insert(Cell{ next: GenID::NULL, value: 3 });
        let mut second = dest.insert(Cell{ next: first, value: 4 });

        let updater = src.append(&mut dest);

        assert_eq!(dest.len(), 0);

        first.update_id(&updater);
        second.update_id(&updater);

        assert_eq!(src[first].next, GenID::NULL);
        assert_eq!(src[first].value, 3);
        assert_eq!(src[second].next, first);
        assert_eq!(src[second].value, 4);
    }

    #[test]
    fn extend_common_struct()
    {
        let mut g = [1,2,3].into_iter().collect::<GenVec<_>>();
        assert_eq!(g.len(), 3);

        g.extend([4,5]);
        assert_eq!(g.len, 5);
    }

    #[test]
    fn iter_size_hint_check()
    {
        let g = [1,2,3,4,5].into_iter().collect::<GenVec<_>>();
        let mut it = g.iter();

        for i in (1..=5).rev()
        {
            assert_eq!(it.size_hint().0, i);
            assert_eq!(it.size_hint().1, Some(i));
            it.next();
        }
    }

    #[test]
    fn iter_mut_size_hint_check()
    {
        let mut g = [1,2,3,4,5].into_iter().collect::<GenVec<_>>();
        let mut it = g.iter_mut();

        for i in (1..=5).rev()
        {
            assert_eq!(it.size_hint().0, i);
            assert_eq!(it.size_hint().1, Some(i));
            it.next();
        }
    }

    #[test]
    fn into_iter_mut_size_hint_check()
    {
        let g = [1,2,3,4,5].into_iter().collect::<GenVec<_>>();
        let mut it = g.into_iter();

        for i in (1..=5).rev()
        {
            assert_eq!(it.size_hint().0, i);
            assert_eq!(it.size_hint().1, Some(i));
            it.next();
        }
    }

    #[test]
    fn basic()
    {
        let mut g = GenVec::new();
        assert_eq!(g.len(), 0);

        let a = g.insert(42);
        assert_eq!(g.len(), 1);
        assert_eq!(g[a], 42);
        assert_eq!(g.get(a), Some(&42));

        let b = g.insert(43);
        assert_eq!(g.len(), 2);
        assert_eq!(g[b], 43);
        assert_eq!(g.get(b), Some(&43));

        assert_eq!(g.remove(a).unwrap(), 42);
        assert_eq!(g.remove(a), None);
        assert_eq!(g.len(), 1);

        assert_eq!(g.remove(b).unwrap(), 43);
        assert_eq!(g.len(), 0);
    }

    #[test]
    fn into_iter()
    {
        assert_eq!(GenVec::<i32>::new().into_iter().next(), None);

        let x = GenVec::from_iter([10,20,30]);
        assert_eq!(x.len(), 3);

        assert_eq!(x[x.index_to_id(0)], 10);
        assert_eq!(x[x.index_to_id(1)], 20);
        assert_eq!(x[x.index_to_id(2)], 30);

        assert_eq!(x.into_values().collect::<Vec<_>>(), vec![10, 20, 30]);
    }


    #[test]
    fn clear_check()
    {
        let mut v = GenVec::new();
        let a = v.insert(42);

        assert_eq!(v.get(a), Some(&42));
        v.remove_all();
        assert_eq!(v.get(a), None);
    }

    #[test]
    fn check_generation()
    {
        let mut v = GenVec::new();
        let a = v.insert(42);
        assert_eq!(v.get(a), Some(&42));
        assert_eq!(v.remove(a), Some(42));
        let b = v.insert(50);
        assert_eq!(v.get(b), Some(&50));
        assert_eq!(v.get(a), None);
        assert_ne!(a, b);
    }


    #[test]
    fn saturation()
    {
        let mut v = GenVecOf::<i32, u8>::new();

        assert_eq!(v.len(), 0);

        for i in 0..300
        {
            let a = v.insert(i);
            v.remove(a);
        }

        assert_eq!(v.len(), 0);
        //dbg!(v);
    }

    #[test]
    fn wrapping()
    {
        let mut v = GenVecOf::<i32, Wrapping<u8>>::new();

        assert_eq!(v.len(), 0);

        let first_key = v.insert(1000);
        v.remove(first_key);

        let second_key = v.insert(2000);
        v.remove(second_key);

        for i in 0..254
        {
            let a = v.insert(i);
            v.remove(a);
        }

        let first_key_wrapped = v.insert(3000);

        assert_eq!(v.len(), 1);
        assert_eq!(first_key_wrapped, first_key);
        assert_ne!(second_key, first_key);

        // dbg!(first_key_wrapped);
        // dbg!(first_key);
        debug_assert_eq!(v.get(first_key_wrapped), Some(&3000));
        // the key was wrapped
        debug_assert_eq!(v.get(first_key), Some(&3000));

        debug_assert_eq!(v.get(second_key), None);
    }

    #[test]
    fn showcase()
    {
        let mut entities = GenVec::new();
        let enemy = entities.insert("zoombie");

        assert_eq!(enemy.get(&entities), Some(&"zoombie"));
        assert_eq!(entities[enemy], "zoombie");
        assert!(entities.get(enemy).is_some());

        entities.remove(enemy); // the key is no longer valid
        assert!(entities.get(enemy).is_none()); // the value don't exist

        entities.insert("slime");
        entities.insert("skeleton");

        for (id, entity) in entities
        {
            println!("{:?} => {}", id, entity)
        }
    }


    fn wrapping_about_to_wrap() -> GenVecOf::<i32, Wrapping<u8>>
    {
        let mut v = GenVecOf::<i32, Wrapping<u8>>::new();

        for i in 0..255
        {
            let a = v.insert(i);
            v.remove(a);
        }

        //dbg!(v);
        v
    }

    fn non_wrapping_about_to_wrap() -> GenVecOf::<i32, u8>
    {
        let mut v = GenVecOf::<i32, u8>::new();

        for i in 0..255
        {
            let a = v.insert(i);
            v.remove(a);
        }

        //dbg!(v);
        v
    }

    #[test]
    fn rollback_remove_empty()
    {
        let mut genvec = GenVec::new();
        // dbg!(&genvec);

        let id = genvec.insert(42);

        let old_gen = genvec.clone();

        // dbg!(&genvec);
        let removed = genvec.remove_from_index(id.index()).unwrap();
        // dbg!(&genvec);
        genvec.rollback_remove_index(id.index(), removed).unwrap();
        // dbg!(&genvec);

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_remove_wrapping_empty()
    {
        let mut genvec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let id = genvec.insert(42);

        let old_gen = genvec.clone();

        let removed = genvec.remove_from_index(id.index()).unwrap();
        genvec.rollback_remove_index(id.index(), removed).unwrap();

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_remove_wrapping()
    {
        let mut genvec = wrapping_about_to_wrap();
        let id = genvec.insert(42);

        let old_gen = genvec.clone();

        let removed = genvec.remove_from_index(id.index()).unwrap();
        genvec.rollback_remove_index(id.index(), removed).unwrap();

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_remove_wrapping_2()
    {
        let mut genvec = wrapping_about_to_wrap();
        genvec.insert(50);

        let id = genvec.insert(42);

        let old_gen = genvec.clone();

        let removed = genvec.remove_from_index(id.index()).unwrap();
        genvec.rollback_remove_index(id.index(), removed).unwrap();

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_remove_non_wrapping()
    {
        let mut genvec = non_wrapping_about_to_wrap();
        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);

        let old_gen = genvec.clone();

        let removed = genvec.remove_from_index(id.index()).unwrap();
        // dbg!(&genvec);

        genvec.rollback_remove_index(id.index(), removed).unwrap();
        // dbg!(&genvec);

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_remove_non_wrapping_2()
    {
        let mut genvec = non_wrapping_about_to_wrap();
        genvec.insert(50);

        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);

        let old_gen = genvec.clone();

        let removed = genvec.remove_from_index(id.index()).unwrap();
        // dbg!(&genvec);

        genvec.rollback_remove_index(id.index(), removed).unwrap();
        // dbg!(&genvec);

        assert_eq!(genvec, old_gen);
    }


    // rollback_insert

    #[test]
    fn rollback_insert_empty()
    {
        let mut genvec = GenVec::new();
        let old_gen = genvec.clone();

        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();

        assert_eq!(genvec, old_gen);
    }


    #[test]
    fn rollback_insert_wrapping_empty()
    {
        // We can't know if the gen vec is new or if the gen vec just wrapped

        let mut genvec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let old_gen = genvec.clone();

         dbg!(&genvec);
        let id = genvec.insert(42);
         dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();
         dbg!(&genvec);

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_insert_wrapping_3()
    {
        // We can't know if the gen vec is new or if the gen vec just wrapped

        let mut genvec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let _id = genvec.insert(45);

        let old_gen = genvec.clone();

         dbg!(&genvec);
        let id = genvec.insert(42);
         dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();
         dbg!(&genvec);

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_insert_wrapping_dif()
    {
        let mut genvec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let id = genvec.insert(45);
        genvec.remove(id);
        assert_ne!(genvec, GenVecOf::<i32,Wrapping<Generation>>::new());
    }

    #[test]
    fn rollback_insert_wrapping_4()
    {
        let mut genvec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let _ = genvec.insert(45);

        //dbg!(&genvec);
        let id = genvec.insert(42);
        //dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();
        //dbg!(&genvec);

         let mut old_gen = GenVecOf::new();
         old_gen.insert(50);

        assert_ne!(genvec, old_gen);
    }

    #[test]
    fn rollback_insert_wrapping()
    {
        let mut genvec = wrapping_about_to_wrap();
        let old_gen = genvec.clone();

        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();
        // dbg!(&genvec);

        assert_eq!(genvec, old_gen);
    }


    #[test]
    fn rollback_insert_wrapping_2()
    {
        let mut genvec = wrapping_about_to_wrap();
        let old_gen = genvec.clone();

        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_insert_non_wrapping()
    {
        let mut genvec = non_wrapping_about_to_wrap();
        let old_gen = genvec.clone();

        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();

        assert_eq!(genvec, old_gen);
    }

    #[test]
    fn rollback_insert_non_wrapping_2()
    {
        let mut genvec = non_wrapping_about_to_wrap();
        let old_gen = genvec.clone();

        // dbg!(&genvec);
        let id = genvec.insert(42);
        // dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();

        assert_eq!(genvec, old_gen);
    }


    #[test]
    fn retain_test() {
        let mut g = GenVec::from_iter([1,2,3,4,5,6,7,8]);
        assert_eq!(g.len(), 8);

        g.retain(|_id,x| x % 2  == 0);
        assert_eq!(g.len(), 4);

        assert!(g.into_values().eq([2,4,6,8]));
    }

}