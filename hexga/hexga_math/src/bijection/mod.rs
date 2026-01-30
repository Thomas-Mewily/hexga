use super::*;

mod bijection;
pub use bijection::*;

mod bijection_fn;
pub use bijection_fn::*;

pub mod prelude
{
    pub use super::bijection::{Bijection, WithBijection};
}
