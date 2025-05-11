use crate::*;
use std::{collections::HashMap, fmt::Debug, hash::{Hash, Hasher}, iter::FusedIterator, marker::PhantomData, ops::{Index, IndexMut}};


pub type Generation = u32;

pub type GenVec<T> = GenVecOf<T,Generation>;
pub type GenID<T>  = GenIDOf<T,Generation>;

pub trait IGeneration             : Eq + Hash + Ord + Increase + Decrease + NumberAttibute + Debug + MaxValue + MinValue + Copy {}
impl<T> IGeneration for T where T : Eq + Hash + Ord + Increase + Decrease + NumberAttibute + Debug + MaxValue + MinValue + Copy {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlotValue<T>
{
    Used(T),
    // Next free
    Free(usize),
}
impl<T> SlotValue<T>
{
    pub fn get(&self) -> Option<&T> { if let Self::Used(v) = self { Some(v) } else { None }}
    pub fn get_mut(&mut self) -> Option<&mut T> { if let Self::Used(v) = self { Some(v) } else { None }}

    pub fn take_and_free(&mut self, free_index: usize) -> T {
        match std::mem::replace(self, SlotValue::Free(free_index)) {
            SlotValue::Used(value) => value,
            SlotValue::Free(_) => panic!("Slot was already free"),
        }
    }

    pub fn is_free(&self) -> bool { matches!(self, Self::Free(_))}
    pub fn is_used(&self) -> bool { matches!(self, Self::Used(_))}
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slot<T,Gen:IGeneration=Generation>
{
    value      : SlotValue<T>,
    generation : Gen,
}
impl <T,Gen:IGeneration> Slot<T,Gen>
{
    pub fn new(value : SlotValue<T>, generation : Gen) -> Self { Self { value, generation }}
    pub fn generation(&self) -> Gen { self.generation }

    pub fn have_value(&self) -> bool { self.value().is_some() }

    pub fn value(&self) -> Option<&T> { self.value.get() }
    pub fn value_mut(&mut self) -> Option<&mut T> { self.value.get_mut() }

    pub fn get_id(&self, idx : usize) -> GenIDOf<T,Gen> { GenIDOf::new(idx, self.generation) }

    pub fn generation_increase(&mut self) -> bool { if self.can_generation_increase() { self.generation.increase(); true } else { false } }
    pub fn can_generation_increase(&self) -> bool { self.generation.can_increase() }

    pub fn generation_decrease(&mut self) -> bool { if self.can_generation_decrease() { self.generation.decrease(); true } else { false } }
    pub fn can_generation_decrease(&self) -> bool { self.generation.can_decrease() }

    pub fn is_generation_saturated(&self) -> bool { !self.can_generation_increase() }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct GenVecOf<T,Gen:IGeneration=Generation>
{
    slot  : Vec<Slot<T,Gen>>,
    head  : usize,
    len   : usize,
}

impl<T, Gen:IGeneration> PartialEq for GenVecOf<T,Gen> where T : PartialEq
{
    fn eq(&self, other: &Self) -> bool 
    {
        if !Gen::OVERFLOW_BEHAVIOR.is_wrapping()
        {
            self.len == other.len && self.slot == other.slot && self.head == other.head
        }else
        {
            /* 
                We can't know if the gen vec is new or if the gen vec just wrapped arround.
            
                Those two are equal: (Assuming Gen::MIN value is 0)
                A: GenVecOf { slot: [Slot { value: Free(18446744073709551615), generation: 0 }], head: 0, len: 0 }
                B: GenVecOf { slot: [], head: 18446744073709551615, len: 0 }

                Both can represent unused wrapped gen vec.
                
                doing :

                let id = B.insert(10);
                B.rollback_insert(id);

                will put A in the same equal state/representation as B.


                But these 2 are different, because that generation was already used.

                X: GenVecOf { slot: [Slot { value: Free(18446744073709551615), generation: 1 }], head: 0, len: 0 }
                Y: GenVecOf { slot: [], head: 18446744073709551615, len: 0 }
            */

            if self.len != other.len { return false; }
            if self.head == other.head { return self.slot == other.slot; }
            if !(self.head.is_max_value() ^ other.head.is_max_value()) { return false; }

            if self.head.is_max_value()
            {
                if self.slot.len() + 1 != other.slot.len() { return false; }
                let mid = other.head;
                debug_assert!(!mid.is_max_value());

                let slot = other.get_slot_index(mid).unwrap();
                let SlotValue::Free(f) = slot.value else { return false; };
                if !f.is_max_value() || !slot.generation().is_min_value() { return false; }

                let self_left = &self.slot[0..mid];
                let self_right = &self.slot[mid..];

                let other_left = &other.slot[0..mid];
                let other_right = &other.slot[mid+1..];

                self_left == other_left && self_right == other_right
            }else if other.head.is_max_value()
            {
                if other.slot.len() + 1 != self.slot.len() { return false; }
                let mid = self.head;
                debug_assert!(!mid.is_max_value());

                let slot = self.get_slot_index(mid).unwrap();
                let SlotValue::Free(f) = slot.value else { return false; };
                if !f.is_max_value() || !slot.generation().is_min_value() { return false; }

                let other_left = &other.slot[0..mid];
                let other_right = &other.slot[mid..];

                let self_left = &self.slot[0..mid];
                let self_right = &self.slot[mid+1..];

                other_left == self_left && other_right == self_right
            }else
            {
                unreachable!()
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<T, Gen:IGeneration> Serialize for GenVecOf<T,Gen> where Slot<T, Gen> : Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { 
        let mut state = serializer.serialize_struct("GenVec", 2)?;
        state.serialize_field("slot", &self.slot)?;
        // need to be in the same order on all machine for determinist
        state.serialize_field("free", &self.head)?;
        state.end()
    }
}


impl<T, Gen:IGeneration> GenVecOf<T,Gen>
{
    pub(crate) fn new_and_check_invariant(slot : Vec<Slot<T, Gen>>, head : usize) -> Result<Self, String>
    {            
        let len = slot.iter().filter(|s| s.have_value()).count();
        
        if slot.len() == usize::MAX 
        {
            return Err("GenVec : the last usize value is used for null in a GenVec and cannot be used".to_owned());
        }

        let mut nb_use = len;
        let mut cur_head = head;

        while nb_use != 0 
        {
            let Some(next_slot) = slot.get(cur_head) else { return Err(format!("GenVec : slot {:?} is out of range", cur_head)); };
            let SlotValue::Free(f) = next_slot.value else { return Err(format!("GenVec : slot {:?} was not free", cur_head)); };
            if f == usize::MAX { return Err(format!("GenVec : invalid free head {:?} at {:?}", f, cur_head));}
            cur_head = f;
            nb_use -= 1;
        }

        Ok(Self{ slot, head, len})
    }
}


#[cfg(feature = "serde")]
impl<'de,T, Gen> Deserialize<'de> for GenVecOf<T, Gen>
where
    Gen: IGeneration + Deserialize<'de>,
    Slot<T, Gen>: Deserialize<'de>,
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
            Slot<T, Gen>: Deserialize<'de>,
        {
            type Value = GenVecOf<T, Gen>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a struct representing GenVec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values : Option<Vec<Slot<T, Gen>>> = None;
                let mut free_index : Option<usize> = None;

                while let Some(key) = map.next_key::<&'de str>()? 
                {
                    match key 
                    {
                        "slot" => {
                            if values.is_some() {
                                return Err(de::Error::duplicate_field("slot"));
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
                                &["slot", "free"],
                            ));
                        }
                    }
                }

                let slot = values.ok_or_else(|| de::Error::missing_field("slot"))?;
                let free = free_index.ok_or_else(|| de::Error::missing_field("free"))?;
                GenVecOf::<T,Gen>::new_and_check_invariant(slot, free).map_err(|e| de::Error::custom(e))
            }
        }

        const FIELDS: &[&str] = &["slot", "free"];
        deserializer.deserialize_struct(
            "GenVec",
            FIELDS,
            GenVecVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

pub struct GenIDOf<T,Gen:IGeneration>
{
    index      : usize,
    generation : Gen,
    value      : PhantomData<T>,
}

impl<T,Gen:IGeneration> Default for GenIDOf<T,Gen>
{
    fn default() -> Self { Self::NULL }
}

#[cfg(feature = "serde")]
impl<T, Gen:IGeneration> Serialize for GenIDOf<T,Gen> where T : Serialize, Gen : Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { 
        if self.index.is_max_value()
        {
            Some((self.index, self.generation))
        }else
        {
            None
        }.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, Gen:IGeneration> Deserialize<'de> for GenIDOf<T,Gen> where T : Deserialize<'de>, Gen : Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
    {
        match Option::deserialize(deserializer)?
        {
            Some((index, generation)) => Ok(Self::new(index, generation)),
            None => Ok(Self::new(usize::MAX, Gen::MIN)),
        }
    }
}

impl<T,Gen:IGeneration> Clone for GenIDOf<T,Gen>{ fn clone(&self) -> Self { Self { index: self.index.clone(), generation: self.generation.clone(), value: PhantomData } } }
impl<T,Gen:IGeneration> Copy for GenIDOf<T,Gen> {}

impl<T,Gen:IGeneration> PartialEq for GenIDOf<T,Gen> { fn eq(&self, other: &Self) -> bool { self.index == other.index && self.generation == other.generation } }
impl<T,Gen:IGeneration> Eq for GenIDOf<T,Gen> {}

impl<T,Gen:IGeneration> Hash for GenIDOf<T,Gen> { fn hash<H: Hasher>(&self, state: &mut H) { self.index.hash(state); self.generation.hash(state); } }

impl<T,Gen:IGeneration> Ord for GenIDOf<T,Gen> { fn cmp(&self, other: &Self) -> std::cmp::Ordering { (self.index, self.generation).cmp(&(other.index, other.generation)) } }
impl<T,Gen:IGeneration> PartialOrd for GenIDOf<T,Gen> { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(&other)) } }

impl<T,Gen:IGeneration> Debug for GenIDOf<T,Gen> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}#{:?}", self.index, self.generation) } }

impl<T,Gen:IGeneration> GenIDOf<T,Gen>
{
    pub const fn new(index : usize, generation : Gen) -> Self { Self { index, generation, value: PhantomData }}

    pub const fn index(self) -> usize { self.index }
    pub const fn generation(self) -> Gen { self.generation }

    pub fn is_null(self) -> bool { self == Self::NULL }
    pub fn is_not_null(self) -> bool { self != Self::NULL }

    pub fn get(self, gen_vec : &GenVecOf<T,Gen>) -> Option<&T> { gen_vec.get(self) }
    pub fn get_mut(self, gen_vec : &mut GenVecOf<T,Gen>) -> Option<&mut T> { gen_vec.get_mut(self) }

    pub fn remove(self, gen_vec : &mut GenVecOf<T,Gen>) -> Option<T> { gen_vec.remove(self) }
    pub fn exist(self, gen_vec : &GenVecOf<T,Gen>) -> bool { self.get(gen_vec).is_some() }

    pub const NULL : Self = GenIDOf { index: usize::MAX, generation: Gen::MIN, value: PhantomData };
}
impl<T,Gen:IGeneration> From<(usize,Gen)> for GenIDOf<T,Gen>
{
    fn from((index,generation): (usize,Gen)) -> Self {
        Self::new(index, generation)
    }
}
impl<T,Gen:IGeneration> Into<(usize,Gen)> for GenIDOf<T,Gen>
{
    fn into(self) -> (usize,Gen) {
        (self.index, self.generation)
    }
}

impl<T,Gen:IGeneration> Default for GenVecOf<T,Gen>
{
    fn default() -> Self { Self::new() }
}

impl<T,Gen:IGeneration> GenVecOf<T,Gen>
{
    pub const fn new() -> Self { Self { slot: Vec::new(), head : usize::MAX, len : 0 }}
    pub fn with_capacity(capacity : usize) -> Self { Self { slot: Vec::with_capacity(capacity), head : usize::MAX, len : 0 }}

    pub fn capacity(&self) -> usize { self.slot.capacity() }
    pub fn shrink_to_fit(mut self) { self.slot.shrink_to_fit(); }


    /// Clear the [GenVec] but don't invalidate all previous [GenID].
    pub fn clear(&mut self) 
    {
        self.head = usize::MAX;
        self.len = 0;
        self.slot.clear();
    }

    /// Clear the [GenVec] and also invalidate all previous [GenID].
    pub fn remove_all(&mut self)
    {
        for (idx, v) in self.slot.iter_mut().enumerate()
        {
            if v.have_value()
            {
                if v.generation_increase()
                {
                    v.value = SlotValue::Free(self.head);
                    self.head = idx;
                }else
                {
                    v.value = SlotValue::Free(usize::MAX);
                }
            }
        }
        self.len = 0;
    }

    pub fn rollback_insert(&mut self, id : GenIDOf<T,Gen>) -> Result<T,()>
    {
        let idx = id.index;
        let head = self.head;

        let slot_len = self.slot.len();

        let Some(slot) = self.get_slot_index_mut(idx) else { return Err(()); };
        if slot.value.is_free() { return Err(()); }

        if head.is_max_value()
        {
            if idx + 1 != slot_len { return Err(()); }
        }

        let can_not_decrease = !slot.can_generation_decrease();
        let val = slot.value.take_and_free(head);
        self.len -= 1;

        if head.is_max_value() && can_not_decrease
        {
            self.slot.pop().ok_or(())?;
        }else
        {
            self.head = idx;
        }

        Ok(val)
    }
    pub fn insert(&mut self, value : T) ->  GenIDOf<T,Gen>
    {
        self.len += 1;

        if self.head == usize::MAX
        {
            let index = self.slot.len();

            // The last index is used for the null() key
            assert!(index != usize::MAX, "How you didn't run out of memory before ?");

            let generation = Gen::MIN;
            self.slot.push(Slot { value: SlotValue::Used(value), generation });
            return GenIDOf::new(index, generation);
        }

        let SlotValue::Free(next_free_idx) = self.slot[self.head].value else { unreachable!(); };
        let head = self.head;
        self.head = next_free_idx;
        self.slot[head].value = SlotValue::Used(value);
        return GenIDOf::new(head, self.slot[head].generation);
    }

    #[inline(always)]
    pub fn get_slot_index(&self, idx : usize) -> Option<&Slot<T,Gen>> { self.slot.get(idx) }
    #[inline(always)]
    pub(crate) fn get_slot_index_mut(&mut self, idx : usize) -> Option<&mut Slot<T,Gen>> { self.slot.get_mut(idx) }

    #[inline(always)]
    pub fn get_index(&self, idx : usize) -> Option<&T> { self.get_slot_index(idx).and_then(|s| s.value()) }
    #[inline(always)]
    pub fn get_index_mut(&mut self, idx : usize) -> Option<&mut T> { self.get_slot_index_mut(idx).and_then(|s| s.value_mut()) }

    #[inline(always)]
    pub fn get_slot(&self, id : GenIDOf<T,Gen>) -> Option<&Slot<T,Gen>> { self.get_slot_index(id.index).filter(|v| v.generation() == id.generation()) }
    #[inline(always)]
    pub(crate) fn get_slot_mut(&mut self, id : GenIDOf<T,Gen>) -> Option<&mut Slot<T,Gen>> { self.get_slot_index_mut(id.index).filter(|v| v.generation() == id.generation()) }
    
    #[inline(always)]
    pub fn get(&self, id : GenIDOf<T,Gen>) -> Option<&T> { self.get_slot(id).and_then(|v| v.value()) }
    #[inline(always)]
    pub fn get_mut(&mut self, id : GenIDOf<T,Gen>) -> Option<&mut T> { self.get_slot_mut(id).and_then(|v| v.value_mut()) }

    /// Return a valid [GenID] to the current index or return null if the idx is outside the range
    pub fn get_id(&self, idx : usize) -> GenIDOf<T, Gen>
    {
        self.get_slot_index(idx).map(|v| v.get_id(idx)).unwrap_or(GenIDOf::NULL)
    }

    /// The operation that once done just after an [Self::remove_index], put this data structure in the same state as before
    pub fn rollback_remove_index(&mut self, idx : usize, value : T) -> Result<(), ()>
    {
        let mut head = self.head;
        let slot = self.get_slot_index_mut(idx).ok_or(())?;
        let SlotValue::Free(f) = slot.value else { return Err(()); };
        let free = f;

        if f.is_non_max_value()
        {
            if head != idx { return Err(()); }
            head = free;
            if !slot.generation_decrease() { return Err(()); }
        }else 
        {
            // Slot don't have a next free slot
            if head == idx 
            {
                head = usize::MAX;
                if !slot.generation_decrease() { return Err(()); }
            }else if !slot.is_generation_saturated() 
            { 
                return Err(()); 
            }
        }

        slot.value = SlotValue::Used(value);

        self.head = head;
        self.len += 1;

        Ok(())
    }
    pub fn remove_index(&mut self, idx : usize) -> Option<T>
    {
        let head = self.head;

        let Some(slot) = self.get_slot_index_mut(idx) else { return None; };
        if slot.value.is_free() { return None; }

        let val = slot.value.take_and_free(head);

        if slot.generation_increase()
        {
            self.head = idx;
        }else
        {
            slot.value = SlotValue::Free(usize::MAX);
        }
        self.len -= 1;

        Some(val)
    }

    pub fn rollback_remove(&mut self, id : GenIDOf<T,Gen>, value : T) -> Result<(), ()>
    {
        // Todo : missing some check to see if the last operation removal was done with id
        self.rollback_remove_index(id.index, value)
    }
    pub fn remove(&mut self, id : GenIDOf<T,Gen>) -> Option<T>
    {
        if self.get(id).is_none() { return None; }
        self.remove_index(id.index)
    }

    /* 
    pub(crate) fn iter_slot(&self) -> impl Iterator<Item = &Slot<T,Gen>> { self.slot.iter() }
    pub(crate) fn iter_slot_mut(&mut self) -> impl Iterator<Item = &mut Slot<T,Gen>> { self.slot.iter_mut() }

    pub(crate) fn iter_slot_with_value(&self) -> impl Iterator<Item = &Slot<T,Gen>> { self.iter_slot().filter(|e| e.have_value()) }
    pub(crate) fn iter_slot_with_value_mut(&mut self) -> impl Iterator<Item = &mut Slot<T,Gen>> { self.iter_slot_mut().filter(|e| e.have_value()) }
    */

    pub fn iter(&self) -> Iter<'_, T, Gen> { self.into_iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_, T, Gen> { self.into_iter() }

    pub fn ids(&self) -> impl Iterator<Item = GenIDOf<T,Gen>> { self.into_iter().map(|(id, _val)| id) }
    pub fn values(&self) -> impl Iterator<Item = &T> { self.iter().map(|(_,val)| val) }

    pub fn into_ids(self) -> impl Iterator<Item = GenIDOf<T,Gen>> { self.into_iter().map(|(id, _val)| id) }
    pub fn into_values(self) -> impl Iterator<Item = T> { self.into_iter().map(|(_id, val)| val) }

    /// Use this instead of `0..gen_vec.len()`. `0..gen_vec.capacity()` is the correct way to do it internally
    pub fn iter_index(&self) -> impl Iterator<Item = usize> { 0..self.capacity() }
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
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<T, Gen:IGeneration> IndexMut<usize> for GenVecOf<T,Gen>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<T, Gen:IGeneration> FromIterator<T> for GenVecOf<T, Gen>
{
    fn from_iter<K: IntoIterator<Item = T>>(iter: K) -> Self {
        let slots : Vec<Slot<T,Gen>> = iter.into_iter().map(|v| Slot::new(SlotValue::Used(v), Gen::MIN)).collect();
        let len = slots.len();
        Self{ slot: slots, head: usize::MAX, len }
    }
}

impl<T, Gen: IGeneration> IntoIterator for GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, T);
    type IntoIter = IntoIter<T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter 
        {
            iter: self.slot.into_iter().enumerate(),
            len_remaining: self.len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter<T, Gen: IGeneration> 
{
    iter: std::iter::Enumerate<std::vec::IntoIter<Slot<T, Gen>>>,
    len_remaining : usize,
}

impl<T, Gen: IGeneration> Iterator for IntoIter<T, Gen> {
    type Item = (GenIDOf<T, Gen>, T);

    fn next(&mut self) -> Option<Self::Item> 
    {
        while let Some((idx, slot)) = self.iter.next() 
        {
            if let SlotValue::Used(value) = slot.value 
            {
                self.len_remaining -= 1;
                return Some((GenIDOf::new(idx, slot.generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<T, Gen: IGeneration> FusedIterator for IntoIter<T, Gen> {}


impl<'a, T, Gen: IGeneration> IntoIterator for &'a GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a T);
    type IntoIter = Iter<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.slot.iter().enumerate(),
            len_remaining : self.len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T, Gen: IGeneration> 
{
    iter: std::iter::Enumerate<std::slice::Iter<'a, Slot<T, Gen>>>,
    len_remaining : usize,
}

impl<'a, T, Gen: IGeneration> Iterator for Iter<'a, T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.iter.next() {
            if let Some(value) = slot.value() {
                self.len_remaining -= 1;
                return Some((GenIDOf::new(idx, slot.generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<'a, T, Gen: IGeneration> FusedIterator for Iter<'a, T, Gen> {}



impl<'a, T, Gen: IGeneration> IntoIterator for &'a mut GenVecOf<T, Gen> 
{
    type Item = (GenIDOf<T, Gen>, &'a mut T);
    type IntoIter = IterMut<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.slot.iter_mut().enumerate(),
            len_remaining : self.len,
        }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T, Gen: IGeneration> 
{
    iter: std::iter::Enumerate<std::slice::IterMut<'a, Slot<T, Gen>>>,
    len_remaining : usize,
}

impl<'a, T, Gen: IGeneration> Iterator for IterMut<'a, T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.iter.next() {
            let generation = slot.generation();
            if let Some(value) = slot.value_mut() {
                self.len_remaining -= 1;
                return Some((GenIDOf::new(idx, generation), value));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len_remaining, Some(self.len_remaining)) }
}
impl<'a, T, Gen: IGeneration> FusedIterator for IterMut<'a, T, Gen> {}

impl<T,Gen:IGeneration> Length for GenVecOf<T,Gen> { #[inline(always)] fn len(&self) -> usize { self.len } }
impl<T,Gen:IGeneration> Capacity for GenVecOf<T,Gen> 
{
    type Param=();

    #[inline(always)]
    fn capacity(&self) -> usize { self.slot.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.slot.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.slot.reserve_exact(additional); }
    
    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.slot.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.slot.try_reserve_exact(additional) }
}
impl<T,Gen:IGeneration> Clearable for GenVecOf<T,Gen> { #[inline(always)] fn clear(&mut self) { self.clear(); } }

impl<T,Gen:IGeneration> Get<usize> for GenVecOf<T,Gen>
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn try_get(&self, idx : usize) -> Result<&Self::Output, ()> { self.get_index(idx).ok_or_void() }
    #[inline(always)]
    fn get(&self, idx : usize) -> Option<&Self::Output> { self.get_index(idx) }
}
impl<T,Gen:IGeneration> Get<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    type Output = <Self as Index<GenIDOf<T,Gen>>>::Output;
    #[inline(always)]
    fn try_get(&self, idx : GenIDOf<T,Gen>) -> Result<&Self::Output, ()> { self.get(idx).ok_or_void() }
    #[inline(always)]
    fn get(&self, idx : GenIDOf<T,Gen>) -> Option<&Self::Output> { self.get(idx) }
}

impl<T,Gen:IGeneration> GetMut<usize> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn try_get_mut(&mut self, idx : usize) -> Result<&mut Self::Output, ()> { self.get_index_mut(idx).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, idx : usize) -> Option<&mut Self::Output> { self.get_index_mut(idx) }
}

impl<T,Gen:IGeneration> GetManyMut<usize> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Result<[&mut Self::Output;N], ()> 
    { 
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.slot.try_get_many_mut(indices).map(|slots| slots.map(|v| v.value_mut()))
        {
            Ok(values) => if values.iter().any(|v| v.is_none()) { Err(()) } else { Ok(values.map(|v| v.unwrap())) },
            Err(()) => Err(()),
        }
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut Self::Output;N] {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        unsafe { self.slot.get_many_unchecked_mut(indices).map(|v| v.value_mut().unwrap()) }
    }
}
impl<T,Gen:IGeneration> GetMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn try_get_mut(&mut self, idx : GenIDOf<T,Gen>) -> Result<&mut Self::Output, ()> { self.get_mut(idx).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, idx : GenIDOf<T,Gen>) -> Option<&mut Self::Output> { self.get_mut(idx) }
}

impl<T,Gen:IGeneration> GetManyMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    #[inline(always)]
    fn try_get_many_mut<const N: usize>(&mut self, indices: [GenIDOf<T,Gen>; N]) -> Result<[&mut Self::Output;N], ()> 
    { 
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        match self.slot.try_get_many_mut(indices.map(|i| i.index))
        {
            Ok(values) => if values.iter().enumerate().any(|(idx,v)| !v.have_value() || v.generation() != indices[idx].generation) 
            { Err(()) } else { Ok(values.map(|v| v.value_mut().unwrap())) },
            Err(_) => Err(()),
        }
    }
}



impl<T,Gen:IGeneration> GenVecOf<T,Gen>
{
    /// Moves all the elements of `other` into `self`, leaving `other` empty (clear the element).
    pub fn append(&mut self, other: &mut GenVecOf<T,Gen>) -> impl GenIDUpdater<T,Gen> + use<T,Gen> where T : GenIDUpdatable<T,Gen>
    {
        let capacity = other.len();
        let mut h = HashMap::with_capacity(capacity);

        for (idx, slot) in other.slot.iter_mut().enumerate().filter(|(_,s)| s.have_value())
        {
            let val = slot.value.take_and_free(usize::MAX);
            let old_id = slot.get_id(idx);
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

        assert_eq!(x[x.get_id(0)], 10);
        assert_eq!(x[x.get_id(1)], 20);
        assert_eq!(x[x.get_id(2)], 30);

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
        let enemi = entities.insert("zoombie");

        assert_eq!(enemi.get(&entities), Some(&"zoombie"));
        assert_eq!(entities[enemi], "zoombie");

        assert!(entities.get(enemi).is_some());
        entities.remove(enemi); // the key is no longer valid
        assert!(entities.get(enemi).is_none()); // the value don't exist
        
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
        let mut gen_vec = GenVec::new();
        // dbg!(&gen_vec);

        let id = gen_vec.insert(42);

        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let removed = gen_vec.remove_index(id.index).unwrap();
        // dbg!(&gen_vec);
        gen_vec.rollback_remove_index(id.index, removed).unwrap();
        // dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
    }  

    #[test]
    fn rollback_remove_wrapping_empty() 
    {
        let mut gen_vec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let id = gen_vec.insert(42);

        let old_gen = gen_vec.clone();

        let removed = gen_vec.remove_index(id.index).unwrap();
        gen_vec.rollback_remove_index(id.index, removed).unwrap();

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_remove_wrapping() 
    {
        let mut gen_vec = wrapping_about_to_wrap();
        let id = gen_vec.insert(42);

        let old_gen = gen_vec.clone();

        let removed = gen_vec.remove_index(id.index).unwrap();
        gen_vec.rollback_remove_index(id.index, removed).unwrap();

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_remove_wrapping_2() 
    {
        let mut gen_vec = wrapping_about_to_wrap();
        gen_vec.insert(50);

        let id = gen_vec.insert(42);

        let old_gen = gen_vec.clone();

        let removed = gen_vec.remove_index(id.index).unwrap();
        gen_vec.rollback_remove_index(id.index, removed).unwrap();

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_remove_non_wrapping() 
    {
        let mut gen_vec = non_wrapping_about_to_wrap();
        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);

        let old_gen = gen_vec.clone();

        let removed = gen_vec.remove_index(id.index).unwrap();
        // dbg!(&gen_vec);

        gen_vec.rollback_remove_index(id.index, removed).unwrap();
        // dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_remove_non_wrapping_2() 
    {
        let mut gen_vec = non_wrapping_about_to_wrap();
        gen_vec.insert(50);

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);

        let old_gen = gen_vec.clone();

        let removed = gen_vec.remove_index(id.index).unwrap();
        // dbg!(&gen_vec);

        gen_vec.rollback_remove_index(id.index, removed).unwrap();
        // dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
    }  


    // rollback_insert

    #[test]
    fn rollback_insert_empty() 
    {
        let mut gen_vec = GenVec::new();
        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();

        assert_eq!(gen_vec, old_gen);
    } 

    
    #[test]
    fn rollback_insert_wrapping_empty() 
    {
        // We can't know if the gen vec is new or if the gen vec just wrapped

        let mut gen_vec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let old_gen = gen_vec.clone();

         dbg!(&gen_vec);
        let id = gen_vec.insert(42);
         dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();
         dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_insert_wrapping_3() 
    {
        // We can't know if the gen vec is new or if the gen vec just wrapped

        let mut gen_vec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let _id = gen_vec.insert(45);

        let old_gen = gen_vec.clone();

         dbg!(&gen_vec);
        let id = gen_vec.insert(42);
         dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();
         dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_insert_wrapping_dif() 
    {
        let mut gen_vec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let id = gen_vec.insert(45);
        gen_vec.remove(id);
        assert_ne!(gen_vec, GenVecOf::<i32,Wrapping<Generation>>::new());
    }

    #[test]
    fn rollback_insert_wrapping_4() 
    {
        let mut gen_vec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let _ = gen_vec.insert(45);

         //dbg!(&gen_vec);
        let id = gen_vec.insert(42);
         //dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();
         //dbg!(&gen_vec);

         let mut old_gen = GenVecOf::new();
         old_gen.insert(50);

        assert_ne!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_insert_wrapping() 
    {
        let mut gen_vec = wrapping_about_to_wrap();
        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();
        // dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
    }


    #[test]
    fn rollback_insert_wrapping_2() 
    {
        let mut gen_vec = wrapping_about_to_wrap();
        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_insert_non_wrapping() 
    {
        let mut gen_vec = non_wrapping_about_to_wrap();
        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();

        assert_eq!(gen_vec, old_gen);
    }

    #[test]
    fn rollback_insert_non_wrapping_2() 
    {
        let mut gen_vec = non_wrapping_about_to_wrap();
        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();

        assert_eq!(gen_vec, old_gen);
    }

}