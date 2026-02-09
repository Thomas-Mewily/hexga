use super::*;

pub mod prelude
{
    pub use super::{GenID,IGenID};
    pub(crate) use super::{GenIDOf, UntypedGenIDOf, Generation, IGeneration};
}

pub trait IGenID<Gen=Generation> : From<(usize,Gen)> + Into<(usize,Gen)> + Clone + Copy + PartialEq + Eq + Hash + Ord + PartialOrd + Default
    where Gen: IGeneration
{
    fn from_index_and_generation(index: usize, generation: Gen) -> Self { Self::from((index, generation)) }

    fn index(self) -> usize;
    fn generation(self) -> Gen;
    #[inline(always)]
    fn index_and_generation(self) -> (usize, Gen) { (self.index(), self.generation()) }

    fn is_null(self) -> bool { self == Self::NULL }
    fn is_not_null(self) -> bool { self != Self::NULL }

    /// Set the value to `Self::NULL`
    fn reset(&mut self) { *self = Self::NULL; }

    const NULL: Self;

    #[inline(always)]
    fn typed<T>(self) -> GenIDOf<T,Gen> { GenIDOf::from(self.index_and_generation()) }
    #[inline(always)]
    fn untyped(self) -> UntypedGenIDOf<Gen> { UntypedGenIDOf::from(self.index_and_generation()) }
}

pub trait IGeneration:
    Eq
    + Hash
    + Ord
    + Increment
    + Decrement
    + OverflowBehavior
    + Debug
    + MaxValue
    + MinValue
    + Zero
    + Copy
    + 'static
{
}
impl<T> IGeneration for T where
    T: Eq
        + Hash
        + Ord
        + Increment
        + Decrement
        + OverflowBehavior
        + Debug
        + MaxValue
        + MinValue
        + Zero
        + Copy
        + 'static
{
}

// Todo: Make a flag for the generation
pub type Generation = u32;
pub type GenID<T> = GenIDOf<T,Generation>;
pub type UntypedGenID = UntypedGenIDOf<Generation>;



pub(crate) struct GenerationDebug<T>(pub(crate) T);
impl<T> Debug for GenerationDebug<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "#{:?}", self.0) }
}




#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct UntypedGenIDOf<Gen: IGeneration = Generation>
{
    index: usize,
    generation: Gen,
}
impl<Gen: IGeneration> Default for UntypedGenIDOf<Gen>
{
    fn default() -> Self { Self::NULL }
}
impl<Gen: IGeneration> IGenID<Gen> for UntypedGenIDOf<Gen>
{
    fn index(self) -> usize { self.index }
    fn generation(self) -> Gen { self.generation }

    const NULL: Self = Self{ index: usize::MAX, generation: Gen::MIN };
}
impl<Gen: IGeneration> UntypedGenIDOf<Gen>
{
    #[inline(always)]
    pub const fn from_index_and_generation(index: usize, generation: Gen) -> Self
    {
        Self { index, generation }
    }

    #[inline(always)]
    pub const fn index(self) -> usize { self.index }
    #[inline(always)]
    pub const fn generation(self) -> Gen { self.generation }
}
impl<Gen: IGeneration, C> IndexExtension<C> for UntypedGenIDOf<Gen> {}

impl<Gen: IGeneration> From<(usize, Gen)> for UntypedGenIDOf<Gen>
{
    fn from((index, generation): (usize, Gen)) -> Self
    {
        Self::from_index_and_generation(index, generation)
    }
}
impl<Gen: IGeneration> From<UntypedGenIDOf<Gen>> for (usize, Gen)
{
    fn from(value: UntypedGenIDOf<Gen>) -> Self { (value.index, value.generation) }
}
impl<T,Gen: IGeneration> From<GenIDOf<T,Gen>> for UntypedGenIDOf<Gen>
{
    fn from(value: GenIDOf<T,Gen>) -> Self {
        Self::from_index_and_generation(value.index(), value.generation())
    }
}
impl<T,Gen: IGeneration> From<UntypedGenIDOf<Gen>> for GenIDOf<T,Gen>
{
    fn from(value: UntypedGenIDOf<Gen>) -> Self {
        Self::from_index_and_generation(value.index, value.generation)
    }
}


#[cfg(feature = "serde")]
impl<Gen: IGeneration> Serialize for UntypedGenIDOf<Gen>
where
    Gen: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.index.is_max()
        {
            None
        }
        else
        {
            Some((self.index, self.generation))
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, Gen: IGeneration> Deserialize<'de> for UntypedGenIDOf<Gen>
where
    Gen: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::deserialize(deserializer)?
        {
            Some((index, generation)) => Ok(Self::from_index_and_generation(index, generation)),
            None => Ok(Self::from_index_and_generation(usize::MAX, Gen::MIN)),
        }
    }
}
impl<Gen: IGeneration> Debug for UntypedGenIDOf<Gen>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.is_null()
        {
            write!(f, "NULL")
        }
        else
        {
            f.debug_tuple("")
                .field(&self.index())
                .field(&GenerationDebug(self.generation()))
                .finish()
        }
    }
}






pub struct GenIDOf<T,Gen: IGeneration = Generation>
{
    untyped : UntypedGenIDOf<Gen>,
    phantom: PhantomData<T>,
}
impl<T,Gen: IGeneration> Clone for GenIDOf<T,Gen>
{
    fn clone(&self) -> Self {
        Self { untyped: self.untyped.clone(), phantom: PhantomData }
    }
}
impl<T,Gen: IGeneration> Copy for GenIDOf<T,Gen> {}
impl<T,Gen: IGeneration> PartialEq for GenIDOf<T,Gen>
{
    fn eq(&self, other: &Self) -> bool {
        self.untyped == other.untyped
    }
}
impl<T,Gen: IGeneration> Eq for GenIDOf<T,Gen> {}
impl<T,Gen: IGeneration> Ord for GenIDOf<T,Gen>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.untyped.cmp(&other.untyped)
    }
}
impl<T,Gen: IGeneration> PartialOrd for GenIDOf<T,Gen>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.untyped.partial_cmp(&other.untyped)
    }
}
impl<T,Gen: IGeneration> Hash for GenIDOf<T,Gen>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.untyped.hash(state);
    }
}
impl<T,Gen: IGeneration> Default for GenIDOf<T,Gen>
{
    fn default() -> Self { Self::NULL }
}
impl<T,Gen: IGeneration> IGenID<Gen> for GenIDOf<T,Gen>
{
    #[inline(always)]
    fn index(self) -> usize { self.untyped.index() }
    #[inline(always)]
    fn generation(self) -> Gen { self.untyped.generation() }

    const NULL: Self = Self { untyped: UntypedGenIDOf::<Gen>::NULL, phantom: PhantomData };
}
impl<T,Gen: IGeneration> GenIDOf<T,Gen>
{
    #[inline(always)]
    pub const fn from_index_and_generation(index: usize, generation: Gen) -> Self
    {
        Self { untyped: UntypedGenIDOf::<Gen>::from_index_and_generation(index, generation), phantom: PhantomData }
    }
    #[inline(always)]

    pub const fn index(self) -> usize { self.untyped.index() }
    #[inline(always)]
    pub const fn generation(self) -> Gen { self.untyped.generation() }
}
impl<T,Gen: IGeneration, C> IndexExtension<C> for GenIDOf<T,Gen> {}

impl<T,Gen: IGeneration> From<(usize, Gen)> for GenIDOf<T,Gen>
{
    fn from((index, generation): (usize, Gen)) -> Self
    {
        Self::from_index_and_generation(index, generation)
    }
}
impl<T,Gen: IGeneration> From<GenIDOf<T,Gen>> for (usize, Gen)
{
    fn from(value: GenIDOf<T,Gen>) -> Self { (value.index(), value.generation()) }
}


#[cfg(feature = "serde")]
impl<T, Gen: IGeneration> Serialize for GenIDOf<T, Gen>
where
    Gen: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.untyped.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, Gen: IGeneration> Deserialize<'de> for GenIDOf<T,Gen>
where
    Gen: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(UntypedGenIDOf::deserialize(deserializer)?))
    }
}
impl<T, Gen: IGeneration> Debug for GenIDOf<T,Gen>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.is_null()
        {
            write!(f, "NULL")
        }
        else
        {
            f.debug_tuple("")
                .field(&self.index())
                .field(&GenerationDebug(self.generation()))
                .finish()
        }
    }
}