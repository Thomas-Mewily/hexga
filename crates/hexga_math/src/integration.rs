/// Frame rate independent time integration
use super::*;

pub mod prelude
{
    pub use super::Integrate;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{SmoothLinear, SmoothPow, SmoothExp, SmoothDecay};
    pub use super::{AddDt,SubDt,MulDt,DivDt,DecayDt,LerpDt};
}

/// Frame rate independent time integration for a linear fn `a += x` => `a += x * dt`.
pub trait SmoothLinear<Dt>
{
    //type Output;

    /// Frame rate independent time integration: replaces `a += x` with a dt scaled step.
    ///
    /// ```ignore
    /// a += x;         // not time integrated
    /// =>              // time integrated
    /// a += x * dt; 
    /// <=>
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

/// Frame rate independent time integration for a multiplication `a *= x` => `x.pow(dt)`.
pub trait SmoothPow<Dt>
{
    //type Output;

    /// Frame rate independent time integration: replaces `a *= x` => `x.pow(dt)`.
    ///
    /// ```ignore
    /// a *= x;         // not time integrated
    /// =>              // time integrated
    /// a *= x.pow(dt); 
    /// <=>
    /// a *= x.smooth_pow(dt);
    /// ```
    ///
    /// `dt=0 -> 1`, `dt=1 -> x`, and `f(a) * f(b) = f(a + b)`.
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

/// Frame rate independent time integration for a continuous decay coefficient `a *= exp(-coef)` => `a *= exp(-coef * dt)`.
pub trait SmoothExp<Dt>
{
    //type Output;

    /// Frame rate independent time integration: replaces `a *= exp(-coef)` with a dt scaled step.
    ///
    /// ```ignore
    /// a *= exp(-coef);        // not time integrated
    /// =>                      // time integrated
    /// a *= exp(-coef * dt); 
    /// <=>
    /// a *= coef.smooth_exp(dt);
    /// ```
    ///
    /// Returns the **retention factor** over `dt`: `exp(-coef * dt)`.
    /// `self` is the non negative decay coefficient per time unit, measured in inverse time units;
    /// `dt=0 -> 1`, `dt=1 -> exp(-coef)`, and `f(a) * f(b) = f(a + b)`.
    fn smooth_exp(self, dt: Dt) -> Self;
}

impl<T> SmoothExp<T> for T
where
    T: Float,
{
    #[inline]
    fn smooth_exp(self, dt: T) -> Self {
        (self * dt).neg().exp()
    }
}

/// Frame rate independent time integration for a continuous decay coefficient : `1.0 - exp(-coef)` => `1.0 - exp(-coef * dt)`
pub trait SmoothDecay<Dt>
{
    /// Returns the fraction decayed over `dt`: `1 - exp(-coef * dt)`.
    ///
    /// ```ignore
    /// a *= 1.0 - exp(-coef);      // not time integrated
    /// =>                          // time integrated
    /// a *= 1.0 - exp(-coef * dt); 
    /// <=>
    /// a *= 1.0 - coef.smooth_decay(dt);
    /// ```
    ///
    /// `self` is the non negative decay coefficient per time unit, measured in inverse time units.
    /// `dt=0 -> 0`, `dt=1 -> 1 - exp(-coef)`. For adjacent intervals,
    /// `f(a + b) = f(a) + f(b) - f(a) * f(b)`.
    fn smooth_decay(self, dt: Dt) -> Self;
}

impl<T> SmoothDecay<T> for T
where
    T: Float,
{
    #[inline]
    fn smooth_decay(self, dt: T) -> Self {
        Self::ONE - self.smooth_exp(dt)
    }
}








/// Frame rate independent time integration of an additive rate: `a += rate`.
pub trait AddDt<Dt> {
    /// Applies `rate` over `dt`: `a += rate * dt`.
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

/// Frame rate independent time integration of a subtraction: `a -= rate`.
pub trait SubDt<Dt> {
    /// Applies `rate` over `dt`: `a -= rate * dt`.
    fn sub_dt(self, rate: Self, dt: Dt) -> Self;
}

impl<T, Dt> SubDt<Dt> for T
where
    T: Sub<Output = Self> + SmoothLinear<Dt>,
{
    #[inline]
    fn sub_dt(self, rate: Self, dt: Dt) -> Self {
        self - rate.smooth_linear(dt)
    }
}

/// Frame rate independent time integration of a multiplicative factor: `a *= factor`.
pub trait MulDt<Dt> {
    /// Applies `factor` (a per-time-unit multiplier) over `dt`: `a *= factor.pow(dt)`.
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

/// Frame rate independent time integration of a division: `a /= factor`.
pub trait DivDt<Dt> {
    /// Applies `factor` over `dt`: `a /= factor.pow(dt)`.
    fn div_dt(self, factor: Self, dt: Dt) -> Self;
}

impl<T, Dt> DivDt<Dt> for T
where
    T: Div<Output = Self> + SmoothPow<Dt>,
{
    #[inline]
    fn div_dt(self, factor: Self, dt: Dt) -> Self {
        self / factor.smooth_pow(dt)
    }
}

/// Frame rate independent time integration of exponential decay: `a *= exp(-rate)` per time unit.
pub trait DecayDt<Rate, Dt> {
    /// Applies a non negative decay rate, measured in inverse time units, over `dt`.
    ///
    /// ```ignore
    /// a *= exp(-rate);        // not time integrated
    /// =>                      // time integrated
    /// a *= exp(-rate * dt); 
    /// <=>
    /// a = a.decay_dt(rate, dt);
    /// ```
    ///
    /// `rate` is non negative and measured in inverse time units; `dt=0` leaves `a` unchanged.
    fn decay_dt(self, rate: Rate, dt: Dt) -> Self;
}

impl<T, Rate, Dt> DecayDt<Rate, Dt> for T
where
    Rate: SmoothExp<Dt>,
    T: Mul<Rate, Output=Self>,
{
    #[inline]
    fn decay_dt(self, rate: Rate, dt: Dt) -> Self {
        self * rate.smooth_exp(dt)
    }
}



/// A `(value, dt)` pair for applying time-scaled arithmetic and interpolation operations.
///
/// ```ignore
/// a + coef // not time integrated
/// a - coef
/// a * coef
/// a / coef
/// a.lerp(dest, coef)
///
/// // replace them by the time integrated version:
/// Integrate(a, dt) + coef
/// Integrate(a, dt) - coef
/// Integrate(a, dt) * coef
/// Integrate(a, dt) / coef
/// Integrate(a, dt).lerp(dest, coef)
///
/// // The assignment operators work on a mutable reference
/// Integrate(&mut a, dt) += coef;
/// Integrate(&mut a, dt) -= coef;
/// Integrate(&mut a, dt) *= coef;
/// Integrate(&mut a, dt) /= coef;
/// ```
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

/*
/// Frame rate independent interpolation of a mutable value.
///
/// ```ignore
/// Integrate(&mut value, dt).lerp(dest, coef);
/// ```
///
/// `coef` is the fraction covered per time unit. The integrated lerp weight is
/// `1 - (1 - coef).pow(dt)`.
impl<F, V, V2> Lerp<F, V2> for Integrate<&mut V, F>
    where V: Lerp<F, V2, Output=V> + Copy, F: Float
{
    type Output = ();
    fn lerp_unclamped(self, dest: V2, coef: F) -> Self::Output {
        let Integrate(value, dt) = self;
        let retention = (F::ONE - coef).smooth_pow(dt);
        let alpha = F::ONE - retention;
        *value = (*value).lerp_unclamped(dest, alpha);
        ()
    }
}
*/

impl<F, V, D> Lerp<F, D> for Integrate<V, F>
where
    V: Lerp<F, D, Output=V>,
    F: Float,
{
    type Output = V;

    fn lerp_unclamped(self, dest: D, coef: F) -> Self::Output {
        let Integrate(value, dt) = self;
        debug_assert!(dt.is_strictly_positive());
        let retention = (F::ONE - coef).smooth_pow(dt);
        let alpha = F::ONE - retention;
        value.lerp_unclamped(dest, alpha)
    }
}

/// Frame rate independent linear interpolation, with the per-time-unit coefficient integrated over `dt`.
///
/// For coefficient `c`, the interpolation weight over `dt` is `1 - (1 - c).pow(dt)`.
pub trait LerpDt<C, D=Self, Dt=C> : Sized
{
    type Output;

    /// Frame rate independent time integration: replaces a per-unit lerp coefficient with a dt scaled step.
    ///
    /// ```ignore
    /// value = value.lerp(dest, coef); // not time integrated
    /// // replace it by
    /// value.lerp_dt(dest, coef, dt);
    /// ```
    ///
    /// The integrated coefficient is `1 - (1 - coef).pow(dt)`. `coef` is clamped to `0.0..=1.0`.
    /// `dt` should be non negative.
    fn lerp_dt(self, dest: D, coef_per_time_unit: C, dt: Dt) -> Self::Output
    where
        C: ClampUnit,
    {
        self.lerp_dt_unchecked(dest, coef_per_time_unit.clamp_unit(), dt)
    }

    /// Performs frame rate independent interpolation in place.
    ///
    /// For per-time-unit coefficient `c`, the interpolation weight is
    /// `1 - (1 - c).pow(dt)`. `c` is clamped to `0.0..=1.0`; `dt` should be non negative.
    ///
    /// ```ignore
    /// value.lerp_dt_to(dest, coef, dt);
    /// ```
    fn lerp_dt_to(&mut self, dest: D, coef_per_time_unit: C, dt: Dt)
    where
        Self: Copy + LerpDt<C, D, Dt, Output=Self>,
        C: ClampUnit,
    {
        *self = (*self).lerp_dt(dest, coef_per_time_unit, dt);
    }

    /// Like [`LerpDt::lerp_dt`], without clamping `coef` to `0.0..=1.0`.
    fn lerp_dt_unchecked(self, dest: D, coef_per_time_unit: C, dt: Dt) -> Self::Output;

    /// Like [`LerpDt::lerp_dt_to`], without clamping the per-time-unit coefficient to
    /// `0.0..=1.0`.
    ///
    /// For coefficient `self`, the interpolation weight is `1 - (1 - self).pow(dt)`. 
    /// `dt` should be non negative.
    ///
    /// ```ignore
    /// value.lerp_dt_unclamped_to(dest, coef, dt);
    /// ```
    fn lerp_dt_unclamped_to(&mut self, dest: D, coef_per_time_unit: C, dt: Dt)
    where
        Self: Copy + LerpDt<C, D, Dt, Output=Self>,
    {
        *self = (*self)
            .lerp_dt_unchecked(dest, coef_per_time_unit, dt);
    }
}

impl<F, V, V2> LerpDt<F, V2, F> for V
where
    V: Lerp<F, V2, Output=V>,
    F: Float
{
    type Output = Self;

    fn lerp_dt_unchecked(self, dest: V2, coef_per_time_unit: F, dt: F) -> Self::Output {
        debug_assert!(dt.is_strictly_positive());
        Integrate(self, dt).lerp_unclamped(dest, coef_per_time_unit)
    }
}

/// Frame rate independent interpolation using a continuous exponential decay rate.
///
/// For a non negative rate `self`, the interpolation weight over `dt` is `1 - exp(-r * dt)`. 
/// Unlike [`LerpDt`], `self` is measured in inverse time units.
pub trait LerpDecay<Rate, D=Self, Dt=Rate> : Sized
{
    type Output;

    /// Moves `self` toward `dest` with continuous exponential decay over `dt`.
    ///
    /// The interpolation weight is clamped to `0.0..=1.0`.
    fn lerp_decay(self, dest: D, rate: Rate, dt: Dt) -> Self::Output;

    /// Moves `self` toward `dest` in place using continuous exponential decay over `dt`.
    ///
    /// For decay rate `self`, the interpolation weight is `1 - exp(-self * dt)`, clamped to
    /// `0.0..=1.0`. `rate` is measured in inverse time units.
    ///
    /// ```ignore
    /// value.lerp_decay_to(dest, rate, dt);
    /// ```
    fn lerp_decay_to(&mut self, dest: D, rate: Rate, dt: Dt)
    where
        Self: Copy + LerpDecay<Rate, D, Dt, Output=Self>,
    {
        *self = (*self).lerp_decay(dest, rate, dt);
    }

    /// Like [`LerpDecay::lerp_decay`], without clamping the interpolation weight.
    ///
    /// `self` and `dt` should be non negative.
    fn lerp_decay_unclamped(self, dest: D, rate: Rate, dt: Dt) -> Self::Output;

    /// Like [`LerpDecay::lerp_decay_to`], without clamping the interpolation weight.
    ///
    /// For decay rate `self`, the interpolation weight is `1 - exp(-self * dt)`. 
    /// `self` and `dt` should be non negative.
    ///
    /// ```ignore
    /// value.lerp_decay_unclamped_to(dest, rate, dt);
    /// ```
    fn lerp_decay_unclamped_to(&mut self, dest: D, rate: Rate, dt: Dt)
    where
        Self: Copy + LerpDecay<Rate, D, Dt, Output=Self>,
    {
        *self = (*self).lerp_decay_unclamped(dest, rate, dt);
    }
}

impl<F, V, V2> LerpDecay<F, V2, F> for V
where
    V: Lerp<F, V2, Output=V>,
    F: Float,
{
    type Output = Self;

    fn lerp_decay(self, dest: V2, rate: F, dt: F) -> Self::Output {
        debug_assert!(dt.is_strictly_positive());
        let alpha = (F::ONE - rate.smooth_exp(dt)).clamp_unit();
        self.lerp_unclamped(dest, alpha)
    }

    fn lerp_decay_unclamped(self, dest: V2, rate: F, dt: F) -> Self::Output {
        debug_assert!(rate.is_strictly_positive());
        debug_assert!(dt.is_strictly_positive());
        let alpha = F::ONE - rate.smooth_exp(dt);
        self.lerp_unclamped(dest, alpha)
    }
}

/// Frame rate independent interpolation parameterized by half-life.
///
/// For a positive half-life `h`, the interpolation weight over `dt` is
/// `1 - 2.pow(-dt / h)`. After one half-life, half of the remaining distance is covered.
pub trait LerpHalfLife<HalfLife, D=Self, Dt=HalfLife> : Sized
{
    type Output;

    /// Moves `self` toward `dest`, covering half of the remaining distance every `half_life`.
    ///
    /// The interpolation weight is clamped to `0.0..=1.0`.
    fn lerp_half_life(self, dest: D, half_life: HalfLife, dt: Dt) -> Self::Output;

    /// Moves `self` toward `dest` in place, covering half of the remaining distance every
    /// `half_life`.
    ///
    /// For half-life `h`, the interpolation weight is `(1 - 2).pow(-dt / h)`, clamped to
    /// `0.0..=1.0`.
    ///
    /// ```ignore
    /// value.lerp_half_life_to(dest, half_life, dt);
    /// ```
    fn lerp_half_life_to(&mut self, dest: D, half_life: HalfLife, dt: Dt)
    where
        Self: Copy + LerpHalfLife<HalfLife, D, Dt, Output=Self>,
    {
        *self = (*self).lerp_half_life(dest, half_life, dt);
    }

    /// Like [`LerpHalfLife::lerp_half_life`], without clamping the interpolation weight.
    ///
    /// `half_life` must be positive and `dt` should be non negative; these invariants are
    /// checked in debug builds.
    fn lerp_half_life_unclamped(self, dest: D, half_life: HalfLife, dt: Dt) -> Self::Output;

    /// Like [`LerpHalfLife::lerp_half_life_to`], without clamping the interpolation weight.
    ///
    /// For half-life `h`, the interpolation weight is `1 - 2.pow(-dt / h)`. `half_life` must
    /// be positive and `dt` should be non negative; these invariants are checked in debug builds.
    ///
    /// ```ignore
    /// value.lerp_half_life_unclamped_to(dest, half_life, dt);
    /// ```
    fn lerp_half_life_unclamped_to(&mut self, dest: D, half_life: HalfLife, dt: Dt)
    where
        Self: Copy + LerpHalfLife<HalfLife, D, Dt, Output=Self>,
    {
        *self = (*self).lerp_half_life_unclamped(dest, half_life, dt);
    }
}

impl<F, V, V2> LerpHalfLife<F, V2, F> for V
where
    V: Lerp<F, V2, Output=V>,
    F: Float,
{
    type Output = Self;

    fn lerp_half_life(self, dest: V2, half_life: F, dt: F) -> Self::Output {
        debug_assert!(half_life.is_strictly_positive());
        debug_assert!(dt.is_strictly_positive());
        let retention = (F::ONE + F::ONE).smooth_pow(-(dt / half_life));
        let alpha = (F::ONE - retention).clamp_unit();
        self.lerp_unclamped(dest, alpha)
    }

    fn lerp_half_life_unclamped(self, dest: V2, half_life: F, dt: F) -> Self::Output {
        debug_assert!(half_life.is_strictly_positive());
        debug_assert!(dt.is_strictly_positive());
        let retention = (F::ONE + F::ONE).smooth_pow(-(dt / half_life));
        let alpha = F::ONE - retention;
        self.lerp_unclamped(dest, alpha)
    }
}

impl<Dt, V> Add<V> for Integrate<V, Dt>
where
    V: AddDt<Dt>
{
    type Output = V;

    fn add(self, rhs: V) -> Self::Output {
        let Integrate(value, dt) = self;
        value.add_dt(rhs, dt)
    }
}

impl<Dt, V> Sub<V> for Integrate<V, Dt>
where
    V: SubDt<Dt>
{
    type Output = V;

    fn sub(self, rhs: V) -> Self::Output {
        let Integrate(value, dt) = self;
        value.sub_dt(rhs, dt)
    }
}

impl<Dt, V> Mul<V> for Integrate<V, Dt>
where
    V: MulDt<Dt>
{
    type Output = V;

    fn mul(self, rhs: V) -> Self::Output {
        let Integrate(value, dt) = self;
        value.mul_dt(rhs, dt)
    }
}

impl<Dt, V> Div<V> for Integrate<V, Dt>
where
    V: DivDt<Dt>
{
    type Output = V;

    fn div(self, rhs: V) -> Self::Output {
        let Integrate(value, dt) = self;
        value.div_dt(rhs, dt)
    }
}

impl<Dt, V, Rate> Decay<Rate> for Integrate<V, Dt>
where
    V: DecayDt<Rate,Dt>
{
    type Output = V;
    fn decay(self, rate: Rate) -> Self::Output {
        let Integrate(value, dt) = self;
        value.decay_dt(rate, dt)
    }
}

impl<Dt, V> AddAssign<V> for Integrate<&mut V, Dt>
where
    V: SmoothLinear<Dt> + AddAssign<V>, Dt: Copy
{
    fn add_assign(&mut self, rhs: V) {
        *self.0 += rhs.smooth_linear(self.1);
    }
}

impl<Dt, V> SubAssign<V> for Integrate<&mut V, Dt>
where
    V: SmoothLinear<Dt> + SubAssign<V>, Dt: Copy
{
    fn sub_assign(&mut self, rhs: V) {
        *self.0 -= rhs.smooth_linear(self.1);
    }
}

impl<Dt, V> MulAssign<V> for Integrate<&mut V, Dt>
where
    V: SmoothPow<Dt> + MulAssign<V>, Dt: Copy
{
    fn mul_assign(&mut self, rhs: V) {
        *self.0 *= rhs.smooth_pow(self.1);
    }
}

impl<Dt, V> DivAssign<V> for Integrate<&mut V, Dt>
where
    V: SmoothPow<Dt> + DivAssign<V>, Dt: Copy
{
    fn div_assign(&mut self, rhs: V) {
        *self.0 /= rhs.smooth_pow(self.1);
    }
}
