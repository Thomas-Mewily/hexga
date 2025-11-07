#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// #![feature(clone_to_uninit)]

// use std::clone::CloneToUninit;
use std::{any::{Any, TypeId}, collections::HashMap, hash::Hash, iter::FusedIterator, marker::PhantomData, ops::{Deref, DerefMut}, pin::Pin, ptr::NonNull, sync::Arc};

use hexga_core::prelude::*;
use hexga_generational::{gen_vec, multi_map::{self, EntryID}, prelude::*, table};
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_singleton::prelude::*;

pub mod prelude
{
    pub use super::{Asset,Assets,AssetsUntyped};
}


pub struct Assets<T> where T:Async
{
    _marker: PhantomData<T>,
}
impl<T> Debug for Assets<T> where T:Async
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assets<{}>", std::any::type_name::<T>())
    }
}
impl<T> Default for Assets<T> where T:Async
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Clone for Assets<T> where T:Async
{
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> Copy for Assets<T> where T:Async {}
impl<T> Assets<T> where T:Async
{
    pub const fn new() -> Self { Self { _marker: PhantomData }}
}
impl<T> Deref for Assets<T> where T:Async
{
    type Target=AssetManager<T>;
    fn deref(&self) -> &Self::Target {
        AssetsUntyped::as_mut().manager()
    }
}
impl<T> DerefMut for Assets<T> where T:Async
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        AssetsUntyped::as_mut().manager_mut()
    }
}

singleton_declare_multi_thread!(pub AssetsUntyped, AssetManagerUntyped, ASSETS, Default::default());


impl SingletonInit for AssetsUntyped
{
    fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult {
        let cell: &::std::sync::Mutex<Option<AssetManagerUntyped>> =
            ASSETS.get_or_init(|| ::std::sync::Mutex::new(None));

        let mut guard = cell.lock().map_err(|_| ())?;
        if guard.is_some() {
            return Err(()); // The Asset can't be initialized twice
        }
        *guard = value;
        Ok(())
    }
}

#[derive(Default)]
pub struct AssetManagerUntyped
{
    managers: HashMap<TypeId, Pin<Box<dyn AsyncAny>>>
}
impl AssetManagerUntyped
{
    /// Creates a new type-specific asset manager if it does not exist using the [`Default`] value of T for loading and error value.
    pub fn update_or_create_manager<T>(&mut self) -> &mut AssetManager<T> where T: Async + Default
    {
        self.update_or_create_manager_with_values::<T>(Some(T::___()), Some(T::___()))
    }

    /// Creates a new type-specific asset manager if it does not exist without loading and error value.
    pub fn update_or_create_manager_with_none<T>(&mut self) -> &mut AssetManager<T> where T: Async
    {
        self.update_or_create_manager_with_values::<T>(None, None)
    }

    /// or updates the existing manager’s default and error values.
    ///
    /// - If `loading_value` is `Some`, updates the manager's `loading_value`.
    /// - If `error_value` is `Some`, updates the manager's `error_value`.
    pub fn update_or_create_manager_with_values<T>(&mut self, loading_value: Option<T>, error_value: Option<T>) -> &mut AssetManager<T>
        where T: Async
    {
        let type_id = TypeId::of::<T>();

        if self.managers.get_mut(&type_id).is_none()
        {
            self.managers.insert(type_id, Box::pin(AssetManager::<T>::with_none()));
        }

        let pinned_any: &mut Pin<Box<dyn AsyncAny>> = self.managers.get_mut(&type_id).unwrap();

        // SAFELY get a mutable reference to the inner value using unsafe
        // Because Pin guarantees the value won't move, this is safe
        let manager: &mut AssetManager<T> = unsafe {
            let raw: *mut dyn Any = Pin::as_mut(pinned_any).get_unchecked_mut();
            &mut *(raw as *mut AssetManager<T>)
        };

        // Update default and error values
        if loading_value.is_some() {
            manager.loading_value = loading_value;
        }
        if error_value.is_some() {
            manager.error_value = error_value;
        }
        manager
    }

    pub fn manager<T>(&mut self) -> &AssetManager<T>
        where T: Async
    {
        self.update_or_create_manager_with_none::<T>()
    }

    pub fn manager_mut<T>(&mut self) -> &mut AssetManager<T>
        where T: Async
    {
        self.update_or_create_manager_with_none::<T>()
    }
}


#[derive(Debug)]
pub struct AssetData<T:Async>
{
    state: AssetState<T>,
    manager_ptr : NonNull<AssetManager<T>>,
    id : TableID,
    count: usize,
}
unsafe impl<T: Async> Send for AssetData<T> {}
unsafe impl<T: Async> Sync for AssetData<T> {}

impl<T> AssetData<Arc<T>> where T: Async
{
    pub fn try_as_arc(&self) -> Option<Arc<T>> { self.as_loading().cloned() }

    /// Panic if the asset is not loaded.
    ///
    /// Use [`Self::try_as_arc`] for a non panicking version.
    #[inline(always)]
    #[track_caller]
    pub fn as_arc(&self) -> Arc<T>
    {
        self.try_as_arc().expect("AssetData::as_arc(): The asset wasn't loaded")
    }
}


impl<T> AssetData<T> where T: Async
{
    pub fn is_loading(&self) -> bool { self.state.is_loading() }
    pub fn is_loaded(&self) -> bool { self.state.is_loaded() }
    pub fn is_not_loaded(&self) -> bool { !self.is_loaded() }
    pub fn is_error(&self) -> bool { self.state.is_error() }

    pub fn as_error(&self) -> Option<&IoError>
    {
        if let AssetState::Error(err) = &self.state { Some(err) } else { None }
    }
    /// Use [`Self::try_value`] or [`Self::value`] to read the value, since it can fallback to an loading or error value.
    pub fn as_loading(&self) -> Option<&T>
    {
        if let AssetState::Loaded(val) = &self.state { Some(val) } else { None }
    }

    #[inline(always)]
    #[track_caller]
    pub fn value(&self) -> &T
    {
        match self.try_value()
        {
            Some(v) => v,
            None =>
            {
                panic!(
                    "Attempted immutable access to AssetData<{}> but the value is not loaded and the AssetManager don't have any fallback value. Call `update_or_create_manager_with_values()` to configure the AssetManager.",
                    std::any::type_name::<T>()
                )
            },
        }
    }
    pub fn try_value(&self) -> Option<&T>
    {
        match &self.state
        {
            AssetState::Loading(dyn_future) => self.asset_manager().loading_value.as_ref(),
            AssetState::Loaded(value) => Some(value),
            AssetState::Error(_) => self.asset_manager().error_value.as_ref(),
        }
    }

    /// Set the asset in a loaded state with the value, and return the old value if the asset was previously loaded.
    pub fn replace_value(&mut self, value: T) -> Option<T>
    {
        match &mut self.state
        {
            AssetState::Loaded(old_value) => Some(std::mem::replace(old_value, value)),
            _ => { self.state = AssetState::Loaded(value); None },
        }
    }

    /// Returns a shared reference to the manager
    pub(crate) fn asset_manager(&self) -> &AssetManager<T> {
        // SAFETY: manager_ptr points to a valid AssetManagerOf<T>
        // and the lifetime of self guarantees it lives as long as the manager
        unsafe { self.manager_ptr.as_ref() }
    }

    pub(crate) fn asset_manager_mut(&mut self) -> &mut AssetManager<T> {
        // SAFETY: manager_ptr points to a valid AssetManagerOf<T>
        // and the lifetime of self guarantees it lives as long as the manager
        unsafe { self.manager_ptr.as_mut() }
    }

    pub fn path(&self) -> &path
    {
        path::from_str(self.asset_manager().assets.get_entry(self.id).unwrap().main_key())
    }

    pub fn keys(&self) -> &[String]
    {
        self.asset_manager().assets.get_entry(self.id).unwrap().keys()
    }

    pub fn save(&mut self) -> IoResult
        where T: Save
    {
        match self.try_value()
        {
            Some(val) => val.save_to_disk(self.path()),
            None => Err(IoError::new(self.path(), EncodeError::custom("The value wasn't loaded")).when_writing()),
        }
    }

    /// Hot reload the value, and return the old one if it was loaded
    pub fn hot_reload(&mut self) -> IoResult<Option<T>>
        where T: Load
    {
        let value = self.load_without_update()?;
        match &mut self.state
        {
            AssetState::Loaded(old_value) => Ok(Some(std::mem::replace(old_value, value))),
            _ =>
            {
                self.state = AssetState::Loaded(value);
                Ok(None)
            },
        }
    }

    /// Load the value from the file and return it, without updating the asset
    pub fn load_without_update(&self) -> IoResult<T>
        where T: Load
    {
        T::load_from_disk(self.path())
    }

    /// This method is expensive to call since it will compare the deserialized version of the file with the current value
    pub fn have_modification_from_io(&self) -> bool
        where T: Load + PartialEq
    {
        if self.is_not_loaded() { return false; }
        match self.load_without_update()
        {
            Ok(value) => self.try_value() == Some(&value),
            Err(_) => true,
        }
    }
}
impl<T> Deref for AssetData<T>
    where T: Async
{
    type Target=T;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target
    {
        self.value()
    }
}


#[derive(Debug)]
enum AssetState<T:Async>
{
    Loading(()), // DynFuture<T>
    Loaded(T),
    Error(IoError),
}
impl<T> AssetState<T> where T:Async
{
    pub fn is_loading(&self) -> bool { matches!(self, Self::Loading(_)) }
    pub fn is_loaded(&self) -> bool { matches!(self, Self::Loaded(_)) }
    pub fn is_error(&self) -> bool { matches!(self, Self::Error(_)) }
}


#[derive(Default)]
pub struct AssetManager<T>
    where T: Async
{
    assets : Table<AssetData<T>>,
    loading_value: Option<T>,
    error_value: Option<T>,
}
impl<T> AssetManager<T>
    where T: Async
{
    pub fn with_loading_values() -> Self where T: Default { Self::with_values(___(), ___())}
    pub fn with_none() -> Self { Self::with_values(None, None) }
    pub fn with_values(loading_value: Option<T>, error_value: Option<T>) -> Self
    {
        Self { assets: ___(), loading_value, error_value }
    }
}

impl<'a,T> IntoIterator for &'a AssetManager<T>
    where T: Async
{
    type Item=&'a AssetData<T>;
    type IntoIter=Iter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
         Iter{ iter: self.assets.iter() }
    }
}
#[derive(Debug)]
pub struct Iter<'a,T>
    where T: Async
{
    iter: table::Iter<'a,AssetData<T>>
}
impl<'a,T> Clone for Iter<'a,T> where T: Async
{
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone() }
    }
}

impl<'a,T> Iterator for Iter<'a,T>
    where T: Async
{
    type Item=&'a AssetData<T>;
    fn next(&mut self) -> Option<Self::Item>
    {
        let (_entry,value) = self.iter.next()?;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a,T> FusedIterator for Iter<'a,T> where T: Async, table::Iter<'a,AssetData<T>>: FusedIterator  {}
impl<'a,T> ExactSizeIterator for Iter<'a,T> where T: Async, table::Iter<'a,AssetData<T>>: ExactSizeIterator { fn len(&self) -> usize { self.iter.len() } }


impl<T> AssetManager<T>
    where T: Async
{
    pub(crate) fn get_or_init_with_state<P,F>(&mut self, path: P, create_state: F) -> Asset<T>
        where P: AsRefPath, F: FnOnce(&path) -> AssetState<T>
    {
        let path = path.as_ref();
        if let Some(asset) = self.assets.get_from_key(path.as_str())
        {
            return Asset::from_asset_data(asset);
        }

        let data = AssetData
        {
            state: create_state(path),
            manager_ptr: unsafe { NonNull::new_unchecked(self as *mut Self) },
            id: TableID::NULL,
            count: 0,
        };
        let id = self.assets.insert(path.to_owned().into(), data).unwrap();
        self.assets[id].id = id;
        Asset::from_asset_data(&self.assets[id])
    }

    /// Sets the asset at `path` to an error state, creating it if needed.
    ///
    /// Returns a handle to the resulting asset.
    pub fn update_or_create_with_error<P>(&mut self, path: P, error: IoError) -> Asset<T>
        where P: AsRefPath, T: Load
    {
        let path = path.as_ref();
        match self.assets.get_mut_from_key(path.as_str())
        {
            Some(asset) =>
            {
                asset.state = AssetState::Error(error);
                Asset::from_asset_data(asset)
            },
            None => self.get_or_init_with_state(path, |path| AssetState::Error(error)),
        }
    }


    /// Sets the asset at `path` to the given value, and create it if needed.
    ///
    /// Returns a handle to the resulting asset.
    pub fn update_or_create_with_value<P>(&mut self, path: P, value: T) -> Asset<T>
        where P: AsRefPath, T: Load
    {
        let path = path.as_ref();
        match self.assets.get_mut_from_key(path.as_str())
        {
            Some(asset) =>
            {
                asset.replace_value(value);
                Asset::from_asset_data(asset)
            },
            None => self.get_or_init_with_state(path, |path| AssetState::Loaded(value)),
        }
    }

    pub fn get_or_load<P>(&mut self, path: P) -> Asset<T>
        where P: AsRefPath, T: Load
    {
        let mut was_loaded_from_file = false;
        let mut asset = self.get_or_init_with_state(path,
            |path|
            {
                was_loaded_from_file = true;
                match T::load_from_disk(path)
                {
                    Ok(asset) => AssetState::Loaded(asset),
                    Err(err) => AssetState::Error(err),
                }
            }
        );
        if !was_loaded_from_file // avoid trying to load the asset twice
            && asset.is_error()
        {
            let _ = asset.hot_reload();
        }
        asset
    }

    pub fn iter(&self) -> Iter<'_,T> { (&self).into_iter() }


    pub fn loading_value(&self) -> Option<&T> { self.loading_value.as_ref() }
    pub fn error_value(&self) -> Option<&T> { self.error_value.as_ref() }

    pub fn set_loading_and_error_value(&mut self, loading_value: T, error_value: T) -> &mut Self { self.set_loading_value(loading_value).set_error_value(error_value) }
    pub fn set_loading_value(&mut self, value: T) -> &mut Self { self.loading_value = Some(value); self }
    pub fn set_error_value(&mut self, value: T) -> &mut Self { self.error_value = Some(value); self }
}

#[derive(Clone)]
pub struct Asset<T>
    where T: Async
{
    id : TableID,
    manager_ptr : NonNull<AssetManager<T>>
}
impl<T> PartialEq for Asset<T> where T: Async
{
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<T> Eq for Asset<T> where T: Async {}
impl<T> Hash for Asset<T> where T: Async
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl<T> Asset<T> where T: Async
{
    pub fn load(path: &path) -> Self
        where T: Load
    {
        AssetsUntyped.manager_mut::<T>().get_or_load(path)
    }
    pub fn new(path: &path, value: T) -> Self
    {
        AssetsUntyped.manager_mut::<T>().get_or_init_with_state(path, |path| AssetState::Loaded(value))
    }
    pub fn with_error(path: &path, error: IoError) -> Self
    {
        AssetsUntyped.manager_mut::<T>().get_or_init_with_state(path, |path| AssetState::Error(error))
    }

    pub fn from_asset_data(asset: &AssetData<T>) -> Self
    {
        let mut s = Self { id: asset.id, manager_ptr: asset.manager_ptr };
        s.count += 1;
        s
    }
}
impl<T> Drop for Asset<T>
    where T:Async
{
    fn drop(&mut self) {
        debug_assert_ne!(self.count, 0);
        self.count -= 1;
        if self.count == 0
        {
            let manager = &mut unsafe { self.manager_ptr.as_mut() };
            let id = self.id;
            let value = manager.assets.remove(id);
            assert!(value.is_some());
        }
    }
}

impl<T> Deref for Asset<T> where T: Async
{
    type Target=AssetData<T>;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.manager_ptr.as_ref() }.assets[self.id]
    }
}
impl<T> DerefMut for Asset<T> where T: Async
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut unsafe { self.manager_ptr.as_mut() }.assets[self.id]
    }
}


// pub struct AssetUntyped
// {
//     type_id: std::any::TypeId,
//     id: TableID,
// }