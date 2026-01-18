use super::*;


pub trait AsyncSpawn where
    Self: Future<Output = ()> + Send + 'static,
{
    fn spawn(self);
}

impl<F> AsyncSpawn for F where
    F: Future<Output = ()> + Send + 'static,
{
    fn spawn(self)
    {
        #[cfg(not(target_arch = "wasm32"))]
        smol::spawn(self).detach();
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(self);
    }
}