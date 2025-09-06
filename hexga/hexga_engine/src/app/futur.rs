use super::*;

// TODO: make an internal private trait, to be sure SpawnFutur can't be impl by external crate

#[cfg(not(target_arch = "wasm32"))]
/// Note : the trait bound vary if you are on wasm32 or not
pub trait SpawnFutur where
    Self: Future<Output = ()> + Send + 'static,
{
    fn spawn(self);
}

#[cfg(not(target_arch = "wasm32"))]
impl<F> SpawnFutur for F where
    F: Future<Output = ()> + Send + 'static,
{
    fn spawn(self)
    {
        async_std::task::spawn(self);
    }
}


#[cfg(target_arch = "wasm32")]
/// Note : the trait bound vary if you are on wasm32 or not
pub trait SpawnFutur where
    Self: Future<Output = ()> + 'static,
{
    fn spawn(self);
}
#[cfg(target_arch = "wasm32")]
impl<F> SpawnFutur for F where
    F: Future<Output = ()> + 'static,
{
    fn spawn(self)
    {
        wasm_bindgen_futures::spawn_local(self);
    }
}