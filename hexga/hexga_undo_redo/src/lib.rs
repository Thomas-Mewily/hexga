//! A crate that handle the command pattern, ideal for undo redo of action
use std::marker::PhantomData;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref,DerefMut};

use hexga_core::prelude::*;

/// Use...
/// 
/// ```rust
/// pub use hexga_undo_redo::*;
/// ```
/// 
/// ... for a quick start
pub mod prelude;

/// The impl of action for all the std
pub mod actions;

/// Action related traits
pub mod action;
pub(crate) use action::*;

/// Commands related traits and data structure
pub mod command;
pub(crate) use command::*;
