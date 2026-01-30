#[cfg(feature = "std")]
pub use std::sync::Weak as ArcWeak;

#[cfg(not(feature = "std"))]
pub use alloc::sync::Weak as ArcWeak;
