#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#![feature(mapped_lock_guards)]
// #![feature(clone_to_uninit)]


// use std::clone::CloneToUninit;
use std::{
    any::{Any, TypeId}, borrow::Cow, collections::HashMap, hash::Hash, iter::FusedIterator, marker::PhantomData, ops::{Deref, DerefMut}, path::{Path, PathBuf}, sync::{Arc, LazyLock, MappedRwLockWriteGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak}
};

use arc_swap::{ArcSwap, ArcSwapAny};
use hexga_core::prelude::*;
use hexga_generational::{gen_vec, multi_map::{self, EntryID}, prelude::*, table};
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_singleton::{Singleton, prelude::*};

pub mod prelude
{
    pub use super::Asset;
}


#[cfg(feature = "serde")]
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(non_upper_case_globals)]
pub(crate) static Assets : LazyLock<AssetsManagerUntyped> = LazyLock::new(|| AssetsManagerUntyped::default());

#[derive(Default)]
pub(crate) struct AssetsManagerUntyped
{
    assets: RwLock<HashMap<TypeId, Arc<AnyAsync>>>
}

impl AssetsManagerUntyped
{
    pub fn manager<T>(&self) -> AssetManager<T> where T: Async
    {
        let type_id = TypeId::of::<AssetManager<T>>();
        {
            let assets_read = self.assets.read().unwrap();
            if let Some(any_manager) = assets_read.get(&type_id)
            {
                let manager = any_manager.downcast_ref::<AssetManager<T>>().unwrap();
                return manager.clone();
            }
        }

        let mut assets_write = self.assets.write().unwrap();
        let manager = AssetManager::<T>{ inner: Arc::new(RwLock::new(AssetManagerInner::new())) };
        assets_write.insert(type_id, manager.inner.clone());
        manager
    }
}

// #[derive(Clone)]
// pub struct Asset<T>
// {
//     handle: GenID,
//     phantom: PhantomData<T>,
//     manager: Arc<RwLock<AssetManager<T>>>
// }


pub(crate) struct AssetManager<T> where T: Async
{
    inner: Arc<RwLock<AssetManagerInner<T>>>,
}
impl<T> Clone for AssetManager<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
impl<T> AssetManager<T> where T: Async
{
    pub fn error_value(&self) -> AssetWeak<T> { self.inner.read().unwrap().error.clone() }
    pub fn set_error_value(&self, value: AssetWeak<T>) -> &Self { self.inner.write().unwrap().error = value; self }

    pub fn loading_value(&self) -> AssetWeak<T> { self.inner.read().unwrap().loading.clone() }
    pub fn set_loading_value(&self, value: AssetWeak<T>) -> &Self { self.inner.write().unwrap().loading = value; self }

    pub fn new<P,Persis,F>(&self, persistance: Persis, init: F) -> Asset<T>
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        F: FnOnce() -> AssetInit<T>
    {
        let persistance = persistance.into();
        let path = match &persistance
        {
            AssetPersistance::Generated =>
            {
                let AssetInit { state, keep_loaded: _ } = init();
                return Asset::_new(state, AssetPersistance::Generated);
            }
            AssetPersistance::Persistant(path) => path.as_ref(),
        };

        let r = self.inner.read().unwrap();

        if let Some(storage) = r.values.get(path)
        {
            match storage
            {
                AssetStorage::Strong(asset) => return asset.clone(),
                AssetStorage::Weak(asset_weak) => if let Some(asset) = asset_weak.upgrade()
                {
                    return asset.clone();
                }
            }
        }

        // Not loaded or weak and invalid
        drop(r);

        let mut w = self.inner.write().unwrap();
        let AssetInit { state: value, keep_loaded } = init();
        let asset = Asset::_new(value, AssetPersistance::Persistant(path.to_owned()));

        let storage = if keep_loaded
        {
            asset.clone().into()
        }else
        {
            Asset::downgrade(&asset).into()
        };
        w.values.insert(path.to_owned(), storage);
        asset
    }
}
pub(crate) struct AssetManagerInner<T> where T: Async
{
    values: HashMap<PathBuf, AssetStorage<T>>,
    loading: AssetWeak<T>,
    error: AssetWeak<T>,
}
impl<T> AssetManagerInner<T> where T: Async
{
    pub(crate) fn new() -> Self
    {
        Self { values: ___(), loading: ___(), error: ___() }
    }
}

pub struct AssetInit<T> where T: Async
{
    pub state : AssetState<T>,
    /// If true, the asset will be keeped loaded, even if there is no reference to it
    pub keep_loaded: bool,
}
impl<T> AssetInit<T> where T: Async
{
    pub const fn with_state(state: AssetState<T>) -> Self
    {
        Self { state, keep_loaded: false }
    }
    pub const fn new(value: T) -> Self
    {
        Self::with_state(AssetState::Loaded(value))
    }
}
impl<T> Default for AssetInit<T> where T: Async + Default
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
impl<T> From<T> for AssetInit<T> where T: Async
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}



pub enum AssetStorage<T> where T: Async
{
    Strong(Asset<T>),
    Weak(AssetWeak<T>),
}
impl<T> From<Asset<T>> for AssetStorage<T> where T: Async
{
    fn from(value: Asset<T>) -> Self {
        Self::Strong(value)
    }
}
impl<T> From<AssetWeak<T>> for AssetStorage<T> where T: Async
{
    fn from(value: AssetWeak<T>) -> Self {
        Self::Weak(value)
    }
}

pub(crate) struct AssetData<T> where T: Async
{
    state: AssetState<T>,
    persistance: AssetPersistance,
}

pub enum AssetState<T> where T: Async
{
    Loading,
    Loaded(Arc<T>),
    Error(IoError),
}
impl<T> From<IoResult<T>> for AssetState<T> where T: Async
{
    fn from(value: IoResult<T>) -> Self {
        match value
        {
            Ok(loaded) => Self::Loaded(Arc::new(loaded)),
            Err(error) => Self::Error(error),
        }
    }
}

pub enum AssetPersistance<P=PathBuf>
{
    Persistant(P),
    Generated,
}
impl<P> From<P> for AssetPersistance<P>
{
    fn from(value: P) -> Self {
        Self::Persistant(value)
    }
}


pub struct Asset<T> where T: Async
{
    // 3 levels of indirection... I'm not proud of that
    inner: Arc<ArcSwap<AssetData<T>>>,
}
impl<T> Asset<T> where T: Async
{
    pub(crate) fn _new(state: AssetState<T>, persistance: AssetPersistance) -> Self
    {
        Self { inner: Arc::new(ArcSwap::new(Arc::new(AssetData { state, persistance }))) }
    }

    pub fn with_state<P,Persis,F>(persistance: Persis, init: F) -> Self
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        F: FnOnce() -> AssetInit<T>
    {
        Assets.manager().new(persistance, init)
    }



    pub fn load<P>(path : P) -> Self where P: AsRef<Path>, T: Load
    {
        let path: &Path = path.as_ref();
        Self::with_state::<&Path, _, _>(AssetPersistance::Persistant(path), || AssetInit::with_state(AssetState::<T>::from(T::load_from_disk(path))))
    }

    pub fn load_or_create<P,Persis,F>(persistance: Persis, init: F) -> Self
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        F: FnOnce() -> T
    {
        let asset = Assets.manager().new(persistance, || AssetInit::with_state(AssetState::Error(___())));
        if asset

        AssetInit::new(init())
    }
}

//pub type AssetReadGuard =
impl<T> Asset<T> where T: Async
{
    pub fn read(&self) -> ()
    {
        self.inner.load()
    }
}
impl<T> Clone for Asset<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
impl<T> Asset<T> where T: Async
{
    pub fn downgrade(this: &Self) -> AssetWeak<T>
    {
        AssetWeak{ inner: Arc::downgrade(&this.inner) }
    }
}
/*
impl<T> TryBorrow for &Asset<T> where T: Async
{
    type Borrow=Arc<AssetData<T>>;
    type Error=();

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        Ok(self.inner.load().clone())
    }
}
*/


pub struct AssetWeak<T> where T: Async
{
    inner: Weak<ArcSwap<AssetData<T>>>,
}
impl<T> Clone for AssetWeak<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
impl<T> Default for AssetWeak<T> where T: Async
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> AssetWeak<T> where T: Async
{
    pub const fn new() -> Self { Self { inner: Weak::new() }}
    pub fn upgrade(&self) -> Option<Asset<T>>
    {
        match self.inner.upgrade()
        {
            Some(value) => Some(Asset{ inner: value }),
            None => None,
        }
    }
}