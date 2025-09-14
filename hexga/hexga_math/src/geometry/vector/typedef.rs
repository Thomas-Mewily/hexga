pub use super::*;
use hexga_typedef::*;

pub type Point<const N : usize> = Vector<int,   N>;
pub type Bool <const N : usize> = Vector<bool,  N>;

pub use super::vector1::*;
pub use super::vector2::*;
pub use super::vector3::*;
pub use super::vector4::*;
pub use super::xyzw_related::*;
pub use super::array_extension::*;