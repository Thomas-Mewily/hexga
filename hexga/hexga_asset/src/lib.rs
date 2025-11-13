#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#![feature(mapped_lock_guards)]
// #![feature(clone_to_uninit)]


// use std::clone::CloneToUninit;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr::NonNull,
    sync::{Arc, LazyLock, MappedRwLockWriteGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard}
};

use hexga_core::prelude::*;
use hexga_generational::{gen_vec, multi_map::{self, EntryID}, prelude::*, table};
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_singleton::prelude::*;


#[cfg(feature = "serde")]
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(non_upper_case_globals)]
pub static Assets : SingletonMultiThread<AssetsManagerUntyped> = SingletonMultiThread::new(|| AssetsManagerUntyped::default());

pub(crate) fn assets() -> RwLockWriteGuard<'static, LazyLock<AssetsManagerUntyped>>
{
    Assets.instance_mut()
}

#[derive(Default)]
pub struct AssetsManagerUntyped
{
    assets: HashMap<TypeId, Pin<Box<dyn AsyncAny>>>
}
impl AssetsManagerUntyped
{
    fn manager<T>(&mut self) -> &'static mut AssetManager<T> where T: Async
    {
        let key = std::any::TypeId::of::<T>();

        let pinned_any: &mut Pin<Box<dyn AsyncAny>> = self.assets.entry(key).or_insert_with(|| Box::pin(AssetManager::<T>::new()));

        // SAFELY get a mutable reference to the inner value using unsafe
        // Because Pin guarantees the value won't move, this is safe
        let manager: &mut AssetManager<T> = unsafe {
            let raw: *mut dyn Any = Pin::as_mut(pinned_any).get_unchecked_mut();
            &mut *(raw as *mut AssetManager<T>)
        };
        manager
    }
}

pub struct AssetManager<T> where T: Async
{
    value: RwLock<MultiHashMap<Path,AssetData<T>>>,
    loading: RwLock<Option<Arc<T>>>,
    error : RwLock<Option<Arc<T>>>,
}
impl<T> AssetManager<T> where T: Async
{
    pub(crate) fn new() -> Self
    {
        Self { value: ___(), loading: ___(), error: ___() }
    }
}

struct AssetData<T> where T: Async
{
    inner: RwLock<AssetDataInner<T>>,
}
struct AssetDataInner<T> where T: Async
{
    state: AssetState<T>,
    path: Path,
    id  : GenID,
}

impl<T> AssetData<T> where T: Async
{
    pub fn manager() -> &'static AssetManager<T>
    {
        assets().manager()
    }
}

impl<'a,T> TryBorrow for &'a AssetData<T> where T: Async
{
    type Borrow=Arc<T>;
    type Error=();

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        match &self.inner.read().ok_or(())?.state
        {
            AssetState::Loading =>
            {
                let manager = AssetData::<T>::manager();
                let loading_guard = manager.loading.read().ok_or(())?;
                Ok(loading_guard.as_ref().ok_or(())?.clone())
            },
            AssetState::Loaded(val) => Ok(val.clone()),
            AssetState::Error(encode_error) =>
            {
                let manager = AssetData::<T>::manager();
                let loading_guard = manager.error.read().ok_or(())?;
                Ok(loading_guard.as_ref().ok_or(())?.clone())
            },
        }
    }
}
impl<'a,T> TryBorrowMut for &'a AssetData<T> where T: Async
{
    type Borrow=MappedRwLockWriteGuard<'a,Arc<T>>;
    type Error=();

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        let write_guard = self.inner.write().ok_or(())?;

        if !write_guard.state.is_loaded()
        {
            return Err(());
        }
        let mapped = RwLockWriteGuard::map(write_guard,
            |g|
            if let AssetState::Loaded(l) = &mut g.state { l } else { unreachable!() }
        );
        Ok(mapped)
    }
}


pub struct Asset<T> where T: Async
{
    value: NonNull<RwLock<AssetData<T>>>,
}


impl<T> AssetData<T> where T: Async
{

}

pub(crate) enum AssetState<T> where T: Async
{
    Loading,
    Loaded(Arc<T>),
    Error(EncodeError),
}
impl<T> AssetState<T> where T: Async
{
    pub const fn is_loading(&self) -> bool { matches!(self, Self::Loading) }
    pub const fn is_loaded(&self) -> bool { matches!(self, Self::Loaded(_)) }
    pub const fn is_error(&self) -> bool { matches!(self, Self::Error(_)) }
}
