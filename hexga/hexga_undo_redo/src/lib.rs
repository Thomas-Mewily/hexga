//! A crate that handle the command pattern, ideal for undo redo of action
use std::marker::PhantomData;
use std::fmt::Debug;
use std::hash::Hash;

use hexga_base::*;

mod undo;
pub use undo::*;

pub mod action;

pub mod prelude;