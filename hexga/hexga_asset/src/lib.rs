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
    sync::{Arc, LazyLock, MappedRwLockWriteGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak}
};

use hexga_core::prelude::*;
use hexga_generational::{gen_vec, multi_map::{self, EntryID}, prelude::*, table};
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_singleton::{Singleton, prelude::*};


#[cfg(feature = "serde")]
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(non_upper_case_globals)]
pub(crate) static Assets : LazyLock<AssetsManagerUntyped> = LazyLock::new(|| AssetsManagerUntyped::default());

#[derive(Default)]
pub struct AssetsManagerUntyped
{
    assets: RwLock<HashMap<TypeId, Arc<AnyAsync>>>
}

impl AssetsManagerUntyped
{
    #[inline(always)]
    #[track_caller]
    fn manager<T>(&self) -> AssetManager<T> where T: Async
    {
        let key = std::any::TypeId::of::<T>();
        let arc = self.assets.borrow_mut().entry(key).or_insert_with(|| AssetManager::<T>::new().inner).clone();
        let inner = arc.downcast::<AssetManagerRwLock<T>>().expect("wrong type");
        AssetManager { inner }
    }
}

type AssetManagerRwLock<T> = RwLock<AssetManagerInner<T>>;

pub struct AssetManager<T> where T: Async
{
    inner: Arc<AssetManagerRwLock<T>>
}
impl<T> Clone for AssetManager<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

pub(crate) enum AssetReference<T> where T: Async
{
    Strong(Asset<T>),
    Weak(AssetWeak<T>),
}

pub(crate) struct AssetManagerInner<T> where T: Async
{
    values:  HashMap<Path,AssetReference<T>>,
    loading: AssetWeak<T>,
    error:   AssetWeak<T>,
}
impl<T> Default for AssetManagerInner<T> where T: Async
{
    fn default() -> Self {
        Self { values: ___(), loading: ___(), error: ___() }
    }
}
impl<T> AssetManager<T> where T: Async
{
    pub(crate) fn new() -> Self
    {
        Self { inner: ___() }
    }

    pub fn set_loading_and_error_value(&self, loading_value: &Asset<T>, error_value: &Asset<T>) -> &Self
    {
        let mut b = self.inner.borrow_mut();
        b.loading.inner = Arc::downgrade(&loading_value.inner);
        b.error.inner = Arc::downgrade(&error_value.inner);
        self
    }
    pub fn set_loading_value(&self, loading_value: &Asset<T>) -> &Self
    {
        self.inner.borrow_mut().loading.inner = Arc::downgrade(&loading_value.inner);
        self
    }
    pub fn set_error_value(&self, error_value: &Asset<T>) -> &Self
    {
        self.inner.borrow_mut().error.inner = Arc::downgrade(&error_value.inner);
        self
    }
    pub fn error_value(&self) -> AssetWeak<T>
    {
        self.inner.borrow().error.clone()
    }
    pub fn loading_value(&self) -> AssetWeak<T>
    {
        self.inner.borrow().error.clone()
    }

    pub fn load<P>(&self, path: P) -> Asset<T> where P: AsRef<Path>, T: Load
    {
        let path = path.as_ref();
        let mut w = self.inner.write().unwrap();

        let e = w.values.entry(path.into());
        match &e
        {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => match occupied_entry.get()
            {
                AssetReference::Strong(asset) => return asset.clone(),
                AssetReference::Weak(asset_weak) => match asset_weak.upgrade()
                {
                    Some(asset) => return asset.clone(),
                    None => {},
                },
            },
            _ => {}
        };
        let asset = Asset { inner: Arc::new(RwLock::new(AssetData { state: AssetState::Loading, path: path.into(), manager: self.clone() })) };
        let value = e.or_insert( AssetReference::Weak(asset.downgrade()));
        drop(w);

        asset.force_hot_reload();
        asset
    }
}

// An handle to an asset.
// Asset<T, FetchPolicySecurity, Content/Affect Gameplay?
// Asset<Texture> vs Content<Texture>
pub struct Asset<T> where T: Async
{
    pub(crate) inner: Arc<RwLock<AssetData<T>>>,
}

pub struct AssetWeak<T> where T: Async
{
    pub(crate) inner: Weak<RwLock<AssetData<T>>>,
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
            Some(inner) => Some(Asset { inner }),
            None => None,
        }
    }
}
impl<T> Clone for Asset<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
struct AssetData<T> where T: Async
{
    state: AssetState<T>,
    path: Path,
    manager: AssetManager<T>,
}
impl<T> Asset<T> where T: Async
{
    pub(crate) fn force_hot_reload(&self) -> AssetState<T> where T: Load
    {
        let mut guard = self.inner.write().unwrap();

        std::mem::replace(&mut guard.state, match T::load_from_disk(&guard.path)
        {
            Ok(v) => AssetState::Loaded(Arc::new(v)),
            Err(e) => AssetState::Error(e),
        })
    }
    // pub(crate) fn new_value_load_or_error() -> Self
    // {
    //     Self { inner: Arc::new(RwLock::new(AssetData { state: AssetState::Error(EncodeError::Unimplemented), path: "error".into(), manager: })) }
    // }
    // //pub fn new(value: T, path: Path) ->

    pub(crate) fn manager(&self) -> AssetManager<T>
    {
        self.inner.borrow().manager.clone()
    }

    pub fn downgrade(&self) -> AssetWeak<T>
    {
        AssetWeak { inner: Arc::downgrade(&self.inner) }
    }
}

impl<'a,T> TryBorrow for &'a Asset<T> where T: Async
{
    type Borrow=AssetBorrow<T>;
    type Error=();

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        match &self.inner.read().ok_or(())?.state
        {
            AssetState::Loading => if let AssetState::Loaded(loaded) = &self.manager().inner.read().unwrap().loading.upgrade().ok_or(())?.inner.read().ok_or(())?.state
            {
                return Ok(loaded.clone());
            }
            AssetState::Loaded(val) => return Ok(val.clone()),
            AssetState::Error(encode_error) => if let AssetState::Loaded(error) = &self.manager().inner.read().unwrap().error.upgrade().ok_or(())?.inner.read().ok_or(())?.state
            {
                return Ok(error.clone());
            },
        }
        Err(())
    }
}

type AssetBorrow<T> = Arc<T>;
type AssetBorrowMut<'a,T> = MappedRwLockWriteGuard<'a,T>;

impl<'a,T> TryBorrowMut for &'a Asset<T> where T: Async
{
    type Borrow=AssetBorrowMut<'a,Arc<T>>;
    type Error=();

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        let write_guard = self.inner.write().ok_or(())?;

        if !write_guard.state.is_loaded()
        {
            return Err(());
        }
        let mapped = RwLockWriteGuard::map(write_guard,
            |g|
            g.state.as_loaded_mut().unwrap()
        );
        Ok(mapped)
    }
}


pub(crate) enum AssetState<T> where T: Async
{
    Loading,
    Loaded(Arc<T>),
    Error(IoError),
}
impl<T> From<T> for AssetState<T> where T: Async
{
    fn from(value: T) -> Self {
        Self::Loaded(Arc::new(value))
    }
}
impl<T> AssetState<T> where T: Async
{
    pub const fn is_loading(&self) -> bool { matches!(self, Self::Loading) }
    pub const fn is_loaded(&self) -> bool { matches!(self, Self::Loaded(_)) }
    pub const fn is_error(&self) -> bool { matches!(self, Self::Error(_)) }

    pub fn as_loaded(&self) -> Option<&Arc<T>>
    {
        match self
        {
            Self::Loaded(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_loaded_mut(&mut self) -> Option<&mut Arc<T>>
    {
        match self
        {
            Self::Loaded(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_error(&self) -> Option<&EncodeError>
    {
        match self
        {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }
    pub fn as_error_mut(&mut self) -> Option<&mut EncodeError>
    {
        match self
        {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }

}
