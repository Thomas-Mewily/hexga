//! A crate that handle the command pattern, ideal for undo redo of action
use std::marker::PhantomData;

pub mod core;
pub use core::*;

pub mod action;

pub mod prelude;