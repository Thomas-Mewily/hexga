#[allow(unused_imports)]
use core::{any::Any, pin::Pin};

pub trait Async: WasmSend + WasmSync + 'static {}
impl<T> Async for T where T: WasmSend + WasmSync + 'static {}

pub trait AnyAsync : Any + WasmSendSync + 'static
{
    fn as_any(&self) -> &dyn Any;
}
impl<T> AnyAsync for T where T: Any + WasmSendSync + 'static 
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub type DynAnyAsync = dyn AnyAsync + 'static;

//pub type DynAnyAsync = dyn Any + Send + Sync + 'static;


// Condition: Native OR WASM with atomics (multi-thread capable)
#[cfg(any(
    not(target_arch = "wasm32"),
    all(target_arch = "wasm32", target_feature = "atomics")
))]
mod sync_trait
{
    /// Send trait compatible with Wasm. Requires real Send on native.
    pub trait WasmSend: Send {}
    
    /// Sync trait compatible with Wasm. Requires real Sync on native.
    pub trait WasmSync: Sync {}
    
    /// Send + Sync trait compatible with Wasm. Requires real Send + Sync on native.
    pub trait WasmSendSync: WasmSend + WasmSync {}
    
    impl<T: Send> WasmSend for T {}
    impl<T: Sync> WasmSync for T {}
    impl<T: Send + Sync> WasmSendSync for T {}
}

// Condition: WASM without atomics (pure single-thread)
#[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
mod sync_trait
{
    /// Send trait for WASM compatibility.
    pub trait WasmSend {}
    
    /// Sync trait for WASM compatibility.
    pub trait WasmSync {}
    
    /// Send + Sync trait for WASM compatibility.
    pub trait WasmSendSync: WasmSend + WasmSync {}
    
    impl<T> WasmSend for T {}
    impl<T> WasmSync for T {}
    impl<T> WasmSendSync for T {}
}
pub use sync_trait::*;
