// based on [miniquad::fs](https://github.com/not-fl3/miniquad/blob/master/src/fs.rs)
use super::*;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::*;

#[cfg(not(any(target_arch = "wasm32")))]
pub(crate) mod desktop;
#[cfg(not(any(target_arch = "wasm32")))]
pub(crate) use desktop::*;