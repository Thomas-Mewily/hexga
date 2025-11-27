use super::*;


// TODO: replace Futurable by Async
// trait Async : Send + Sync + 'static {}



#[cfg(not(target_arch = "wasm32"))]
/// Note : the trait bound vary if you are on wasm32 or not
pub trait Futurable: Send + 'static {}
#[cfg(not(target_arch = "wasm32"))]
impl<T> Futurable for T where T: Send + 'static {}

#[cfg(target_arch = "wasm32")]
/// Note : the trait bound vary if you are on wasm32 or not
pub trait Futurable: 'static {}
#[cfg(target_arch = "wasm32")]
impl<T> Futurable for T where T: 'static {}


pub trait SpawnFutur where
    Self: Future<Output = ()> + Futurable,
{
    fn spawn(self);
}

impl<F> SpawnFutur for F where
    F: Future<Output = ()> + Futurable,
{
    fn spawn(self)
    {
        #[cfg(not(target_arch = "wasm32"))]
        async_std::task::spawn(self);
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(self);
    }
}