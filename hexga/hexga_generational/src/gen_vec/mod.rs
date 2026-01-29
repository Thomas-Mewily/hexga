use super::*;


pub mod prelude
{
    pub use super::{GenVec, GenView, GenViewMut, CollectToGenVec};
}

#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
use serde_impl::*;


pub type GenVec<T> = GenVecOf<T,Generation,Vec<Entry<T,Generation>>>;
pub type GenView<'a,T> = GenVecOf<T,Generation,&'a [Entry<T,Generation>]>;
pub type GenViewMut<'a,T> = GenVecOf<T,Generation,&'a mut [Entry<T,Generation>]>;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryValue<T>
{
    Occupied(T),
    /// Next free entry.
    ///
    /// All Vacant entries form a linked list, where the last one point to usize::MAX.
    Vacant(usize),
}


impl<T> EntryValue<T>
{
    pub fn get(&self) -> Option<&T> { if let Self::Occupied(v) = self { Some(v) } else { None }}
    pub fn get_mut(&mut self) -> Option<&mut T> { if let Self::Occupied(v) = self { Some(v) } else { None }}

    /// Panic is the entry is free
    pub(crate) fn take_and_set_vacant_unchecked(&mut self, free_head: usize) -> T {
        match std::mem::replace(self, EntryValue::Vacant(free_head)) {
            EntryValue::Occupied(value) => value,
            EntryValue::Vacant(_) => panic!("Entry was already free"),
        }
    }

    pub fn is_vacant(&self) -> bool { matches!(self, Self::Vacant(_))}
    pub fn is_occupied(&self) -> bool { matches!(self, Self::Occupied(_))}
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Entry<T,Gen=Generation> where Gen:IGeneration
{
    pub(crate) value: EntryValue<T>,
    #[cfg_attr(feature = "serde", serde(rename = "gen"))]
    generation: Gen,
}

impl<T, Gen> std::fmt::Debug for Entry<T, Gen>
where
    T: std::fmt::Debug,
    Gen: IGeneration + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("");
        match self.value() {
            Some(value) => tuple.field(value),
            None => tuple.field(&"_"),
        };
        tuple.field(&gen_id::GenerationDebug(self.generation)).finish()
    }
}




impl<T,Gen> From<(EntryValue<T>, Gen)> for Entry<T,Gen> where Gen:IGeneration
{
    fn from((value, generation): (EntryValue<T>, Gen)) -> Self {
        Self::new(value, generation)
    }
}
impl<T,Gen> From<Entry<T,Gen>> for (EntryValue<T>, Gen) where Gen:IGeneration
{
    fn from(entry: Entry<T,Gen>) -> Self {
        (entry.value, entry.generation)
    }
}

impl<T,Gen> Entry<T,Gen> where Gen:IGeneration
{
    pub fn new(value: EntryValue<T>, generation: Gen) -> Self { Self { value, generation }}
    pub fn generation(&self) -> Gen { self.generation }

    pub fn have_value(&self) -> bool { self.value().is_some() }

    pub fn value(&self) -> Option<&T> { self.value.get() }
    pub fn value_mut(&mut self) -> Option<&mut T> { self.value.get_mut() }

    pub fn get_id(&self, index: usize) -> GenIDOf<Gen> { GenIDOf::from_index_and_generation(index, self.generation) }

    pub fn increment_generation(&mut self) -> bool { if self.can_increment_generation() { self.generation.increment(); true } else { false } }
    pub fn can_increment_generation(&self) -> bool { self.generation.can_increment() }

    pub fn decrement_generation(&mut self) -> bool { if self.can_decrement_generation() { self.generation.decrement(); true } else { false } }
    pub fn can_decrement_generation(&self) -> bool { self.generation.can_decrement() }

    pub fn is_generation_saturated(&self) -> bool { !self.can_increment_generation() }
}

pub struct GenVecOf<T,Gen=Generation,C=Vec<Entry<T,Gen>>>
    where
    //C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    Gen:IGeneration
{
    pub(crate) values: C,
    /// The first index of the slot that is None.
    /// usize::MAX if all slot are used
    free: usize,
    len: usize,
    phantom: PhantomData<(T,Gen)>,
}

impl<T, C, Gen> Debug for GenVecOf<T, Gen, C>
where
    C: for<'a> View<'a, View = &'a [Entry<T, Gen>]>,
    Gen: IGeneration,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.entries())
            .finish()
    }
}
impl<T, C, Gen> Clone for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    C: Clone
{
    fn clone(&self) -> Self {
        Self { values: self.values.clone(), free: self.free.clone(), len: self.len.clone(), phantom: self.phantom.clone() }
    }
}
impl<T, C, Gen> Copy for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    C: Copy
{
}
impl<T, C, Gen> Eq for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    T: PartialEq
{
}

impl<T, C, Gen> Hash for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.len.hash(state);

        if !Gen::OVERFLOW_BEHAVIOR.is_wrapping()
        {
            self.values.as_view().hash(state);
            self.free.hash(state);
        }else
        {
            for (id, value) in self.iter()
            {
                id.generation().hash(state);
                value.hash(state);
            }
        }
    }
}

impl<T, C, C2, Gen> PartialEq<GenVecOf<T,Gen,C2>> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C2: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    Gen:IGeneration,
    T: PartialEq
{
    fn eq(&self, other: &GenVecOf<T,Gen,C2>) -> bool
    {
        let view = self.values.as_view();
        let other_view = self.values.as_view();

        if !Gen::OVERFLOW_BEHAVIOR.is_wrapping()
        {
            self.len == other.len && view == other_view && self.free == other.free
        }else
        {
            /*
                We can't know if the gen vec is new or if the gen vec just wrapped arround.

                Those two are equal: (Assuming Gen::MIN value is 0)
                A: GenVecOf { entry: [Entry { value: Next(18446744073709551615), generation: 0 }], free: 0, len: 0 }
                B: GenVecOf { entry: [], free: 18446744073709551615, len: 0 }

                Both can represent unused wrapped gen vec.

                doing :

                let id = B.insert(10);
                B.rollback_insert(id);

                will put A in the same equal state/representation as B.


                But these 2 are different, because that generation was already used.

                X: GenVecOf { entry: [Entry { value: Next(18446744073709551615), generation: 1 }], free: 0, len: 0 }
                Y: GenVecOf { entry: [], free: 18446744073709551615, len: 0 }
            */

            if self.len != other.len { return false; }
            if self.free == other.free { return view == other_view; }
            if !(self.free.is_max() ^ other.free.is_max()) { return false; }

            if self.free.is_max()
            {
                if view.len() + 1 != other_view.len() { return false; }
                let mid = other.free;
                debug_assert!(!mid.is_max());

                let entry = other.get_entry_from_index(mid).unwrap();
                let EntryValue::Vacant(f) = entry.value else { return false; };
                if !f.is_max() || !entry.generation().is_min() { return false; }

                let self_left = &view[0..mid];
                let self_right = &view[mid..];

                let other_left = &other_view[0..mid];
                let other_right = &other_view[mid+1..];

                self_left == other_left && self_right == other_right
            }else if other.free.is_max()
            {
                if other_view.len() + 1 != view.len() { return false; }
                let mid = self.free;
                debug_assert!(!mid.is_max());

                let entry = self.get_entry_from_index(mid).unwrap();
                let EntryValue::Vacant(f) = entry.value else { return false; };
                if !f.is_max() || !entry.generation().is_min() { return false; }

                let other_left = &other_view[0..mid];
                let other_right = &other_view[mid..];

                let self_left = &view[0..mid];
                let self_right = &view[mid+1..];

                other_left == self_left && other_right == self_right
            }else
            {
                unreachable!()
            }
        }
    }
}


impl<T, C, Gen> GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
{
    pub fn try_from_raw_parts(values: C, free: usize) -> Result<Self, String>
    {
        let view = values.as_view();
        let len = view.iter().filter(|s| s.have_value()).count();

        if view.len() == usize::MAX
        {
            return Err("GenVec: the last usize value is used for null in a GenVec and cannot be used".to_owned());
        }

        let nb_use = len;
        let mut nb_free = view.len() - nb_use;
        let mut cur_free = free;

        if nb_free != 0
        {
            loop
            {
                let Some(next_entry) = view.get(cur_free) else { return Err(format!("GenVec: entry {:?} is out of range", cur_free)); };
                let EntryValue::Vacant(f) = next_entry.value else { return Err(format!("GenVec: entry {:?} was not free", cur_free)); };

                // This is super important to check if there is no cycle in the free list.
                // Any invalid EntryValue::Free(index) can lead to crash

                if f == usize::MAX
                {
                    if nb_free == 1 // last free index, should point to nothings/usize::MAX
                    {
                        break;
                    }
                    // not the last free index, should point to the next one
                    return Err(format!("GenVec: invalid free head {:?} at {:?}", f, cur_free));
                }
                cur_free = f;
                nb_free -= 1;

                if nb_free == 0
                {
                    return Err(format!("GenVec: last value at index {cur_free} should point to nothings and not {f}"));
                }
            }
        }else
        {
            if free.is_not_max()
            {
                return Err(format!("GenVec: invalid next {free} in a fully used genvec")); // should be max value
            }
        }


        Ok(unsafe { Self::from_raw_parts_unchecked(values, free, len) })
    }

    pub unsafe fn from_raw_parts_unchecked(values: C, free: usize, len: usize) -> Self
    {
        Self { values, free, len, phantom: PhantomData }
    }
}



impl<T,Gen,C> Default for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration, C: Default
{
    fn default() -> Self { Self::new() }
}

impl<T,Gen,C> GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
{
    pub fn new() -> Self where C: Default { Self { values: C::default(), free: usize::MAX, len: 0, phantom: PhantomData }}
    /*
    pub fn with_capacity(capacity: usize) -> Self { Self { values: Vec::with_capacity(capacity), free: usize::MAX, len: 0 }}

    pub fn capacity(&self) -> usize { self.values.capacity() }
    pub fn shrink_to_fit(mut self) { self.values.shrink_to_fit(); }
    */

    /// Clears the [`GenVec`], removing all elements and resetting all [`GenID`] values.
    ///
    /// After calling this method, any previous [`GenID`] is no longer valid (not enforced) and
    /// **must** not be used, as doing so may lead to undefined behavior.
    pub fn clear(&mut self) where C: Clear
    {
        self.free = usize::MAX;
        self.len = 0;
        self.values.clear();
    }

    /// Removes all elements from the [`GenVec`] and invalidates all existing [`GenID`] (enforced).
    pub fn remove_all(&mut self) where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    {
        for (index, v) in self.values.as_mut_view().iter_mut().enumerate()
        {
            if v.have_value()
            {
                if v.increment_generation()
                {
                    v.value = EntryValue::Vacant(self.free);
                    self.free = index;
                }else
                {
                    v.value = EntryValue::Vacant(usize::MAX);
                }
            }
        }
        self.len = 0;
    }

    pub fn rollback_insert(&mut self, id: GenIDOf<Gen>) -> Result<T,()>
        where
        C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
        C: Pop<Entry<T,Gen>>
    {
        let index = id.index();
        let free = self.free;

        let entry_len = self.values.as_view().len();

        let Some(entry) = self.get_entry_mut_from_index(index) else { return Err(()); };
        if entry.value.is_vacant() { return Err(()); }

        if free.is_max()
        {
            if index + 1 != entry_len { return Err(()); }
        }

        let can_not_decrease = !entry.can_decrement_generation();
        let val = entry.value.take_and_set_vacant_unchecked(free);
        self.len -= 1;

        if free.is_max() && can_not_decrease
        {
            self.values.pop().ok_or(())?;
        }else
        {
            self.free = index;
        }

        Ok(val)
    }

    pub fn insert_cyclic<F>(&mut self, init: F) -> GenIDOf<Gen>
        where
        F: FnOnce(GenIDOf<Gen>) -> T,
        C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
        C: Push<Entry<T,Gen>>
    {
        self.len += 1;

        if self.free == usize::MAX
        {
            let index = self.values.as_view().len();

            // The last index is used for the null() key
            assert!(index != usize::MAX, "How you didn't run out of memory before ?"); // ZST ?

            let generation = Gen::MIN;
            let id = GenIDOf::from_index_and_generation(index, generation);
            self.values.push(Entry { value: EntryValue::Occupied(init(id)), generation });
            return id;
        }

        let values = self.values.as_mut_view();

        let EntryValue::Vacant(next_free_index) = values[self.free].value else { unreachable!(); };
        let free = self.free;
        self.free = next_free_index;
        let id = GenIDOf::from_index_and_generation(free, values[free].generation);
        values[free].value = EntryValue::Occupied(init(id));
        return id;
    }

    #[inline(always)]
    pub fn insert(&mut self, value: T) ->  GenIDOf<Gen>
        where
        C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
        C: Push<Entry<T,Gen>>
    {
        self.insert_cyclic(|_| value)
    }

    #[inline(always)]
    pub fn get_entry_from_index(&self, index: usize) -> Option<&Entry<T,Gen>> { self.values.as_view().get(index) }
    // Do not expose, having a &mut Entry<T,Gen> allow to mutate the Occupied/Vacant state of the entry, breaking invariant
    #[inline(always)]
    pub(crate) fn get_entry_mut_from_index(&mut self, index: usize) -> Option<&mut Entry<T,Gen>>
        where
        C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    { self.values.as_mut_view().get_mut(index) }

    #[inline(always)]
    pub fn get_from_index(&self, index: usize) -> Option<&T> { self.get_entry_from_index(index).and_then(|s| s.value()) }
    #[inline(always)]
    pub fn get_mut_from_index(&mut self, index: usize) -> Option<&mut T>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    { self.get_entry_mut_from_index(index).and_then(|s| s.value_mut()) }

    #[inline(always)]
    pub fn get_entry(&self, id: GenIDOf<Gen>) -> Option<&Entry<T,Gen>>
    { self.get_entry_from_index(id.index()).filter(|v| v.generation() == id.generation()) }
    // Do not expose, having a &mut Entry<T,Gen> allow to mutate the Occupied/Vacant state of the entry, breaking invariant
    #[inline(always)]
    pub(crate) fn get_entry_mut(&mut self, id: GenIDOf<Gen>) -> Option<&mut Entry<T,Gen>>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    { self.get_entry_mut_from_index(id.index()).filter(|v| v.generation() == id.generation()) }

    #[inline(always)]
    pub fn get(&self, id: GenIDOf<Gen>) -> Option<&T>
    { self.get_entry(id).and_then(|v| v.value()) }
    #[inline(always)]
    pub fn get_mut(&mut self, id: GenIDOf<Gen>) -> Option<&mut T>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    { self.get_entry_mut(id).and_then(|v| v.value_mut()) }

    /// Return a valid [`GenID`] to the current index or return [`GenIDOf::NULL`] if the index is outside the range
    pub fn index_to_id(&self, index: usize) -> GenIDOf<Gen>
    { self.get_entry_from_index(index).map(|v| v.get_id(index)).unwrap_or(GenIDOf::NULL) }

    /// The operation that once done just after an [`Self::remove_from_index`], put this data structure in the same state as before
    pub fn rollback_remove_index(&mut self, index: usize, value: T) -> Result<(), ()>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    {
        let mut head = self.free;
        let entry = self.get_entry_mut_from_index(index).ok_or(())?;
        let EntryValue::Vacant(f) = entry.value else { return Err(()); };
        let free = f;

        if f.is_not_max()
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

        entry.value = EntryValue::Occupied(value);

        self.free = head;
        self.len += 1;

        Ok(())
    }

    pub fn remove_from_index(&mut self, index: usize) -> Option<T>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    {
        let head = self.free;

        let Some(entry) = self.get_entry_mut_from_index(index) else { return None; };
        if entry.value.is_vacant() { return None; }

        let val = entry.value.take_and_set_vacant_unchecked(head);

        if entry.increment_generation()
        {
            self.free = index;
        }else
        {
            entry.value = EntryValue::Vacant(usize::MAX);
        }
        self.len -= 1;

        Some(val)
    }

    pub fn rollback_remove(&mut self, id: GenIDOf<Gen>, value: T) -> Result<(), ()>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    {
        // TODO: missing some check to see if the last operation removal was done with id
        self.rollback_remove_index(id.index(), value)
    }

    pub fn remove(&mut self, id: GenIDOf<Gen>) -> Option<T>
        where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    {
        if self.get(id).is_none() { return None; }
        self.remove_from_index(id.index())
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry<T,Gen>> + DoubleEndedIterator + FusedIterator { self.values.as_view().iter() }

    pub const fn len(&self) -> usize { self.len }

    pub fn iter(&self) -> Iter<'_, T, Gen> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_, T, Gen> where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]> { self.into_iter() }

    pub fn ids(&self) -> impl Iterator<Item = GenIDOf<Gen>> { self.into_iter().map(|(id, _val)| id) }

    pub fn values(&self) -> impl Iterator<Item = &T> { self.iter().map(|(_,val)| val) }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> where C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]> { self.iter_mut().map(|(_,val)| val) }

    pub fn into_ids(self) -> impl Iterator<Item = GenIDOf<Gen>> where C: IntoIterator<Item=Entry<T,Gen>>
    {
        self.into_iter().map(|(id, _val)| id)
    }
    pub fn into_values(self) -> impl Iterator<Item = T> where C: IntoIterator<Item=Entry<T,Gen>>
    {
        self.into_iter().map(|(_id, val)| val)
    }

    /// Iter over all entry index, including the free/unused one.
    ///
    /// The correct way to iterate over all entry index.
    ///
    /// Use this instead of `0..genvec.len()` (wrong).
    pub fn iter_index(&self) -> impl Iterator<Item = usize> + 'static { 0..self.values.as_view().len() }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(id, v)` for which `f(id, &v)` returns `false`. The elements are visited in unsorted (and unspecified) order.
    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(GenIDOf<Gen>, &T) -> bool, C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
    {
        self.retain_mut(|id,elem| f(id,elem));
    }

        /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(id, v)` for which `f(id, &mut v)` returns `false`. The elements are visited in unsorted (and unspecified) order.
    pub fn retain_mut<F>(&mut self, mut f: F)
        where
        F: FnMut(GenIDOf<Gen>, &mut T) -> bool,
        C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>
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

    pub fn to_owned(&self) -> GenVecOf<T, Gen, C::Owned> where C: ToOwned
    {
        GenVecOf
        {
            values: self.values.to_owned(),
            free: self.free,
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<T, C, Gen> Index<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration
{
    type Output=T;
    fn index(&self, index: GenIDOf<Gen>) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, C, Gen> IndexMut<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>, Gen:IGeneration
{
    fn index_mut(&mut self, index: GenIDOf<Gen>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, C, Gen> Index<usize> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_from_index(index).unwrap() }
}
impl<T, C, Gen> IndexMut<usize> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>, Gen:IGeneration
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut_from_index(index).unwrap() }
}

impl<T, C, Gen> FromIterator<T> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    Gen:IGeneration,
    C: FromIterator<Entry<T,Gen>>
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let values : C = iter.into_iter().map(|v| Entry::new(EntryValue::Occupied(v), Gen::MIN)).collect();
        let len = values.as_view().len();
        Self { values, free: usize::MAX, len, phantom: PhantomData }
    }
}

impl<T, C, Gen> IntoIterator for GenVecOf<T,Gen,C>
    where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    Gen:IGeneration,
    C: IntoIterator<Item = Entry<T,Gen>>
{
    type Item = (GenIDOf<Gen>, T);
    type IntoIter = IntoIter<T, <C as IntoIterator>::IntoIter, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter
        {
            iter: self.values.into_iter().enumerate(),
            len_remaining: self.len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter<T,I,Gen>
    where I: Iterator<Item = Entry<T,Gen>>, Gen:IGeneration
{
    iter: std::iter::Enumerate<I>,
    len_remaining: usize,
}


impl<T, I, Gen> Iterator for IntoIter<T, I, Gen>
    where I: Iterator<Item = Entry<T,Gen>>, Gen:IGeneration
{
    type Item = (GenIDOf<Gen>, T);

    fn next(&mut self) -> Option<Self::Item>
    {
        while let Some((index, entry)) = self.iter.next()
        {
            if let EntryValue::Occupied(value) = entry.value
            {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, entry.generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<T, I, Gen> DoubleEndedIterator for IntoIter<T, I, Gen>
    where I: Iterator<Item = Entry<T,Gen>> + DoubleEndedIterator + ExactSizeIterator, Gen:IGeneration
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.iter.next_back()
        {
            if let EntryValue::Occupied(value) = entry.value
            {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, entry.generation), value));
            }
        }
        None
    }
}

// Kinda unsafe:
impl<T, I, Gen> FusedIterator for IntoIter<T, I, Gen> where I: Iterator<Item = Entry<T,Gen>>, Gen:IGeneration {}
// Kinda unsafe:
impl<T, I, Gen> ExactSizeIterator for IntoIter<T, I, Gen> where I: Iterator<Item = Entry<T,Gen>>, Gen:IGeneration { fn len(&self) -> usize { self.len_remaining } }


impl<'b, T, C, Gen> IntoIterator for &'b GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    Gen:IGeneration
{
    type Item = (GenIDOf<Gen>, &'b T);
    type IntoIter = Iter<'b, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        Iter
        {
            iter: self.values.as_view().iter().enumerate(),
            len_remaining: self.len,
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, T, Gen: IGeneration=Generation>
{
    iter: std::iter::Enumerate<std::slice::Iter<'a, Entry<T, Gen>>>,
    len_remaining: usize,
}
impl<'a, T, Gen: IGeneration> Clone for Iter<'a, T, Gen>
{
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone(), len_remaining: self.len_remaining.clone() }
    }
}


impl<'a, T, Gen: IGeneration> Iterator for Iter<'a, T, Gen> {
    type Item = (GenIDOf<Gen>, &'a T);

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
impl<'a, T, Gen: IGeneration> DoubleEndedIterator for Iter<'a, T, Gen> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.iter.next_back() {
            if let Some(value) = entry.value() {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, entry.generation), value));
            }
        }
        None
    }
}
impl<'a, T, Gen: IGeneration> FusedIterator for Iter<'a, T, Gen> {}
impl<'a, T, Gen: IGeneration> ExactSizeIterator for Iter<'a, T, Gen> { fn len(&self) -> usize { self.len_remaining } }



impl<'a, T, C, Gen> IntoIterator for &'a mut GenVecOf<T,Gen,C>
    where
    C: for<'b> View<'b,View = &'b [Entry<T,Gen>]>,
    C: for<'b> ViewMut<'b,ViewMut = &'b mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    type Item = (GenIDOf<Gen>, &'a mut T);
    type IntoIter = IterMut<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut
        {
            iter: self.values.as_mut_view().iter_mut().enumerate(),
            len_remaining: self.len,
        }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T, Gen: IGeneration=Generation>
{
    iter: std::iter::Enumerate<std::slice::IterMut<'a, Entry<T, Gen>>>,
    len_remaining: usize,
}

impl<'a, T, Gen: IGeneration> Iterator for IterMut<'a, T, Gen>
{
    type Item = (GenIDOf<Gen>, &'a mut T);

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
impl<'a, T, Gen: IGeneration> DoubleEndedIterator for IterMut<'a, T, Gen>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.iter.next_back() {
            let generation = entry.generation();
            if let Some(value) = entry.value_mut() {
                self.len_remaining -= 1;
                return Some((GenIDOf::from_index_and_generation(index, generation), value));
            }
        }
        None
    }
}
impl<'a, T, Gen: IGeneration> FusedIterator for IterMut<'a, T, Gen> {}
impl<'a, T, Gen: IGeneration> ExactSizeIterator for IterMut<'a, T, Gen> { fn len(&self) -> usize { self.len_remaining } }


impl<T,Gen,C> Collection for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration {}
impl<T,Gen,C> CollectionBijective for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration {}
impl<T,Gen,C> Length for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration { #[inline(always)] fn len(&self) -> usize { self.len() } }
impl<T,Gen,C> Clear for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration, C: Clear { #[inline(always)] fn clear(&mut self) { self.clear(); } }

impl<T,Gen,C> Push<T> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration, C: Push<Entry<T,Gen>>
{
    type Output=GenIDOf<Gen>;
    fn push(&mut self, item: T) -> Self::Output {
        self.insert(item)
    }
}

impl<'s, T, Gen, C> From<&'s GenVecOf<T, Gen, C>> for GenVecOf<T, Gen, &'s [Entry<T, Gen>]>
    where
    C: View<'s, View = &'s [Entry<T, Gen>]>,
    Gen: IGeneration,
    T:'s
{
    fn from(value: &'s GenVecOf<T, Gen, C>) -> Self {
        value.as_view()
    }
}

impl<'s, T, Gen, C> From<&'s mut GenVecOf<T, Gen, C>> for GenVecOf<T, Gen, &'s mut [Entry<T, Gen>]>
    where
    C: View<'s, View = &'s [Entry<T, Gen>]>,
    C: ViewMut<'s, ViewMut = &'s mut [Entry<T, Gen>]>,
    Gen: IGeneration,
    T:'s
{
    fn from(value: &'s mut GenVecOf<T, Gen, C>) -> Self {
        value.as_mut_view()
    }
}

impl<'s, T, Gen, C> View<'s> for GenVecOf<T, Gen, C>
where
    C: View<'s, View = &'s [Entry<T, Gen>]>,
    Gen: IGeneration,
    T:'s
{
    type View = GenVecOf<T, Gen, &'s [Entry<T, Gen>]>;

    fn as_view(&'s self) -> Self::View {
        GenVecOf {
            values: self.values.as_view(),
            free: self.free,
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<'s, T, Gen, C> ViewMut<'s> for GenVecOf<T, Gen, C>
where
    C: View<'s, View = &'s [Entry<T, Gen>]>,
    C: ViewMut<'s, ViewMut = &'s mut [Entry<T, Gen>]>,
    Gen: IGeneration,
    T:'s
{
    type ViewMut = GenVecOf<T, Gen, &'s mut [Entry<T, Gen>]>;

    fn as_mut_view(&'s mut self) -> Self::ViewMut {
        GenVecOf {
            values: self.values.as_mut_view(),
            free: self.free,
            len: self.len,
            phantom: PhantomData,
        }
    }
}


impl<T,Gen,C> Shrink for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration, C: Shrink
{
    fn shrink_to_fit(&mut self) { self.values.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.values.shrink_to(min_capacity); }
}
impl<T,Gen,C> Truncate for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration, C: Truncate
{
    fn truncate(&mut self, len: usize) { self.values.truncate(len); }
}


impl<T,Gen,C> Capacity for GenVecOf<T,Gen,C> where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration, C: Capacity
{
    type Param=();

    #[inline(always)]
    fn capacity(&self) -> usize { self.values.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.values.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.values.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.values.try_reserve_exact(additional) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenVecError<T,Gen> where Gen:IGeneration
{
    IndexOutOfRange(IndexOutOfBounds),
    WrongGeneration(GenVecWrongGeneration<T,Gen>),
    /// The entry at this index is saturated
    Saturated(usize),
}
impl<T,Gen> Eq for GenVecError<T,Gen> where Gen:IGeneration {}
impl<T,Gen> PartialEq for GenVecError<T,Gen> where Gen:IGeneration
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IndexOutOfRange(l0), Self::IndexOutOfRange(r0)) => l0 == r0,
            (Self::WrongGeneration(l0), Self::WrongGeneration(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<T,Gen> Hash for GenVecError<T,Gen> where Gen:IGeneration
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

// error: the `Copy` impl for `hexga_core::collections::IndexOutOfRange` requires that `std::ops::Range<usize>: Copy`
// impl<T,Gen,C> Copy for GenVecError<T,Gen> {}
impl<T,Gen> Clone for GenVecError<T,Gen> where Gen:IGeneration
{
    fn clone(&self) -> Self {
        match self {
            GenVecError::IndexOutOfRange(v) => Self::IndexOutOfRange(v.clone()),
            GenVecError::WrongGeneration(v) => Self::WrongGeneration(v.clone()),
            GenVecError::Saturated(v) => Self::Saturated(v.clone()),
        }
    }
}

impl<T,Gen> Debug for GenVecError<T,Gen> where Gen:IGeneration
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenVecError::IndexOutOfRange(arg0) => f.debug_tuple("IndexOutOfRange").field(arg0).finish(),
            GenVecError::WrongGeneration(arg0) => f.debug_tuple("WrongGeneration").field(arg0).finish(),
            GenVecError::Saturated(arg0) => f.debug_tuple("Saturated").field(arg0).finish(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenVecWrongGeneration<T,Gen>
    where Gen:IGeneration
{
    pub got: Gen,
    pub expected: Gen,
    #[cfg_attr(feature = "serde", serde(skip))]
    phantom: PhantomData<T>,
}


impl<T,Gen> GenVecWrongGeneration<T,Gen> where Gen:IGeneration
{
    pub fn new(got: Gen, expected: Gen) -> Self { Self{ got, expected, phantom: PhantomData }}
}
impl<T,Gen> Eq for GenVecWrongGeneration<T,Gen> where Gen:IGeneration {}
impl<T,Gen> PartialEq for GenVecWrongGeneration<T,Gen> where Gen:IGeneration
{
    fn eq(&self, other: &Self) -> bool { self.got == other.got && self.expected == other.expected }
}

impl<T,Gen> Hash for GenVecWrongGeneration<T,Gen> where Gen:IGeneration
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.got.hash(state);
        self.expected.hash(state);
    }
}


impl<T,Gen> Copy for GenVecWrongGeneration<T,Gen> where Gen:IGeneration {}
impl<T,Gen> Clone for GenVecWrongGeneration<T,Gen> where Gen:IGeneration
{
    fn clone(&self) -> Self {
        Self { got: self.got.clone(), expected: self.expected.clone(), phantom: PhantomData }
    }
}

impl<T,Gen> Debug for GenVecWrongGeneration<T,Gen> where Gen:IGeneration
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenVecWrongGeneration").field("got", &self.got).field("expected", &self.expected).field("phantom", &self.phantom).finish()
    }
}



impl<T,Gen,C> Get<usize> for GenVecOf<T,Gen,C>
    where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn get(&self, index: usize) -> Option<&Self::Output> { self.get_from_index(index) }
}
impl<T,Gen,C> TryGet<usize> for GenVecOf<T,Gen,C>
    where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration
{
    type Error=IndexOutOfBounds;
    fn try_get(&self, index: usize) -> Result<&Self::Output, Self::Error>
    {
        self.get_from_index(index).ok_or_else(|| IndexOutOfBounds::new(index, 0..self.len()))
    }
}

impl<T,Gen,C> Get<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration
{
    type Output = <Self as Index<GenIDOf<Gen>>>::Output;
    #[inline(always)]
    fn get(&self, index: GenIDOf<Gen>) -> Option<&Self::Output> { self.get(index) }
}
impl<T,Gen,C> TryGet<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration
{
    type Error=GenVecError<T,Gen>;
    fn try_get(&self, id:  GenIDOf<Gen>) -> Result<&Self::Output, Self::Error>
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
            None => Err(GenVecError::IndexOutOfRange(IndexOutOfBounds::new(id.index(), 0..self.len()))),
        }
    }
}


impl<T,Gen,C> GetMut<usize> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> { self.get_mut_from_index(index) }
}
impl<T,Gen,C> TryGetMut<usize> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    fn try_get_mut(&mut self, index: usize) -> Result<&mut Self::Output, Self::Error>
    {
        let len = self.len();
        self.get_mut_from_index(index).ok_or_else(|| IndexOutOfBounds::new(index, 0..len))
    }
}


impl<T,Gen,C> GetManyMut<usize> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        // TODO: check the SlotMap crate for a better implementation of any overlapping indices
        // It is possible to it in O(n) be moving the value of the slot when iterating (thus, making the slot invalid),
        // then move back the value to the original slot
        //
        // But because N is generally small, I keep it this way
        //
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.as_mut_view().try_get_many_mut(indices).map(|entries| entries.map(|v| v.value_mut()))
        {
            Ok(values) => if values.iter().any(|v| v.is_none()) { Err(ManyMutError::IndexOutOfBounds) } else { Ok(values.map(|v| v.unwrap())) },
            Err(e) => Err(e),
        }
    }

    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut Self::Output;N]> {
        // TODO: check the SlotMap crate for a better implementation of any overlapping indices
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.as_mut_view().get_many_mut(indices).map(|entries| entries.map(|v| v.value_mut()))
        {
            Some(values) => if values.iter().any(|v| v.is_none()) { None } else { Some(values.map(|v| v.unwrap())) },
            None => None,
        }
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut Self::Output;N] {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        unsafe { self.values.as_mut_view().get_many_unchecked_mut(indices).map(|v| v.value_mut().unwrap()) }
    }
}


impl<T,Gen,C> GetMut<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    #[inline(always)]
    fn get_mut(&mut self, index: GenIDOf<Gen>) -> Option<&mut Self::Output> { self.get_mut(index) }
}
impl<T,Gen,C> TryGetMut<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    fn try_get_mut(&mut self, id:  GenIDOf<Gen>) -> Result<&mut Self::Output, Self::Error>
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
            None => Err(GenVecError::IndexOutOfRange(IndexOutOfBounds::new(id.index(), 0..len))),
        }
    }
}

impl<T,Gen,C> GetManyMut<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [GenIDOf<Gen>; N]) -> Result<[&mut Self::Output;N], ManyMutError>
    {
        // Todo: use O(N) complexity to check the overlaping
        // Check SlotMap imply that put tmp Free slot/entry in the current indices to

        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.as_mut_view().try_get_many_mut(indices.map(|id| id.index()))
        {
            Ok(values) => if values.iter().enumerate().any(|(index,v)| !v.have_value() || v.generation() != indices[index].generation())
            { Err(std::slice::GetDisjointMutError::OverlappingIndices) } else { Ok(values.map(|v| v.value_mut().unwrap())) },
            Err(e) => Err(e),
        }
    }

    fn get_many_mut<const N: usize>(&mut self, indices: [GenIDOf<Gen>; N]) -> Option<[&mut Self::Output;N]>
    {
        // Todo: use O(N) complexity to check the overlaping
        // Check SlotMap imply that put tmp Free slot/entry in the current indices to

        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.values.as_mut_view().get_many_mut(indices.map(|id| id.index()))
        {
            Some(values) =>
                if values.iter().enumerate().any(|(index,v)| !v.have_value() || v.generation() != indices[index].generation())
                { None } else { Some(values.map(|v| v.value_mut().unwrap())) },
            None => None,
        }
    }
}
impl<T,Gen,C> Remove<usize> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration,
{
    type Output=T;
    fn remove(&mut self, index: usize) -> Option<Self::Output> {
        self.remove_from_index(index)
    }
}

impl<T,Gen,C> Remove<GenIDOf<Gen>> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration,
{
    type Output=T;
    fn remove(&mut self, index: GenIDOf<Gen>) -> Option<Self::Output> {
        self.remove(index)
    }
}

impl<T,Gen,C> CollectionStableKey for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>, Gen:IGeneration,
{}



impl<T,Gen,C> GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    Gen:IGeneration
{
    /// Moves all the elements of `other` into `self`, leaving `other` empty by clearing it (don't invalidate all previous [GenID]).
    pub fn append<C2>(&mut self, other: &mut GenVecOf<T,Gen,C2>) -> impl GenIDUpdater<Gen> + 'static
        where
        T: GenIDUpdatable<Gen>,
        C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]> + Push<Entry<T,Gen>>,
        C2: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
        C2: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
        C2: Clear,
    {
        let capacity = other.len();
        let mut h = HashMap::with_capacity(capacity);

        for (index, entry) in other.values.as_mut_view().iter_mut().enumerate().filter(|(_,s)| s.have_value())
        {
            let val = entry.value.take_and_set_vacant_unchecked(usize::MAX);
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

impl<T,Gen,C> Extend<T> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration,
    C: Push<Entry<T,Gen>>
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    {
        for val in iter.into_iter()
        {
            self.insert(val);
        }
    }
}

pub trait GenIDUpdater<Gen> where Gen: IGeneration
{
    fn update(&self, dest: &mut GenIDOf<Gen>);
}
impl<Gen> GenIDUpdater<Gen> for HashMap<GenIDOf<Gen>,GenIDOf<Gen>> where Gen: IGeneration
{
    fn update(&self, dest: &mut GenIDOf<Gen>) {
        debug_assert!(dest.is_null() || self.get(&dest).is_some());
        *dest = self.get(&dest).copied().unwrap_or(GenIDOf::NULL);
    }
}
impl<C,Gen> GenIDUpdater<Gen> for GenVecOf<GenIDOf<Gen>,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<GenIDOf<Gen>,Gen>]>,
    Gen:IGeneration,
{
    fn update(&self, dest: &mut GenIDOf<Gen>) {
        debug_assert!(dest.is_null() || self.get(*dest).is_some());
        *dest = self.get(*dest).copied().unwrap_or(GenIDOf::NULL);
    }
}

pub trait GenIDUpdatable<Gen:IGeneration=Generation>: Sized
{
    fn update_id<U: GenIDUpdater<Gen>>(&mut self, updater: &U);
}
impl<Gen> GenIDUpdatable<Gen> for GenIDOf<Gen> where Gen: IGeneration
{
    fn update_id<U: GenIDUpdater<Gen>>(&mut self, updater: &U) {
        updater.update(self);
    }
}

impl<T,Gen,C> Extend<(GenIDOf<Gen>, T)> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View = &'a [Entry<T,Gen>]>,
    C: for<'a> ViewMut<'a,ViewMut = &'a mut [Entry<T,Gen>]>,
    Gen:IGeneration,
    C: Push<Entry<T,Gen>>,
    T: GenIDUpdatable<Gen>
{
    fn extend<I: IntoIterator<Item = (GenIDOf<Gen>, T)>>(&mut self, iter: I)
    {
        /*
        let mut it = iter.into_iter();
        let mut updater = GenVec::with_capacity(it.size_hint().0);

        for (old_id, val) in it
        {
            // Not technically possible to insert_at a given position with the current api (then manually change the generation)
            let new_id = self.insert(val);
            //h.insert(old_id, new_id);
        }
        */

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

pub trait CollectToGenVec<T>: Sized + IntoIterator<Item = T>
{
    fn to_genvec(self) -> GenVec<T>
    {
        let values: Vec<Entry<T>> = self.into_iter().map(|v| Entry::new(EntryValue::Occupied(v), Generation::MIN)).collect();
        let len = values.as_view().len();
        GenVec { values, free: usize::MAX, len, phantom: PhantomData }
    }
}
impl<I,T> CollectToGenVec<T> for I where I: IntoIterator<Item = T>, {}



#[allow(dead_code)]
#[cfg(test)]
mod tests
{
    use std::num::Wrapping;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Cell
    {
        next: GenID,
        value: i32,
    }

    impl GenIDUpdatable for Cell
    {
        fn update_id<U: GenIDUpdater<u32>>(&mut self, updater: &U) {
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
        let mut v = GenVecOf::<i32, u8, Vec<Entry<i32,u8>>>::new();

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
        let mut v = GenVecOf::<i32, Wrapping<u8>, Vec<Entry<i32,Wrapping<u8>>>>::new();

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


    fn wrapping_about_to_wrap() -> GenVecOf::<i32, Wrapping<u8>, Vec<Entry<i32,Wrapping<u8>>>>
    {
        let mut v = GenVecOf::<i32, Wrapping<u8>, Vec<Entry<i32,Wrapping<u8>>>>::new();

        for i in 0..255
        {
            let a = v.insert(i);
            v.remove(a);
        }

        //dbg!(v);
        v
    }

    fn non_wrapping_about_to_wrap() -> GenVecOf::<i32, u8, Vec<Entry<i32,u8>>>
    {
        let mut v = GenVecOf::<i32, u8, Vec<Entry<i32,u8>>>::new();

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
        let mut genvec = GenVecOf::<i32,Wrapping<Generation>, Vec<Entry<i32,Wrapping<Generation>>>>::new();
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

        let mut genvec = GenVecOf::<i32, Wrapping<Generation>, Vec<Entry<i32, Wrapping<Generation>>>>::new();
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

        let mut genvec = GenVecOf::<i32,Wrapping<Generation>, Vec<Entry<i32, Wrapping<Generation>>>>::new();
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
        let mut genvec = GenVecOf::<i32,Wrapping<Generation>,Vec<Entry<i32, Wrapping<Generation>>>>::new();
        let id = genvec.insert(45);
        genvec.remove(id);
        assert_ne!(genvec, GenVecOf::<i32,Wrapping<Generation>,Vec<Entry<i32, Wrapping<Generation>>>>::new());
    }

    #[test]
    fn rollback_insert_wrapping_4()
    {
        let mut genvec = GenVecOf::<i32,Wrapping<Generation>,Vec<Entry<i32, Wrapping<Generation>>>>::new();
        let _ = genvec.insert(45);

        //dbg!(&genvec);
        let id = genvec.insert(42);
        //dbg!(&genvec);
        genvec.rollback_insert(id).unwrap();
        //dbg!(&genvec);

        let mut old_gen: GenVecOf<i32, Wrapping<Generation>,Vec<Entry<i32, Wrapping<Generation>>>> = GenVecOf::new();
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


// Note: it is useless to have view_mut type (for iterating over slot) because this will break the invariant
// Todo: be able to change the generation of a slot ?