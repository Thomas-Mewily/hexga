#[cfg(feature = "std")]
pub use std::rc::Weak as RcWeak;

#[cfg(not(feature = "std"))]
pub use alloc::rc::Weak as RcWeak;
