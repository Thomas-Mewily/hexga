//! # Geometry Rectangle Module
//!
//! This module provides generic, N-dimensional rectangles and related utilities for geometric computations.
//! It supports rectangles in arbitrary dimensions, with methods for construction, manipulation, cropping, intersection, and iteration.
//!
//! ## Features
//!
//! - Generic `Rectangle<T, N>` type for N-dimensional rectangles
//! - Construction from position and size, or from two corner points
//! - Querying rectangle corners, edges, and center points
//! - Cropping and margin operations
//! - Intersection checks
//! - Iteration over rectangle indices (for integer types)
//! - Traits for rectangle-like types (`IRectangle`, `Crop`)
//!
//! ## Usage
//!
//! ```rust
//! use hexga_math::prelude::*;
//!
//! let rect = rect2i(0, 0, 10, 10);
//! assert!(rect.is_inside(vec2i(5, 5)));
//!
//! let cropped = rect.crop_margin_intersect(vec2i(2, 2), vec2i(2, 2));
//! assert_eq!(cropped, rect2i(2, 2, 6, 6));
//! ```
//!
//! ## See Also
//!
//! - [`Vector`] for N-dimensional vector operations
//! - [`GetRectangle`] and [`SetRectangle`] trait for rectangle-like abstractions
//! - [`Crop`] trait for cropping operations
use super::*;

mod typedef;
pub use typedef::*;

mod iter;
pub use iter::*;

mod rectangle;
pub use rectangle::*;

pub mod prelude
{
    pub use super::
    {
        typedef::*,
        rectangle::{Rectangle,Crop,GetRectangle,SetRectangle},
    };
}