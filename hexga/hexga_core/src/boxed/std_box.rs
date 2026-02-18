
#[cfg(feature = "std")]
pub use std::boxed::*;
#[cfg(not(feature = "std"))]
pub use ::alloc::boxed::*;