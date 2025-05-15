use crate::*;
use std::{fmt, marker::PhantomData, ops::*};
use hexga_number::*;
use std::fmt::{Debug,Formatter,Result as DResult};

pub trait BitMasksExtension : IntegerUnsigned
{
    #[must_use]
    fn masked          (self, mask : Self, set : bool) -> Self { if set { self.masked_set_one(mask) } else { self.masked_set_zero(mask) } }
    #[must_use]
    fn masked_set_one  (self, mask : Self) -> Self { self | mask  }
    #[must_use]
    fn masked_set_zero (self, mask : Self) -> Self { self & !mask }
    #[must_use]
    fn masked_toggle   (self, mask : Self) -> Self { self ^ mask  }
    
    fn mask            (&mut self, mask : Self, set : bool) -> &mut Self { *self = self.masked(mask, set); self }
    fn mask_set_one    (&mut self, mask : Self) -> &mut Self { *self = self.masked_set_one(mask); self }
    fn mask_set_zero   (&mut self, mask : Self) -> &mut Self { *self = self.masked_set_zero(mask); self }
    fn mask_toggle     (&mut self, mask : Self) -> &mut Self { *self = self.masked_toggle(mask); self }

    fn mask_all_true   (&self, mask : Self) -> bool { let m = mask; (*self & m) == m }
    fn mask_any_one    (&self, mask : Self) -> bool { (*self & mask).is_non_zero() }
    fn mask_any_zero   (&self, mask : Self) -> bool { !self.mask_any_one(mask) }
}
impl<T> BitMasksExtension for T where T : IntegerUnsigned { }


/*
pub trait BitFlagsExtension<F> : Sized + From<F>
{
    #[must_use]
    /// Set the flag using a bool
    fn with_flag       (mut self, bit_idx : F, set : bool) -> Self { self.set_flag(bit_idx, set); self }
    #[must_use]
    /// Set the flag to 1, even if the flag was already 1
    fn flagged         (mut self, bit_idx : F) -> Self { self.flag(bit_idx); self }
    #[must_use]
    /// Set the flag to 0, even if the flag was already 0
    fn removed_flag    (mut self, bit_idx : F) -> Self { self.remove_flag(bit_idx); self }
    #[must_use]
    /// Flip the flag transforming `1 <=> 0`
    fn toggled_flag    (mut self, bit_idx : F) -> Self { self.toggle_flag(bit_idx); self }

    /// Set the flag using a bool
    fn set_flag        (&mut self, bit_idx : F, set : bool) -> &mut Self;
    /// Set the flag to 1, even if the flag was already 1
    fn flag            (&mut self, bit_idx : F) -> &mut Self;
    /// Set the flag to 0, even if the flag was already 0
    fn remove_flag     (&mut self, bit_idx : F) -> &mut Self;
    /// Flip the flag transforming `1 <=> 0`
    fn toggle_flag     (&mut self, bit_idx : F) -> &mut Self;

    /// Check if a flag is present
    fn have_flag       (self, bit_idx : F) -> bool;
}

 
impl<T, F> BitFlagsExtension<F> for T where T : IntegerUnsigned, T : From<F> 
{
    fn with_flag       (self, bit_idx : F, set : bool) -> Self { self.masked(Self::ONE << bit_idx.into(),set) } 
    fn flagged         (self, bit_idx : F) -> Self { self.masked_set_one(Self::ONE << bit_idx.into()) }
    fn removed_flag    (self, bit_idx : F) -> Self { self.masked_set_zero(Self::ONE << bit_idx.into()) }
    fn toggled_flag    (self, bit_idx : F) -> Self { self.masked_toggle(Self::ONE << bit_idx.into()) }

    fn set_flag        (&mut self, bit_idx : F, set : bool) -> &mut Self { self.mask(Self::ONE << bit_idx.into(),set); self } 
    fn flag            (&mut self, bit_idx : F) -> &mut Self { self.mask_set_one(Self::ONE << bit_idx.into()); self }
    fn remove_flag     (&mut self, bit_idx : F) -> &mut Self { self.mask_set_zero(Self::ONE << bit_idx.into()); self }
    fn toggle_flag     (&mut self, bit_idx : F) -> &mut Self { self.mask_toggle(Self::ONE << bit_idx.into()); self }
    fn have_flag       (self, bit_idx : F) -> bool { self.mask_any_one(Self::ONE << bit_idx.into()) }
}
*/

pub trait BitFlagsIntegerUnsigned : IntegerUnsigned + fmt::Binary {}
impl<T> BitFlagsIntegerUnsigned for T where T : IntegerUnsigned + fmt::Binary {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    flags : U,
    phantom : PhantomData<F>,
}

#[cfg(feature = "serde")]
impl<F,U> Serialize for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F> + Serialize, F : MaxValue + Copy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    { 
        self.flags.serialize(serializer)
    }
}


#[cfg(feature = "serde")]
impl<'de,F,U> Deserialize<'de> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F> + Deserialize<'de>, F : MaxValue + Copy
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = U::deserialize(deserializer)?;
        Self::try_from_flags(flags).ok_or_else(|| serde::de::Error::custom(format!("invalid value: {:#b}, expected {:#b}", flags, Self::from_flags(flags).flags())))
    }
}


impl<F,U> Debug for BitFlags<F,U> 
where 
    U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy, 
    F : TryFrom<U> + Debug,
    RangeInclusive<U>: Iterator<Item = U>,
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

impl<F,U> From<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    fn from(value: F) -> Self { Self::from_flag(value) }
}
impl<F,U> Zero for BitFlags<F,U>  where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
{
    const ZERO : Self = unsafe { Self::from_flags_unchecked(U::ZERO) };
}
impl<F,U> BitFlags<F,U>  where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
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

impl<F,U> BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy
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

impl<F,U> IntoIterator for BitFlags<F,U> 
    where 
    U : BitFlagsIntegerUnsigned + From<F>, 
    F : MaxValue + Copy,
    F : TryFrom<U>, RangeInclusive<U>: Iterator<Item = U>
{
    type Item=F;

    type IntoIter=BitFlagsIntoIter<F,U>;
    fn into_iter(self) -> Self::IntoIter { BitFlagsIntoIter::new(self) }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitFlagsIntoIter<F,U> 
    where 
    U : BitFlagsIntegerUnsigned + From<F>, 
    F : MaxValue + Copy + TryFrom<U>, 
    RangeInclusive<U>: Iterator<Item = U>
{
    flags : U,
    flag_offset : U,
    phantom : PhantomData<F>,
}
impl<F,U> BitFlagsIntoIter<F,U>
    where 
    U : BitFlagsIntegerUnsigned + From<F>, 
    F : MaxValue + Copy + TryFrom<U>, 
    RangeInclusive<U>: Iterator<Item = U>
{
    pub fn new_start_at(bitflag : BitFlags<F,U>, idx : U) -> Self { Self { flag_offset: idx, flags : bitflag.flags, phantom : PhantomData }}
    pub fn new(bitflag : BitFlags<F,U>) -> Self { Self::new_start_at(bitflag, U::ZERO) }
}

impl<F,U> Iterator for BitFlagsIntoIter<F,U>
    where 
    U : BitFlagsIntegerUnsigned + From<F>, 
    F : MaxValue + Copy + TryFrom<U>, 
    RangeInclusive<U>: Iterator<Item = U>
{
    type Item=F;
    fn next(&mut self) -> Option<Self::Item> 
    {
        while self.flags.is_non_zero()
        {
            let f = self.flags & U::ONE == U::ONE;
            let flag_idx = self.flag_offset;

            self.flags >>= U::ONE;
            self.flag_offset += U::ONE;
            
            if f 
            {
                if let Ok(v) = F::try_from(flag_idx)
                {
                    return Some(v);
                }
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

impl<F,U> std::iter::FusedIterator for BitFlagsIntoIter<F,U>
    where 
    U : BitFlagsIntegerUnsigned + From<F>, 
    F : MaxValue + Copy + TryFrom<U>, 
    RangeInclusive<U>: Iterator<Item = U> {}

impl<F,U> std::iter::ExactSizeIterator for BitFlagsIntoIter<F,U> 
    where 
    U : BitFlagsIntegerUnsigned + From<F>, 
    F : MaxValue + Copy + TryFrom<U>, 
    RangeInclusive<U>: Iterator<Item = U>
{
    fn len(&self) -> usize
    {
        self.flags.count_bit_ones() as _
    }
}


impl<F,U> BitAnd<Self> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitand(self, rhs: Self) -> Self::Output { Self::from_flags(self.flags.bitand(rhs.flags)) } }
impl<F,U> BitAndAssign<Self> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitand_assign(&mut self, rhs: Self) { self.flags.bitand_assign(rhs.flags); } }
impl<F,U> BitAnd<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitand(self, rhs: F) -> Self::Output { self.bitand(Self::from(rhs)) } }
impl<F,U> BitAndAssign<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitand_assign(&mut self, rhs: F) { self.bitand_assign(Self::from(rhs)); } }

impl<F,U> BitOr<Self> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitor(self, rhs: Self) -> Self::Output { Self::from_flags(self.flags.bitor(rhs.flags)) } }
impl<F,U> BitOrAssign<Self> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitor_assign(&mut self, rhs: Self) { self.flags.bitor_assign(rhs.flags); } }
impl<F,U> BitOr<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitor(self, rhs: F) -> Self::Output { self.bitor(Self::from(rhs)) } }
impl<F,U> BitOrAssign<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitor_assign(&mut self, rhs: F) { self.bitor_assign(Self::from(rhs)); } }

impl<F,U> BitXor<Self> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitxor(self, rhs: Self) -> Self::Output { Self::from_flags(self.flags.bitxor(rhs.flags)) } }
impl<F,U> BitXorAssign<Self> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitxor_assign(&mut self, rhs: Self) { self.flags.bitxor_assign(rhs.flags); } }
impl<F,U> BitXor<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { type Output=Self; fn bitxor(self, rhs: F) -> Self::Output { self.bitxor(Self::from(rhs)) } }
impl<F,U> BitXorAssign<F> for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy { fn bitxor_assign(&mut self, rhs: F) { self.bitxor_assign(Self::from(rhs)); } }

impl<F,U> Not for BitFlags<F,U> where U : BitFlagsIntegerUnsigned + From<F>, F : MaxValue + Copy 
{
    type Output=Self;
    fn not(self) -> Self::Output  { Self::from_flags(!self.flags) }
}


#[cfg(test)]
mod big_flag_ex
{
    use super::*;

    type Team = BitFlags<TeamFlag, u8>;

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