use super::*;


pub trait SpawnFutur where
    Self: Future<Output = ()> + Async,
{
    fn spawn(self);
}

impl<F> SpawnFutur for F where
    F: Future<Output = ()> + Async,
{
    fn spawn(self)
    {
        #[cfg(not(target_arch = "wasm32"))]
        async_std::task::spawn(self);
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(self);
    }
}