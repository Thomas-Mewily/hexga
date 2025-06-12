use crate::*;
use std::{fmt, marker::PhantomData, ops::*};
use hexga_number::*;
use std::fmt::{Debug,Formatter,Result as DResult};


pub trait BitFlagsIntegerUnsigned : NumberIntegerUnsigned + fmt::Binary {}
impl<T> BitFlagsIntegerUnsigned for T where T: NumberIntegerUnsigned + fmt::Binary {}

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    flags : U,
    phantom : PhantomData<F>,
}
impl<F,U> Default for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    fn default() -> Self { Self::new() }
}

// Todo: Better serialization ? Vector of F ? but without allocation / PartialArray ?
#[cfg(feature = "serde")]
impl<F,U> Serialize for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F> + Serialize, F : MaxValue + Copy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        self.flags.serialize(serializer)
    }
}


#[cfg(feature = "serde")]
impl<'de,F,U> Deserialize<'de> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F> + Deserialize<'de>, F : MaxValue + Copy
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = U::deserialize(deserializer)?;
        Self::try_from_flags(flags).ok_or_else(|| serde::de::Error::custom(format!("invalid value: {:#b}, expected {:#b}", flags, Self::from_flags(flags).flags())))
    }
}


impl<F,U> Debug for BoolFlags<F,U>
where
    U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U>,
    F : Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult
    {
        let mut it = self.into_iter().peekable();
        write!(f, "[")?;
        while let Some(v) = it.next()
        {
            write!(f, "{:?}", v)?;
            if it.peek().is_some()
            {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<F,U> From<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    fn from(value: F) -> Self { Self::from_flag(value) }
}
impl<F,U> Zero for BoolFlags<F,U>  where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    const ZERO : Self = unsafe { Self::from_flags_unchecked(U::ZERO) };
}
impl<F,U> BoolFlags<F,U>  where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    pub const unsafe fn from_flags_unchecked(flags : U) -> Self { Self { flags, phantom: PhantomData }}

    /// Return None if the flags contains unknow toggled bit (greater than `F::MAX`)
    pub fn try_from_flags(flags : U) -> Option<Self>
    {
        let s = Self::from_flags(flags);
        if s.flags == flags { Some(s) } else { None }
    }
    pub fn from_flags(flags : U) -> Self { let max : U = U::ONE << F::MAX.into(); unsafe { Self::from_flags_unchecked(flags & (max | max - U::ONE)) } }
    //pub fn from_flags(flag : U) -> Self { Self::_from_flags(U::ONE << flag)}
    pub fn from_flag(flag : F) -> Self { Self::from_flags(U::ONE << flag.into()) }

    pub fn new() -> Self { Self::ZERO }

    pub fn flags(self) -> U { self.flags }
}

impl<F,U> BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    /// Set the flag using a bool
    #[must_use]
    pub fn with   (mut self, flag : F, set : bool) -> Self { self.set(flag, set); self }
    /// Set the flag to 1, even if the flag was already 1
    #[must_use]
    pub fn added  (mut self, flag : F) -> Self { self.add(flag); self }
    /// Set the flag to 0, even if the flag was already 0
    #[must_use]
    pub fn removed(mut self, flag : F) -> Self { self.remove(flag); self }
    /// Flip the flag transforming `1 <=> 0`
    #[must_use]
    pub fn toggled(mut self, flag : F) -> Self { self.toggle(flag); self }

    /// Set the flag using a bool
    pub fn set   (&mut self, flag : F, set : bool) -> &mut Self { self.flags.mask(U::ONE << flag.into(),set); self }
    /// Set the flag to 1, even if the flag was already 1
    pub fn add   (&mut self, flag : F) -> &mut Self { self.flags.mask_set_one(U::ONE << flag.into()); self }
    /// Set the flag to 0, even if the flag was already 0
    pub fn remove(&mut self, flag : F) -> &mut Self { self.flags.mask_set_zero(U::ONE << flag.into()); self }
    /// Flip the flag transforming `1 <=> 0`
    pub fn toggle(&mut self, flag : F) -> &mut Self { self.flags.mask_toggle(U::ONE << flag.into()); self }

    /// Check if a flag is present
    pub fn have(&self, flag : F) -> bool { self.flags.mask_any_one(U::ONE << flag.into()) }

    pub fn have_any(&self, flags : &[F]) -> bool { flags.iter().any(|f| self.have(*f)) }
    pub fn have_all(&self, flags : &[F]) -> bool { flags.iter().all(|f| self.have(*f)) }
}

impl<F,U> IntoIterator for BoolFlags<F,U>
    where
    U : BitFlagsIntegerUnsigned + From<F>,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U>,
{
    type Item=F;

    type IntoIter=BoolFlagsIntoIter<F,U>;
    fn into_iter(self) -> Self::IntoIter { BoolFlagsIntoIter::new(self) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoolFlagsIntoIter<F,U>
    where
    U : BitFlagsIntegerUnsigned + From<F>,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U>
{
    flags : U,
    idx : U,
    #[serde(skip)]
    phantom : PhantomData<F>,
}
impl<F,U> BoolFlagsIntoIter<F,U>
    where
    U : BitFlagsIntegerUnsigned + From<F>,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U>
{
    pub fn new_start_at(flags : BoolFlags<F,U>, idx : U) -> Self { Self { idx, flags : flags.flags, phantom : PhantomData }}
    pub fn new(flags : BoolFlags<F,U>) -> Self { Self::new_start_at(flags, U::ZERO) }
}

impl<F,U> Iterator for BoolFlagsIntoIter<F,U>
    where
    U : BitFlagsIntegerUnsigned + From<F>,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U>
{
    type Item=F;
    fn next(&mut self) -> Option<Self::Item>
    {
        while self.flags.is_non_zero()
        {
            let f = self.flags & U::ONE == U::ONE;
            let flag_idx = self.idx;

            self.flags >>= U::ONE;
            self.idx += U::ONE;

            if f
            {
                return Some(F::try_from(flag_idx).unwrap()) // Since we assume the exact size
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let nb_ones = self.flags.count_bit_ones() as _;
        (nb_ones, Some(nb_ones))
    }
}

impl<F,U> std::iter::FusedIterator for BoolFlagsIntoIter<F,U>
    where
    U : BitFlagsIntegerUnsigned + From<F>,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U> {}

impl<F,U> std::iter::ExactSizeIterator for BoolFlagsIntoIter<F,U>
    where
    U : BitFlagsIntegerUnsigned + From<F>,
    F : MaxValue + Copy + TryFrom<U>,
    <F as TryFrom<U>>::Error : Debug,
    RangeInclusive<U>: Iterator<Item = U>
{
    fn len(&self) -> usize
    {
        self.flags.count_bit_ones() as _
    }
}


impl<F,U> BitAnd<Self> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitand(self, rhs: Self) -> Self::Output { Self::from_flags(self.flags.bitand(rhs.flags)) } }
impl<F,U> BitAndAssign<Self> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitand_assign(&mut self, rhs: Self) { self.flags.bitand_assign(rhs.flags); } }
impl<F,U> BitAnd<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitand(self, rhs: F) -> Self::Output { self.bitand(Self::from(rhs)) } }
impl<F,U> BitAndAssign<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitand_assign(&mut self, rhs: F) { self.bitand_assign(Self::from(rhs)); } }

impl<F,U> BitOr<Self> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitor(self, rhs: Self) -> Self::Output { Self::from_flags(self.flags.bitor(rhs.flags)) } }
impl<F,U> BitOrAssign<Self> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitor_assign(&mut self, rhs: Self) { self.flags.bitor_assign(rhs.flags); } }
impl<F,U> BitOr<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitor(self, rhs: F) -> Self::Output { self.bitor(Self::from(rhs)) } }
impl<F,U> BitOrAssign<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitor_assign(&mut self, rhs: F) { self.bitor_assign(Self::from(rhs)); } }

impl<F,U> BitXor<Self> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitxor(self, rhs: Self) -> Self::Output { Self::from_flags(self.flags.bitxor(rhs.flags)) } }
impl<F,U> BitXorAssign<Self> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitxor_assign(&mut self, rhs: Self) { self.flags.bitxor_assign(rhs.flags); } }
impl<F,U> BitXor<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitxor(self, rhs: F) -> Self::Output { self.bitxor(Self::from(rhs)) } }
impl<F,U> BitXorAssign<F> for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitxor_assign(&mut self, rhs: F) { self.bitxor_assign(Self::from(rhs)); } }

impl<F,U> Not for BoolFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    type Output=Self;
    fn not(self) -> Self::Output  { Self::from_flags(!self.flags) }
}


#[cfg(test)]
mod bool_flag_test
{
    use super::*;

    type Team = BoolFlags<TeamFlag, u8>;

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum TeamFlag
    {
        Blue,
        Red,
        Yellow,
    }
    impl MaxValue for TeamFlag
    {
        const MAX : Self = Self::Yellow;
    }

    impl From<TeamFlag> for u8
    {
        fn from(value: TeamFlag) -> Self {
            value as u8
        }
    }

    // There are some better way to create/generate the TryFrom for an enum with an rep
    impl TryFrom<u8> for TeamFlag
    {
        type Error=();
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            Ok
            (
                match value
                {
                    0 => TeamFlag::Blue,
                    1 => TeamFlag::Red,
                    2 => TeamFlag::Yellow,
                    _ => return Err(())
                }
            )
        }
    }

    #[test]
    fn old_version()
    {
        /*
        let mut t = Team::ZERO.flagged(TeamFlag::Blue).flagged(TeamFlag::Red);

        assert!(t.have_flag(TeamFlag::Blue));
        assert!(t.have_flag(TeamFlag::Red));
        assert!(t.have_flag(TeamFlag::Yellow).not());

        t.toggle_flag(TeamFlag::Blue);
        assert!(t.have_flag(TeamFlag::Blue).not());
        assert!(t.have_flag(TeamFlag::Red));
        assert!(t.have_flag(TeamFlag::Yellow).not());

        t.remove_flag(TeamFlag::Red);
        assert!(t.have_flag(TeamFlag::Red).not());

        t.toggle_flag(TeamFlag::Red).flag(TeamFlag::Yellow);
        assert!(t.have_flag(TeamFlag::Red));
        assert!(t.have_flag(TeamFlag::Yellow));
        */
    }



    #[test]
    fn new_version()
    {
        let mut t1 = Team::ZERO.added(TeamFlag::Blue).added(TeamFlag::Red);

        assert!(t1.have(TeamFlag::Blue));
        assert!(t1.have(TeamFlag::Red));
        assert!(t1.have(TeamFlag::Yellow).not());

        t1.toggle(TeamFlag::Blue);
        assert!(t1.have(TeamFlag::Blue).not());
    }


    #[test]
    fn new_version_some_op()
    {
        let mut t1 = Team::ZERO | TeamFlag::Blue | TeamFlag::Red;

        t1 ^= TeamFlag::Red;

        assert!(t1.have(TeamFlag::Blue));

        assert!(t1.have(TeamFlag::Red).not());
        assert!(t1.have(TeamFlag::Yellow).not());

        t1 |= TeamFlag::Yellow;
        assert!(t1.have(TeamFlag::Yellow));
    }


    #[test]
    fn rep()
    {
        let t1 = Team::ZERO | TeamFlag::Blue | TeamFlag::Red;

        assert_eq!(t1.flags(), 0b0000_0011);

        let t1 = !t1;
        assert_eq!(t1.flags(), 0b0000_0100); // because TeamFlag only use the last 3 bits
    }


    #[test]
    fn iteration()
    {
        let mut t1 = Team::new(); // same as Team::ZERO;
        let blue_and_yellow = [TeamFlag::Blue, TeamFlag::Yellow];

        for flag in blue_and_yellow
        {
            t1 |= flag;
        }

        assert!(t1.have_all(&blue_and_yellow));
        assert!(t1.have(TeamFlag::Red).not());

        assert_eq!(t1.into_iter().count(), blue_and_yellow.len());

        for (f1, f2) in t1.into_iter().zip(blue_and_yellow.iter().copied())
        {
            assert_eq!(f1, f2);
        }
    }
}