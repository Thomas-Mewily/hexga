use super::*;

pub mod prelude
{
    pub use super::{Integrate, LerpDt};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{SmoothLinear, SmoothPow, SmoothExp};
    pub use super::{AddDt,MulDt,DecayDt};
}

/// Frame rate independent time integration for rate `a += x`.
pub trait SmoothLinear<Dt>
{
    //type Output;

    /// Frame rate independent time integration: replaces `a += x` with a dt scaled step.
    ///
    /// ```ignore
    /// a += x; // not time integrated
    /// =>
    /// a += (x * dt); // time integrated
    /// // replace it by
    /// a += x.smooth_linear(dt);
    /// ```
    fn smooth_linear(self, dt: Dt) -> Self;
}

impl<T> SmoothLinear<T> for T
where
    T: Mul<Output = Self>,
{
    #[inline]
    fn smooth_linear(self, dt: T) -> Self {
        self * dt
    }
}

/// Frame rate independent time integration for a per-second multiplicative coefficient `a *= x`.
pub trait SmoothPow<Dt>
{
    //type Output;

    /// Frame rate independent time integration: replaces `a *= x` with a dt scaled step.
    ///
    /// ```ignore
    /// a *= x; // not time integrated
    /// =>
    /// a *= x.pow(dt); // time integrated
    /// // replace it by
    /// a *= x.smooth_pow(dt);
    /// ```
    ///
    /// `dt=0 -> 1`, `dt=1 -> x`, and `f(a) * f(b) = f(a+b)`.
    fn smooth_pow(self, dt: Dt) -> Self;
}

impl<T> SmoothPow<T> for T
where
    T: Pow<T, Output = T>,
{
    #[inline]
    fn smooth_pow(self, dt: T) -> Self {
        self.pow(dt)
    }
}

/// Frame rate independent time integration for a per-second decay rate `a *= 1.0 - r`.
pub trait SmoothExp<Dt>
{
    //type Output;

    /// Frame rate independent time integration: replaces `a -= a * r * dt` with a dt scaled step.
    ///
    /// ```ignore
    /// a *= 1.0 - r; // not time integrated
    /// =>
    /// a *= 1.0 - (r * dt).neg().exp(); // time integrated
    /// // replace it by
    /// a *= 1.0 - r.smooth_exp(dt);
    /// ```
    ///
    /// Returns the **fraction decayed** over `dt`: `dt=0 -> 0`, and it composes as `1-(1-f(a))(1-f(b)) = f(a+b)`.
    fn smooth_exp(self, dt: Dt) -> Self;
}

impl<T> SmoothExp<T> for T
where
    T: Float,
{
    #[inline]
    fn smooth_exp(self, dt: T) -> Self {
        Self::ONE - (self * dt).neg().exp() // 1 - exp(-self * dt)
    }
}


/// Frame-rate independent time integration of an additive rate: `a += rate`.
pub trait AddDt<Dt> {
    /// Applies `rate` over `dt` seconds: `a += rate * dt`.
    fn add_dt(self, rate: Self, dt: Dt) -> Self;
}

impl<T, Dt> AddDt<Dt> for T
where
    T: Add<Output = Self> + SmoothLinear<Dt>,
{
    #[inline]
    fn add_dt(self, rate: Self, dt: Dt) -> Self {
        self + rate.smooth_linear(dt)
    }
}

/// Frame-rate independent time integration of a multiplicative factor: `a *= factor`.
pub trait MulDt<Dt> {
    /// Applies `factor` (a per-second multiplier) over `dt` seconds: `a *= factor^dt`.
    fn mul_dt(self, factor: Self, dt: Dt) -> Self;
}

impl<T, Dt> MulDt<Dt> for T
where
    T: Mul<Output = Self> + SmoothPow<Dt>,
{
    #[inline]
    fn mul_dt(self, factor: Self, dt: Dt) -> Self {
        self * factor.smooth_pow(dt)
    }
}

/// Frame-rate independent time integration of a decay: `a *= 1 - rate`.
pub trait DecayDt<Dt> {
    /// Applies a per-second decay `rate` over `dt` seconds: `a *= 1 - (1 - exp(-rate*dt))`.
    fn decay_dt(self, rate: Self, dt: Dt) -> Self;
}

impl<T, Dt> DecayDt<Dt> for T
where
    T: Mul<Output = Self> + Sub<Output = Self> + Float + SmoothExp<Dt>,
{
    #[inline]
    fn decay_dt(self, rate: Self, dt: Dt) -> Self {
        self * (T::ONE - rate.smooth_exp(dt))
    }
}



/// A (value, dt), so that `a += x`, `a *= x` can be written as `a += x * dt` / `a *= x * dt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Integrate<V,Time>(pub V, pub Time);

/*
// TODO impl it
impl<Time, V, V2, C> Lerp<C, V2> for  Integrate<V, Time>
    where V: Lerp<C, V2, Output=V>, C: SmoothCoef<Time>
{
    type Output = V;
    fn lerp_unclamped(self, dest: V2, coef: C) -> Self::Output {

    }
}
*/

// https://youtu.be/LSNQuFEDOyQ *Lerp smoothing is broken* by Freya Holmér

/*
impl<Time, V, V2, C> Lerp<C, V2> for Integrate<V, Time>
    where V: Lerp<C, V2, Output=V>, C: SmoothCoef<Time>
{
    type Output = V;

    fn lerp_unclamped(self, dest: V2, coef: C) -> Self::Output {
        let Integrate(value, dt) = self;
        value.lerp_unclamped(dest, coef.integrate_coef(dt))
    }
}
*/


impl<F, V, V2> Lerp<F, V2> for Integrate<&mut V, F>
    where V: Lerp<F, V2, Output=V> + Copy, F: Float
{
    type Output = ();
    fn lerp_unclamped(self, dest: V2, coef: F) -> Self::Output {
        let Integrate(value, dt) = self;
        *value = (*value).lerp_unclamped(dest, coef.smooth_exp(dt));
        ()
    }
}

/// Frame rate independent linear interpolation, with the coefficient integrated over `dt` before interpolating.
///
/// The interpolation coefficient is integrated over `dt` before interpolation.
pub trait LerpDt<C, D=Self, Dt=C> : Sized
{
    type Output;

    /// `Self` in 1 second move closer to `coef_per_unit` % to `dest` (or whatever unit you are using than second).
    /// 
    /// `coef_per_unit` is clamped between `0.0..=1.0`.
    fn lerp_dt(self, dest: D, coef_per_unit: C, dt: Dt) -> Self::Output
    where
        C: ClampUnit,
    {
        self.lerp_dt_unchecked(dest, coef_per_unit.clamp_unit(), dt)
    }

    /// `Self` in 1 second move closer to `coef_per_unit` % to `dest` (or whatever unit you are using than second).
    ///
    /// `coef_per_unit` is not clamped.
    fn lerp_dt_unchecked(self, dest: D, coef_per_unit: C, dt: Dt) -> Self::Output;
}

impl<F, V, V2> LerpDt<F, V2, F> for &mut V
where
    V: Lerp<F, V2, Output=V> + Copy,
    F: Float
{
    type Output = ();

    fn lerp_dt_unchecked(self, dest: V2, coef: F, dt: F) {
        Integrate(self, dt).lerp_unclamped(dest, coef)
    }
}


impl<Time, V> AddAssign<V> for Integrate<&mut V, Time>
where
    V: SmoothLinear<Time> + AddAssign<V>, Time: Copy
{
    fn add_assign(&mut self, rhs: V) {
        *self.0 += rhs.smooth_linear(self.1);
    }
}

impl<Time, V> MulAssign<V> for Integrate<&mut V, Time>
where
    V: SmoothExp<Time> + MulAssign<V>,
    Time: Copy,
{
    fn mul_assign(&mut self, rhs: V) {
        *self.0 *= rhs.smooth_exp(self.1);
    }
}



/*
pub trait IntegrateLerp<F>
{
    fn integrate_rate(self, dt: DurationOf<F>) -> Self;
}
*/


/*
let mut pos = 42.;

Integrate(&mut pos, dt.s()) += 2;
pos += 2.integrate_rate(dt.s());

Integrate(&mut pos, dt.s()) *= 4;
pos *= 4.integrate_coef(dt.s());


integrate(&mut pos, dt.s()) + 4
*/
