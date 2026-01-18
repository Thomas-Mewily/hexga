use super::*;

pub mod prelude
{
    pub use super::{GenID};
    pub(crate) use super::{Generation,IGeneration,GenIDOf,CollectionWithGenVecID};
}

pub trait IGeneration            : Eq + Hash + Ord + Increment + Decrement + OverflowBehavior + Debug + MaxValue + MinValue + Copy {}
impl<T> IGeneration for T where T: Eq + Hash + Ord + Increment + Decrement + OverflowBehavior + Debug + MaxValue + MinValue + Copy {}

// Todo: Make a flag for the generation
pub type Generation = u32;
pub type GenID  = GenIDOf<Generation>;



#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct GenIDOf<Gen:IGeneration>
{
    index      : usize,
    generation : Gen,
}

impl<Gen:IGeneration> Default for GenIDOf<Gen>
{
    fn default() -> Self { Self::NULL }
}

#[cfg(feature = "serde")]
impl<Gen:IGeneration> Serialize for GenIDOf<Gen> where Gen : Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        if self.index.is_max()
        {
            None
        }else
        {
            Some((self.index, self.generation))
        }.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, Gen:IGeneration> Deserialize<'de> for GenIDOf<Gen> where Gen : Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
    {
        match Option::deserialize(deserializer)?
        {
            Some((index, generation)) => Ok(Self::from_index_and_generation(index, generation)),
            None => Ok(Self::from_index_and_generation(usize::MAX, Gen::MIN)),
        }
    }
}
impl<Gen:IGeneration> Debug for GenIDOf<Gen> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}#{:?}", self.index, self.generation) } }

// Todo: add GetManyMut ?
pub trait CollectionWithGenVecID<T,Gen:IGeneration> : Get<GenIDOf<Gen>,Output=T> + GetMut<GenIDOf<Gen>,Output=T> + Remove<GenIDOf<Gen>,Output = T> {}
impl<T,Gen:IGeneration,S> CollectionWithGenVecID<T,Gen> for S where S: Get<GenIDOf<Gen>,Output=T> + GetMut<GenIDOf<Gen>,Output=T> + Remove<GenIDOf<Gen>,Output = T> {}

impl<Gen:IGeneration> GenIDOf<Gen>
{
    pub const fn from_index_and_generation(index: usize, generation : Gen) -> Self { Self { index, generation }}

    pub const fn index(self) -> usize { self.index }
    pub const fn generation(self) -> Gen { self.generation }

    pub fn is_null(self) -> bool { self == Self::NULL }
    pub fn is_not_null(self) -> bool { self != Self::NULL }


    /// Set the value to `Self::NULL`
    pub fn reset(&mut self) { *self = Self::NULL; }

    pub const NULL : Self = GenIDOf { index: usize::MAX, generation: Gen::MIN };
}
impl<Gen:IGeneration> IndexExtension for GenIDOf<Gen> {}

impl<Gen:IGeneration> From<(usize,Gen)> for GenIDOf<Gen>
{
    fn from((index,generation): (usize,Gen)) -> Self {
        Self::from_index_and_generation(index, generation)
    }
}
impl<Gen:IGeneration> Into<(usize,Gen)> for GenIDOf<Gen>
{
    fn into(self) -> (usize,Gen) {
        (self.index, self.generation)
    }
}