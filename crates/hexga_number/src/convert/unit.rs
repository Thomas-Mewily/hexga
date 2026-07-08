use super::*;

/// To construct basic unit from their inner precision type.
pub trait Unit:
    Additive
    + Mul<Self::Precision, Output = Self>
    + MulAssign<Self::Precision>
    + Div<Self::Precision, Output = Self>
    + DivAssign<Self::Precision>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + PartialEq
{
    type Precision: Number + PrimitiveType + OverflowBehavior;
    /// Return the inner value.
    /// This expose how the inner value is stored, but it's impl details and it may change.
    #[doc(hidden)]
    fn inner_value(self) -> Self::Precision;

    /// Create from the inner value.
    /// This expose how the inner value is stored, but it's impl details and it may change.
    #[doc(hidden)]
    fn from_inner_value(inner_value: Self::Precision) -> Self;
}

map_on_number!(
    ($type_name : ident) =>
    {
        impl Unit for $type_name
        {
            type Precision = Self;
            fn inner_value(self) -> $type_name { self }
            fn from_inner_value(inner_value: $type_name) -> Self { inner_value }
        }
    }
);

// TODO: re enable
/*
pub struct UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
{
    pub it: It,
    phantom: PhantomData<(U, Precision)>,
}
impl<U, Precision, It> UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
{
    pub const fn new(it: It) -> Self { Self { it, phantom: PhantomData } }
}

impl<U, Precision, It> Debug for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", &self.it) }
}

impl<U, Precision, It> Copy for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: Copy,
{
}
impl<U, Precision, It> Clone for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: Clone,
{
    fn clone(&self) -> Self
    {
        Self {
            it: self.it.clone(),
            phantom: PhantomData,
        }
    }
}

impl<U, Precision, It> PartialEq for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: PartialEq,
{
    fn eq(&self, other: &Self) -> bool { PartialEq::eq(&self, &other) }
}
impl<U, Precision, It> Eq for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: Eq,
{
}

impl<U, Precision, It> PartialOrd for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { PartialOrd::partial_cmp(&self.it, &other.it) }
}
impl<U, Precision, It> Ord for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering { Ord::cmp(&self.it, &other.it) }
}

impl<U, Precision, It> Hash for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.it.hash(state); }
}

impl<U, Precision, It> Iterator for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> { self.it.next().map(|v| unsafe { U::from_inner_value(v) }) }
}

impl<U, Precision, It> DoubleEndedIterator for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> { self.it.next_back().map(|v| unsafe { U::from_inner_value(v) }) }
}

impl<U, Precision, It> FusedIterator for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: FusedIterator,
{
}
impl<U, Precision, It> ExactSizeIterator for UnitIterator<U, Precision, It>
where
    It: Iterator<Item = Precision>,
    U: Unit<Precision>,
    It: ExactSizeIterator,
{
}
*/
