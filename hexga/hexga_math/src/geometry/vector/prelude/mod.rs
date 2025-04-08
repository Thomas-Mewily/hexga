use super::*;

pub type Point<const N : usize> = Vector<int,   N>;
pub type Bool <const N : usize> = Vector<bool,  N>;

mod vector1;
pub use vector1::*;

mod vector2;
pub use vector2::*;

mod vector3;
pub use vector3::*;

mod vector4;
pub use vector4::*;

mod xyzw_related;
pub use xyzw_related::*;