use super::*;

/// Constant Policy Value
pub trait Constant<T>
{
    const CONSTANT: T;
}

pub struct ConstantZero<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantZero<T>
where
    T: Zero,
{
    const CONSTANT: T = T::ZERO;
}
impl<T> std::fmt::Debug for ConstantZero<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Zero") }
}

pub struct ConstantOne<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantOne<T>
where
    T: One,
{
    const CONSTANT: T = T::ONE;
}
impl<T> std::fmt::Debug for ConstantOne<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "One") }
}

pub struct ConstantMinusOne<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantMinusOne<T>
where
    T: MinusOne,
{
    const CONSTANT: T = T::MINUS_ONE;
}
impl<T> std::fmt::Debug for ConstantMinusOne<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "MinusOne") }
}

pub struct ConstantHalf<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantHalf<T>
where
    T: Half,
{
    const CONSTANT: T = T::HALF;
}
impl<T> std::fmt::Debug for ConstantHalf<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Half") }
}

pub struct ConstantNaN<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantNaN<T>
where
    T: NaNValue,
{
    const CONSTANT: T = T::NAN;
}
impl<T> std::fmt::Debug for ConstantNaN<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "NaN") }
}

pub struct ConstantMin<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantMin<T>
where
    T: MinValue,
{
    const CONSTANT: T = T::MIN;
}
impl<T> std::fmt::Debug for ConstantMin<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Min") }
}

pub struct ConstantMax<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantMax<T>
where
    T: MaxValue,
{
    const CONSTANT: T = T::MAX;
}
impl<T> std::fmt::Debug for ConstantMax<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Max") }
}

pub struct ConstantInfinity<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantInfinity<T>
where
    T: Infinity,
{
    const CONSTANT: T = T::INFINITY;
}
impl<T> std::fmt::Debug for ConstantInfinity<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Infinity") }
}

pub struct ConstantMinusInfinity<T>
{
    phantom: PhantomData<T>,
}
impl<T> Constant<T> for ConstantMinusInfinity<T>
where
    T: MinusInfinity,
{
    const CONSTANT: T = T::MINUS_INFINITY;
}
impl<T> std::fmt::Debug for ConstantMinusInfinity<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "MinusInfinity") }
}
