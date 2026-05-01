use super::*;
use crate::bijection::*;

#[non_exhaustive]
#[derive(Clone)]
pub enum DenseGridError<Idx, const N: usize>
where
    Idx: Integer,
{
    NegativeSize(Vector<Idx, N>),
    /// (dim, got)
    WrongDimension(Vector<Idx, N>, usize),
    ToBig(Vector<Idx, N>),
}
impl<Idx, const N: usize> Debug for DenseGridError<Idx, N>
where
    Idx: Debug,
    Idx: Integer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self
        {
            DenseGridError::NegativeSize(size) =>
            {
                write!(f, "Can't have a negative size {:?}", size)
            }
            DenseGridError::WrongDimension(size, got) => write!(
                f,
                "Wrong dimension : expected {:?} elements for a {:?} grid but got {:?} elements",
                size.area_usize(),
                size,
                got
            ),
            DenseGridError::ToBig(size) => write!(f, "The size {:?} is too big !", size),
        }
    }
}
impl<Idx, const N: usize> Display for DenseGridError<Idx, N>
where
    Idx: Debug,
    Idx: Integer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { Debug::fmt(self, f) }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
//#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Grid"))]
pub struct DenseGrid<C, B, T, Idx, const N: usize>
where
    C: Deref<Target = [T]>,
    B: BijectionFn<Source = Vector<Idx, N>, Target = usize>,
    Idx: Integer,
{
    pub(crate) bijection: Bijection<C, B>,
    phantom: PhantomData<Idx>,
}

impl<C, B, T, Idx, const N: usize> From<DenseGrid<C, B, T, Idx, N>> for Bijection<C, B>
where
    C: Deref<Target = [T]>,
    B: BijectionFn<Source = Vector<Idx, N>, Target = usize>,
    Idx: Integer,
{
    fn from(value: DenseGrid<C, B, T, Idx, N>) -> Self { value.bijection }
}

pub trait NewDenseGrid<C, B, T, Idx, const N: usize>: Sized
where
    C: Deref<Target = [T]>,
    B: BijectionFn<Source = Vector<Idx, N>, Target = usize>,
    Idx: Integer,
{
    fn from_values<P>(size: P, values: C) -> Option<Self>
    where
        P: Into<Vector<Idx, N>>,
        B: From<Vector<Idx, N>>,
    {
        Self::try_from_values(size, values).ok()
    }

    fn try_from_values<P>(size: P, values: C) -> Result<Self, DenseGridError<Idx, N>>
    where
        P: Into<Vector<Idx, N>>,
        B: From<Vector<Idx, N>>,
    {
        let size = size.into();
        if *size.min_element() < Idx::ZERO
        {
            return Err(DenseGridError::NegativeSize(size));
        }

        let area_size = match size.area_usize_checked()
        {
            Some(v) => v,
            None => return Err(DenseGridError::ToBig(size)),
        };
        if area_size != values.len()
        {
            return Err(DenseGridError::WrongDimension(size, area_size));
        }
        Ok(unsafe {
            Self::from_bijection_unchecked(Bijection::from_values_and_bijection(
                values,
                B::from(size),
            ))
        })
    }

    unsafe fn from_bijection_unchecked(bijection: Bijection<C, B>) -> Self;
}

impl<S, B, T, Idx, const N: usize> NewDenseGrid<S, B, T, Idx, N> for DenseGrid<S, B, T, Idx, N>
where
    S: Deref<Target = [T]>,
    B: BijectionFn<Source = Vector<Idx, N>, Target = usize>,
    Idx: Integer,
{
    unsafe fn from_bijection_unchecked(bijection: Bijection<S, B>) -> Self
    {
        Self {
            bijection,
            phantom: PhantomData,
        }
    }
}
