use super::*;

pub mod prelude
{
    pub use super::{GenID};
    pub(crate) use super::{Generation,IGeneration,GenIDOf,CollectionWithGenVecID};
}

pub trait IGeneration            : Eq + Hash + Ord + Increment + Decrement + OverflowBehavior + Debug + MaxValue + MinValue + Copy {}
impl<T> IGeneration for T where T: Eq + Hash + Ord + Increment + Decrement + OverflowBehavior + Debug + MaxValue + MinValue + Copy {}

pub type Generation = u32;
pub type GenID<T>  = GenIDOf<T,Generation>;


/// TODO: Make it untyped ? Remove the T generic type

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
impl<T, Gen:IGeneration> Serialize for GenIDOf<T,Gen> where Gen : Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        if self.index.is_max_value()
        {
            None
        }else
        {
            Some((self.index, self.generation))
        }.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, Gen:IGeneration> Deserialize<'de> for GenIDOf<T,Gen> where Gen : Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
    {
        match Option::deserialize(deserializer)?
        {
            Some((index, generation)) => Ok(Self::from_index_and_generation(index, generation)),
            None => Ok(Self::from_index_and_generation(usize::MAX, Gen::MIN)),
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

// Todo: add GetManyMut ?
pub trait CollectionWithGenVecID<T,Gen:IGeneration> : Get<GenIDOf<T,Gen>,Output=T> + GetMut<GenIDOf<T,Gen>,Output=T> + Remove<GenIDOf<T,Gen>,Output = T> {}
impl<T,Gen:IGeneration,S> CollectionWithGenVecID<T,Gen> for S where S: Get<GenIDOf<T,Gen>,Output=T> + GetMut<GenIDOf<T,Gen>,Output=T> + Remove<GenIDOf<T,Gen>,Output = T> {}

impl<T,Gen:IGeneration> GenIDOf<T,Gen>
{
    pub const fn from_index_and_generation(index: usize, generation : Gen) -> Self { Self { index, generation, value: PhantomData }}

    pub const fn index(self) -> usize { self.index }
    pub const fn generation(self) -> Gen { self.generation }

    pub fn is_null(self) -> bool { self == Self::NULL }
    pub fn is_not_null(self) -> bool { self != Self::NULL }


    pub fn get<C>(self, c: &C) -> Option<&T> where C: CollectionWithGenVecID<T,Gen> { c.get(self) }
    pub fn get_mut<C>(self, c: &mut C) -> Option<&mut T> where C: CollectionWithGenVecID<T,Gen> { c.get_mut(self) }

    pub fn remove<C>(self, c: &mut C) -> Option<T> where C: CollectionWithGenVecID<T,Gen> { c.remove(self) }
    pub fn exist<C>(self, c: &C) -> bool where C: CollectionWithGenVecID<T,Gen> { self.get(c).is_some() }

    pub fn from_other_id<T2>(other : GenIDOf<T2,Gen>) -> GenIDOf<T,Gen> { GenIDOf::from_index_and_generation(other.index, other.generation) }

    /// Set the value to `Self::NULL`
    pub fn reset(&mut self) { *self = Self::NULL; }

    pub const NULL : Self = GenIDOf { index: usize::MAX, generation: Gen::MIN, value: PhantomData };
}
impl<T,Gen:IGeneration> From<(usize,Gen)> for GenIDOf<T,Gen>
{
    fn from((index,generation): (usize,Gen)) -> Self {
        Self::from_index_and_generation(index, generation)
    }
}
impl<T,Gen:IGeneration> Into<(usize,Gen)> for GenIDOf<T,Gen>
{
    fn into(self) -> (usize,Gen) {
        (self.index, self.generation)
    }
}