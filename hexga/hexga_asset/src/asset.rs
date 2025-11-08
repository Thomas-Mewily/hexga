use super::*;


// Idea: add an Autosave flag (when dropped) ?


#[derive(Debug)]
pub struct AssetData<T:Async>
{
    pub(crate) state: AssetState<T>,
    pub(crate) manager_ptr : NonNull<AssetManager<T>>,
    pub(crate) id : TableID,
    pub(crate) count: usize,
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
                    "AssetData<{}> was not loaded and the AssetManager don't have any fallback value. Call `Assets::<{}>::new().set_loading_and_error_value(<loading_value>, <error_value>)` to set them.",
                    std::any::type_name::<T>(),
                    std::any::type_name::<T>(),
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

    pub fn path(&self) -> Option<&path>
    {
        let path = self.path_or_empty();
        if path.is_empty() { None } else { Some(path) }
    }
    pub fn path_or_empty(&self) -> &path
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
        // Todo: save it in a temporary file ?
        let Some(path) = self.path() else { return Err(IoError::new(self.path_or_empty(), FileError::custom("Asset didn't have a path"))); };
        match self.try_value()
        {
            Some(val) => val.save_to_disk(self.path_or_empty()),
            None => Err(IoError::new(self.path_or_empty(), EncodeError::custom("The value wasn't loaded")).when_writing()),
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
        let Some(path) = self.path() else { return Err(IoError::new(self.path_or_empty(), FileError::custom("Asset didn't have a path"))); };
        T::load_from_disk(self.path_or_empty())
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
pub(crate) enum AssetState<T:Async>
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



pub struct Asset<T>
    where T: Async
{
    id : TableID,
    manager_ptr : NonNull<AssetManager<T>>
}
impl<T> Clone for Asset<T> where T: Async
{
    fn clone(&self) -> Self {
        Self::from_asset_data(self)
    }
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
    pub fn load<P>(path: P) -> Self
        where T: Load, P: AsRefPath
    {
        AssetsUntyped.manager_mut::<T>().get_or_load(path)
    }

    pub fn with_path<P>(&mut self, path: Path) -> &mut Self where P: AsRefPath
    {
        // Change the path, and change the key
        todo!();
        self

    }
    pub fn load_from_value<P>(path: P, value: T) -> Self
        where P: AsRefPath
    {
        AssetsUntyped.manager_mut::<T>().update_or_create_with_value(path, value)
    }
    pub fn load_from_error<P>(path: P, error: IoError) -> Self
        where P: AsRefPath
    {
        AssetsUntyped.manager_mut::<T>().update_or_create_with_error(path, error)
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
