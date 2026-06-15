use super::*;

// Todo: finish it.
// Missing Crop trait, and see how to merge the experimental grid with grid using bijection
mod bijection;
pub use bijection::*;

mod bijection_fn;
pub use bijection_fn::*;

pub mod prelude
{
    pub use super::bijection::{Bijection, WithBijection};
}
