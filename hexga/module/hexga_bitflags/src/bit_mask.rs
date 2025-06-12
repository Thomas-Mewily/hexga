use hexga_number::*;

pub trait BitMasksExtension : NumberIntegerUnsigned
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
impl<T> BitMasksExtension for T where T: NumberIntegerUnsigned { }


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


impl<T, F> BitFlagsExtension<F> for T where T: IntegerUnsigned, T : From<F>
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
