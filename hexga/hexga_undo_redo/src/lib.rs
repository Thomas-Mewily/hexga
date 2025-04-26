//! A crate that handle the command pattern, ideal for undo redo of action
use std::marker::PhantomData;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref,DerefMut};

use hexga_base::*;

pub mod action;

mod core;
pub use core::*;

pub mod prelude;