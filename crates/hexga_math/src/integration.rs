use super::*;

pub mod prelude
{
    pub use super::Integrate;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{IntegrateLinear, IntegrateProduct};
}

/// Time integration for rate `a += x`.
pub trait IntegrateLinear<F>
{
    //type Output;

    /// Time integration: replaces `a += x` with a dt scaled step.
    ///
    /// ```ignore
    /// a += x; // not time integrated
    /// =>
    /// a += (x * dt); // time integrated
    /// // replace it by
    /// a += x.integrate_linear(dt);
    /// ```
    fn integrate_linear(self, dt: F) -> Self;
}

/// Time integration for product `a *= x`.
pub trait IntegrateProduct<F>
{
    //type Output;

    /// Time integration: replaces `a *= x` with a dt scaled step.
    ///
    /// ```ignore
    /// a *= x; // not time integrated
    /// =>
    /// a *= (x.pow(dt)); // time integrated
    /// // replace it by
    /// a *= x.integrate_product(dt);
    /// ```
    fn integrate_product(self, dt: F) -> Self;
}

map_on_float!(
    ($name:ty) => {
        impl IntegrateLinear<$name> for $name
        {
            fn integrate_linear(self, dt: $name) -> Self {
                self * dt
            }
        }

        impl IntegrateProduct<$name> for $name
        {
            fn integrate_product(self, dt: $name) -> Self {
                self.pow(dt)
            }
        }
    }
);

/// A (value, dt), so that `a += x`, `a *= x` can be written as `a += x * dt` / `a *= x * dt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Integrate<V,Time>(pub V, pub Time);

/*
// TODO impl it
impl<Time, V, V2, C> Lerp<C, V2> for  Integrate<V, Time>
    where V: Lerp<C, V2, Output=V>, C: IntegrateProduct<Time>
{
    type Output = V;
    fn lerp_unchecked(self, dest: V2, coef: C) -> Self::Output {

    }
}
*/

// https://youtu.be/LSNQuFEDOyQ *Lerp smoothing is broken* by Freya Holmér

/*
impl<Time, V, V2, C> Lerp<C, V2> for Integrate<V, Time>
    where V: Lerp<C, V2, Output=V>, C: IntegrateProduct<Time>
{
    type Output = V;

    fn lerp_unchecked(self, dest: V2, coef: C) -> Self::Output {
        let Integrate(value, dt) = self;
        value.lerp_unchecked(dest, coef.integrate_product(dt))
    }
}
*/


impl<Time, V, V2, C> Lerp<C, V2> for  Integrate<&mut V, Time>
    where V: Lerp<C, V2, Output=V> + Copy, C: IntegrateProduct<Time>
{
    type Output = ();
    fn lerp_unchecked(self, dest: V2, coef: C) -> Self::Output {
        let Integrate(value, dt) = self;
        *value = (*value).lerp_unchecked(dest, coef.integrate_product(dt));
        ()
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
    V: IntegrateProduct<Time> + MulAssign<V>,
    Time: Copy,
{
    fn mul_assign(&mut self, rhs: V) {
        *self.0 *= rhs.integrate_product(self.1);
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
pos *= 4.integrate_product(dt.s());


integrate(&mut pos, dt.s()) + 4
*/