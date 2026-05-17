🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.
It is subject to **breaking changes** in future releases.
Use it at your own risk, and keep in mind that the API may change in future versions.


## HexGa Guard

Hexga guard provides an unified trait interfaces (`Guarded` and `GuardedMut`) traits with type that support interior mutability.  It's offer the same API, independ of the synchronization primitives, allowing you to write generic code that works with `Mutex`, `RwLock`, `RefCell`.

The crate also define the `Guard` and `GuardedMut` traits, and an easy way to map/project a value within the guard.

- Todo: add a non blocking version / API.

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.