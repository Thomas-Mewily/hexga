#![allow(unused)]
pub use hexga_event_loop::prelude::*;

struct MonJeu;

impl PlatformEventHandler for MonJeu {}

fn main() { hexga_event_loop::event_loop::run_with_param(|proxy| MonJeu, Default::default()); }
