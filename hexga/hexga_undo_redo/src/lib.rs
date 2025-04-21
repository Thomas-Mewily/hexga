//! A crate that handle the command pattern, ideal for undo redo of action
use std::marker::PhantomData;

mod undo;
pub use undo::*;

pub mod action;

pub mod prelude;