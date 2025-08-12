use super::*;

// todo : lerp and easing using Time + Dt
// todo : lerp using itself (for independant coef)
pub trait Lerpable : Sized
{
    /// Restricted between `[0..1]`
    fn lerp(self, dest : Self, coef : float) -> Self { self.lerp_unchecked(dest, coef.clamp(0., 1.)) }
    /// Not restricted between `[0..1]`
    fn lerp_unchecked(self, dest : Self, coef : float) -> Self;
}
impl<T> Lerpable for T where T: Mul<float,Output=Self> + Add<Self,Output=Self> + Sized
{
    fn lerp_unchecked(self, dest : Self, coef : float) -> Self { self * (1. - coef) + dest * coef  }
}

