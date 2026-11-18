use super::*;

pub mod prelude
{
    pub use super::{Integrate, LerpDt};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{SmoothLinear, SmoothPow, SmoothExp};
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

/// A (value, dt), so that `a += x`, `a *= x` can be written as `a += x * dt` / `a *= x * dt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Integrate<V,Time>(pub V, pub Time);

/*
// TODO impl it
impl<Time, V, V2, C> Lerp<C, V2> for  Integrate<V, Time>
    where V: Lerp<C, V2, Output=V>, C: IntegrateCoef<Time>
{
    type Output = V;
    fn lerp_unclamped(self, dest: V2, coef: C) -> Self::Output {

    }
}
*/

// https://youtu.be/LSNQuFEDOyQ *Lerp smoothing is broken* by Freya Holmér

/*
impl<Time, V, V2, C> Lerp<C, V2> for Integrate<V, Time>
    where V: Lerp<C, V2, Output=V>, C: IntegrateCoef<Time>
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

/// Linearly interpolate a mutable value with time integration.
///
/// The interpolation coefficient is integrated over `dt` before interpolation.
pub trait LerpDt<C, D=Self, Dt=C> : Sized
{
    type Output;

    /// Interpolate with the coefficient clamped between `0.0..=1.0` and apply the result to `self`.
    fn lerp_dt(self, dest: D, coef_per_unit: C, dt: Dt) -> Self::Output
    where
        C: ClampUnit,
    {
        self.lerp_dt_unchecked(dest, coef_per_unit.clamp_unit(), dt)
    }

    fn lerp_dt_unchecked(self, dest: D, coef_per_unit: C, dt: Dt) -> Self::Output;
}

impl<Time, V, V2, C> LerpDt<C, V2, Time> for &mut V
where
    V: Lerp<C, V2, Output=V> + Copy,
    C: IntegrateExp<Time>
{
    type Output = ();

    fn lerp_dt_unchecked(self, dest: V2, coef: C, dt: Time) {
        Integrate(self, dt).lerp_unclamped(dest, coef)
    }
}


impl<Time, V> AddAssign<V> for Integrate<&mut V, Time>
where
    V: IntegrateLinear<Time> + AddAssign<V>, Time: Copy
{
    fn add_assign(&mut self, rhs: V) {
        *self.0 += rhs.integrate_linear(self.1);
    }
}

impl<Time, V> MulAssign<V> for Integrate<&mut V, Time>
where
    V: IntegrateExp<Time> + MulAssign<V>,
    Time: Copy,
{
    fn mul_assign(&mut self, rhs: V) {
        *self.0 *= rhs.integrate_exp(self.1);
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
