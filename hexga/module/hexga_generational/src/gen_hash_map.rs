use crate::*;
use std::{collections::HashMap, fmt::Debug, hash::{Hash, Hasher}, iter::FusedIterator, marker::PhantomData, ops::{Index, IndexMut}};

/*
pub struct GenHashMapIDOf<K,T,Gen:IGeneration=Generation>
{
    id : GenVecIDOf<T,Gen>,
    phantom : PhantomData<K>,
}

impl<K,T,Gen:IGeneration> GenHashMapIDOf<K,T,Gen>
{
    pub const fn from_gen_vec_id(id: GenVecIDOf<T,Gen>) -> Self { Self { id, phantom: PhantomData } }
    pub const fn id(&self) -> GenVecIDOf<T,Gen> { self.id }

    pub const fn new(index : usize, generation : Gen) -> Self { Self { id: GenVecIDOf::new(index, generation), phantom: PhantomData } }

    pub const fn index(self) -> usize { self.id().index() }
    pub const fn generation(self) -> Gen { self.id().generation() }

    pub fn is_null(self) -> bool { self == Self::NULL }
    pub fn is_not_null(self) -> bool { self != Self::NULL }

    pub fn get(self, gen_hash_map : &GenHashMapOf<K,T,Gen>) -> Option<&T> { gen_hash_map.get(self) }
    pub fn get_mut(self, gen_hash_map : &mut GenHashMapOf<K,T,Gen>) -> Option<&mut T> { gen_hash_map.get_mut(self) }

    pub fn remove(self, gen_hash_map : &mut GenHashMapOf<K,T,Gen>) -> Option<T> { gen_hash_map.remove(self) }
    pub fn exist(self, gen_hash_map : &GenHashMapOf<K,T,Gen>) -> bool { self.get(gen_hash_map).is_some() }

    pub const NULL : Self = GenHashMapIDOf::from_gen_vec_id(GenVecIDOf::NULL);
}

impl<K,T,Gen:IGeneration> From<GenVecIDOf<T,Gen>> for GenHashMapIDOf<K,T,Gen>
{
    fn from(id: GenVecIDOf<T,Gen>) -> Self { Self::from_gen_vec_id(id) }
}

impl<K,T,Gen:IGeneration> Clone for GenHashMapIDOf<K,T,Gen> { fn clone(&self) -> Self { Self { id: self.id.clone(), phantom: PhantomData, } } }
impl<K,T,Gen:IGeneration> Copy for GenHashMapIDOf<K,T,Gen> {}
impl<K,T,Gen:IGeneration> PartialEq for GenHashMapIDOf<K,T,Gen> { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
impl<K,T,Gen:IGeneration> Eq for GenHashMapIDOf<K,T,Gen> {}
impl<K,T,Gen:IGeneration> Hash for GenHashMapIDOf<K,T,Gen> { fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state); } }
impl<K,T,Gen:IGeneration> Debug for GenHashMapIDOf<K,T,Gen> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.id()) } }
impl<K,T,Gen:IGeneration> PartialOrd for GenHashMapIDOf<K,T,Gen> { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.id.partial_cmp(&other.id) } }
impl<K,T,Gen:IGeneration> Ord for GenHashMapIDOf<K,T,Gen> { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.id.cmp(&other.id) } }

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenHashMapOf<K,T,Gen:IGeneration=Generation>
{
    values : GenVecOf<T,Gen>,
    search : HashMap<K,GenVecIDOf<T,Gen>>,
}
    */