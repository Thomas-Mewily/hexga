use crate::*;
use std::{fmt::Debug, hash::{Hash, Hasher}, iter::FusedIterator, marker::PhantomData, ops::{Index, IndexMut}, usize};


type Generation=u32;

pub type GenVec<T> = GenVecOf<T,Generation>;
pub type GenID<T>  = GenIDOf<T,Generation>;

pub trait IGeneration             : Eq + Hash + Ord + Increase + Decrease + NumberAttibute + Debug + Zero + MaxValue + MinValue + Copy {}
impl<T> IGeneration for T where T : Eq + Hash + Ord + Increase + Decrease + NumberAttibute + Debug + Zero + MaxValue + MinValue + Copy {}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenVecOf<T,Gen:IGeneration=Generation>
{
    slot  : Vec<Slot<T,Gen>>,
    head  : usize,
    len   : usize,
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
            None => Ok(Self::new(usize::MAX, Gen::ZERO)),
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

    pub const NULL : Self = GenIDOf { index: usize::MAX, generation: Gen::ZERO, value: PhantomData };
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

impl<T,Gen:IGeneration> GenVecOf<T,Gen>
{
    pub const fn new() -> Self { Self { slot: Vec::new(), head : usize::MAX, len : 0 }}
    pub fn with_capacity(capacity : usize) -> Self { Self { slot: Vec::with_capacity(capacity), head : usize::MAX, len : 0 }}

    pub fn capacity(&self) -> usize { self.slot.capacity() }
    pub fn shrink_to_fit(mut self) { self.slot.shrink_to_fit(); }

    /// Clear the [GenVec], while also invalidating all previous [GenID]
    pub fn clear(&mut self) 
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

            let generation = Gen::ZERO;
            self.slot.push(Slot { value: SlotValue::Used(value), generation });
            return GenIDOf::new(index, generation);
        }

        let SlotValue::Free(next_free_idx) = self.slot[self.head].value else { unreachable!(); };
        let head = self.head;
        self.head = next_free_idx;
        self.slot[head].value = SlotValue::Used(value);
        return GenIDOf::new(head, self.slot[head].generation);
    }

    pub fn get_slot_index(&self, idx : usize) -> Option<&Slot<T,Gen>> { self.slot.get(idx) }
    pub(crate) fn get_slot_index_mut(&mut self, idx : usize) -> Option<&mut Slot<T,Gen>> { self.slot.get_mut(idx) }

    pub fn get_index(&self, idx : usize) -> Option<&T> { self.get_slot_index(idx).and_then(|s| s.value()) }
    pub fn get_index_mut(&mut self, idx : usize) -> Option<&mut T> { self.get_slot_index_mut(idx).and_then(|s| s.value_mut()) }

    pub fn get_slot(&self, id : GenIDOf<T,Gen>) -> Option<&Slot<T,Gen>> { self.get_slot_index(id.index).filter(|v| v.generation() == id.generation()) }
    pub(crate) fn get_slot_mut(&mut self, id : GenIDOf<T,Gen>) -> Option<&mut Slot<T,Gen>> { self.get_slot_index_mut(id.index).filter(|v| v.generation() == id.generation()) }
    
    pub fn get(&self, id : GenIDOf<T,Gen>) -> Option<&T> { self.get_slot(id).and_then(|v| v.value()) }
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

        if f.is_not_max_value()
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
    fn index(&self, index: GenIDOf<T,Gen>) -> &Self::Output { self.get(index).unwrap() }
}
impl<T, Gen:IGeneration> IndexMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    fn index_mut(&mut self, index: GenIDOf<T,Gen>) -> &mut Self::Output { self.get_mut(index).unwrap() }
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
        let slots : Vec<Slot<T,Gen>> = iter.into_iter().map(|v| Slot::new(SlotValue::Used(v), Gen::ZERO)).collect();
        let len = slots.len();
        Self{ slot: slots, head: usize::MAX, len }
    }
}

impl<T, Gen: IGeneration> IntoIterator for GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, T);
    type IntoIter = IntoIter<T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.slot.into_iter().enumerate(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter<T, Gen: IGeneration> {
    iter: std::iter::Enumerate<std::vec::IntoIter<Slot<T, Gen>>>,
}

impl<T, Gen: IGeneration> Iterator for IntoIter<T, Gen> {
    type Item = (GenIDOf<T, Gen>, T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.iter.next() {
            if let SlotValue::Used(value) = slot.value {
                return Some((GenIDOf::new(idx, slot.generation), value));
            }
        }
        None
    }
}
impl<T, Gen: IGeneration> FusedIterator for IntoIter<T, Gen> {}


impl<'a, T, Gen: IGeneration> IntoIterator for &'a GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a T);
    type IntoIter = Iter<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.slot.iter().enumerate(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T, Gen: IGeneration> {
    iter: std::iter::Enumerate<std::slice::Iter<'a, Slot<T, Gen>>>,
}

impl<'a, T, Gen: IGeneration> Iterator for Iter<'a, T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.iter.next() {
            if let Some(value) = slot.value() {
                return Some((GenIDOf::new(idx, slot.generation), value));
            }
        }
        None
    }
}
impl<'a, T, Gen: IGeneration> FusedIterator for Iter<'a, T, Gen> {}



impl<'a, T, Gen: IGeneration> IntoIterator for &'a mut GenVecOf<T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a mut T);
    type IntoIter = IterMut<'a, T, Gen>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.slot.iter_mut().enumerate(),
        }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T, Gen: IGeneration> {
    iter: std::iter::Enumerate<std::slice::IterMut<'a, Slot<T, Gen>>>,
}

impl<'a, T, Gen: IGeneration> Iterator for IterMut<'a, T, Gen> {
    type Item = (GenIDOf<T, Gen>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.iter.next() {
            let generation = slot.generation();
            if let Some(value) = slot.value_mut() {
                return Some((GenIDOf::new(idx, generation), value));
            }
        }
        None
    }
}
impl<'a, T, Gen: IGeneration> FusedIterator for IterMut<'a, T, Gen> {}

impl<T,Gen:IGeneration> Length for GenVecOf<T,Gen>
{
    fn len(&self) -> usize { self.len }
}
//impl<T,Gen:IGeneration> typed_index::IndexLike for GenIDOf<T,Gen>{}


impl<T,Gen:IGeneration> GetIndex<usize> for GenVecOf<T,Gen>
{
    fn get(&self, idx : usize) -> Option<&Self::Output> { self.get_index(idx) }
}
impl<T,Gen:IGeneration> GetIndex<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    fn get(&self, idx : GenIDOf<T,Gen>) -> Option<&Self::Output> { self.get(idx) }
}
impl<T,Gen:IGeneration> GetIndexMut<usize> for GenVecOf<T,Gen>
{
    fn get_mut(&mut self, idx : usize) -> Option<&mut Self::Output> { self.get_index_mut(idx) }
}
impl<T,Gen:IGeneration> GetIndexMut<GenIDOf<T,Gen>> for GenVecOf<T,Gen>
{
    fn get_mut(&mut self, idx : GenIDOf<T,Gen>) -> Option<&mut Self::Output> { self.get_mut(idx) }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests 
{
    use std::num::Wrapping;

    use super::*;

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
        v.clear();
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
        dbg!(v);
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
        // We can't know if the gen vec is new or is the gen vec just wrapped

        let mut gen_vec = GenVecOf::<i32,Wrapping<Generation>>::new();
        let old_gen = gen_vec.clone();

        // dbg!(&gen_vec);
        let id = gen_vec.insert(42);
        // dbg!(&gen_vec);
        gen_vec.rollback_insert(id).unwrap();
        // dbg!(&gen_vec);

        assert_eq!(gen_vec, old_gen);
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