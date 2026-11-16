use super::*;

pub mod prelude
{
    use super::traits::*;
}

pub mod traits
{
    
}

/// Time integration for rate `a += x`.
pub trait IntegrateRate<F>
{
    //type Output;

    /// Time integration: replaces `a += x` with a dt-scaled step.
    ///
    /// ```
    /// a += x;
    /// // replace it by
    /// a += x.integrate_rate(dt);
    /// ```
    fn integrate_rate(self, dt: F) -> Self;
}

/// Time integration for product `a *= x`.
pub trait IntegrateProduct<F>
{
    //type Output;

    /// Time integration: replaces `a *= x` with a dt-scaled step.
    ///
    /// ```
    /// a *= x;
    /// // replace it by
    /// a *= x.integrate_product(dt);
    /// ```
    fn integrate_product(self, dt: F) -> Self;
}

map_on_float!(
    ($name:ty) => {
        impl IntegrateRate<$name> for $name
        {
            fn integrate_rate(self, dt: $name) -> Self {
                self * dt
            }
        }

        impl IntegrateProduct<$name> for $name
        {
            fn integrate_product(self, dt: $name) -> Self {
                self * dt
            }
        }
    }
);

/// A (value, dt), so that `a += x`, `a *= x` can be written as `a += x * dt` / `a *= x * dt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Integrate<V,Time>(pub V, pub Time);

impl<Time, V> AddAssign<V> for Integrate<&mut V, Time>
where
    V: IntegrateRate<Time> + AddAssign<V>, Time: Copy,
{
    fn add_assign(&mut self, rhs: V) {
        *self.0 += rhs.integrate_rate(self.1);
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