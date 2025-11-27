use super::*;



pub struct AssetReadGuard<'a,T>
{
    guard: MappedRwLockReadGuard<'a,T>,
}
impl<'a,T> Deref for AssetReadGuard<'a,T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.guard.deref() }
}
pub struct AssetWriteGuard<'a,T>
{
    guard: MappedRwLockWriteGuard<'a,T>
}
impl<'a,T> Deref for AssetWriteGuard<'a,T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.guard.deref() }
}
impl<'a,T> DerefMut for AssetWriteGuard<'a,T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.guard.deref_mut() }
}


#[derive(Debug)]
pub struct Asset<T> where T: Async
{
    pub(crate) inner: Arc<RwLock<AssetData<T>>>,
}
impl<T> Clone for Asset<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

#[cfg(feature = "serde")]
mod serde_impl
{
    use super::*;
    type A<T> = super::Asset<T>;

    mod ser_impl
    {
        use super::*;

        #[derive(Serialize)]
        pub(crate) enum Asset<'a,T> where T: Serialize + Save
        {
            Path(&'a Path),
            Value(&'a T)
        }

        impl<T> Serialize for A<T> where T: Async + Serialize + Save
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
            {
                let r = self.inner.read().unwrap();
                let asset_rep = match &r.persistance
                {
                    AssetPersistance::Persistant(path) => Asset::Path(path.as_path()),
                    AssetPersistance::Generated => match &r.state
                    {
                        AssetState::Loaded(value) => Asset::Value(value),
                        _ => return Err(serde::ser::Error::custom("")),
                    },
                };
                asset_rep.serialize(serializer)
            }
        }
    }

    mod de_impl
    {
        use super::*;

        #[derive(Deserialize)]
        pub(crate) enum Asset<T> where T: Async
        {
            Path(PathBuf),
            Value(T)
        }

        #[cfg(feature = "serde")]
        impl<'de, T> Deserialize<'de> for A<T> where T: Async + Load
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match Asset::deserialize(deserializer)?
                {
                    Asset::Path(path_buf) => Ok(A::load(path_buf)),
                    Asset::Value(value) => Ok(A::update_or_create(AssetPersistance::Generated, AssetInit::new(value))),
                }
            }
        }
    }
}


impl<T> Asset<T> where T: Async
{
    pub(crate) fn _new(state: AssetState<T>, persistance: AssetPersistance) -> Self
    {
        Self { inner: Arc::new(RwLock::new(AssetData { state, persistance })) }
    }

    pub fn ptr_eq(&self, other: &Asset<T>) -> bool
    {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
    pub fn strong_count(&self) -> usize
    {
        Arc::strong_count(&self.inner)
    }
    pub fn weak_count(&self) -> usize
    {
        Arc::weak_count(&self.inner)
    }

    /// Returns a reference to the asset value only if it is in a loaded state.
    ///
    /// Unlike [`Self::get_or_placeholder`], this method never
    /// returns placeholder values for loading and error states.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let asset = manager.load("texture.png");
    ///
    /// if let Some(texture) = asset.get() {
    ///     draw_texture(&texture);
    /// } else {
    ///     loading_cursor();
    /// }
    /// ```
    pub fn get(&self) -> Option<AssetReadGuard<'_,T>>
    {
        let guard = self.inner.read().unwrap();
        match &guard.state
        {
            AssetState::Loaded(_) => Some(AssetReadGuard{ guard:  RwLockReadGuard::map(guard, |g| g.state.as_loaded().unwrap())}),
            _ => None,
        }
    }

    pub fn state(&self) -> AssetReadGuard<'_, AssetState<T>>
    {
        AssetReadGuard{ guard: RwLockReadGuard::map(self.inner.read().unwrap(), |r| &r.state) }
    }
    pub fn state_mut(&self) -> AssetWriteGuard<'_, AssetState<T>>
    {
        AssetWriteGuard{ guard: RwLockWriteGuard::map(self.inner.write().unwrap(), |r| &mut r.state) }
    }

    /// Returns a mutable reference to the asset value only if it is in a loaded state.
    pub fn get_mut(&mut self) -> Option<AssetWriteGuard<'_,T>>
    {
        let guard = self.inner.write().unwrap();
        match &guard.state
        {
            AssetState::Loaded(_) => Some(AssetWriteGuard{ guard: RwLockWriteGuard::map(guard, |g| g.state.as_loaded_mut().unwrap())}),
            _ => None,
        }
    }

    /// Returns a reference to the asset value, using placeholder values for loading or error states if available.
    ///
    /// For access to only loaded assets, use [`Self::get`].
    pub fn get_or_placeholder<'a>(&'a self) -> Option<AssetReadGuard<'a,T>>
    {
        let guard = self.inner.read().unwrap();
        let guard = match &guard.state
        {
            AssetState::Loading =>
            {
                let manager = Self::manager();
                let manager_guard = manager.inner.read().unwrap();
                let loading_asset = manager_guard.loading.upgrade()?;

                // SAFETY: The AssetManager<T>, once created, lives for the entire program duration.
                // The RwLockReadGuard is tied to data owned by the eternal manager, so extending the lifetime is safe.
                unsafe {
                    std::mem::transmute::<
                        RwLockReadGuard<'_, AssetData<T>>,
                        RwLockReadGuard<'a, AssetData<T>>
                    >(loading_asset.inner.read().unwrap())
                }
            },
            AssetState::Loaded(_) => guard,
            AssetState::Error(_) =>
            {
                let manager = Self::manager();
                let manager_guard = manager.inner.read().unwrap();
                let loading_asset = manager_guard.error.upgrade()?;

                // SAFETY: The AssetManager<T>, once created, lives for the entire program duration.
                // The RwLockReadGuard is tied to data owned by the eternal manager, so extending the lifetime is safe.
                unsafe {
                    std::mem::transmute::<
                        RwLockReadGuard<'_, AssetData<T>>,
                        RwLockReadGuard<'a, AssetData<T>>
                    >(loading_asset.inner.read().unwrap())
                }
            },
        };
        Some(AssetReadGuard{ guard:  RwLockReadGuard::map(guard, |g| g.state.as_loaded().unwrap())})
    }

    pub fn replace_state(&mut self, state: AssetState<T>) -> AssetState<T>
    {
        std::mem::replace(&mut self.inner.write().unwrap().state, state)
    }
    pub fn replace(&mut self, value: T) -> AssetState<T>
    {
        self.replace_state(AssetState::Loaded(value))
    }

    /// Create a weak reference that won't keep the asset loaded
    pub fn downgrade(&self) -> AssetWeak<T>
    {
        AssetWeak{ inner: Arc::downgrade(&self.inner) }
    }

    pub fn lifetime(&self) -> AssetLifetime
    {
        match &self.inner.read().unwrap().persistance
        {
            AssetPersistance::Persistant(path) => Self::manager().inner.read().unwrap().values[path].lifetime(),
            AssetPersistance::Generated => AssetLifetime::ReferenceCounted,
        }
    }

    pub fn set_lifetime(&mut self, lifetime: AssetLifetime) -> &mut Self
    {
        let r = self.inner.read().unwrap();
        let p = &r.persistance;

        match p
        {
            AssetPersistance::Persistant(path) =>
            {
                let manager = Self::manager();
                let r = manager.inner.read().unwrap();

                if r.values[path].lifetime() != lifetime
                {
                    drop(r);
                    let mut w = manager.inner.write().unwrap();
                    let e = w.values.get_mut(path).unwrap();
                    match e
                    {
                        AssetStorage::Persistant(asset) => { *e = AssetStorage::ReferenceCounted(asset.downgrade()); },
                        AssetStorage::ReferenceCounted(asset_weak) => { *e = AssetStorage::Persistant(asset_weak.upgrade().unwrap() /* The current asset have an arc to the value, so the weak one is still alive */); },
                    }
                }
            },
            AssetPersistance::Generated => {},
        };
        drop(r);
        self
    }

    pub fn manager() -> AssetManager<T>
    {
        Assets.manager()
    }


    pub fn is_loading(&self) -> bool { self.state().is_loading() }
    pub fn is_loaded(&self) -> bool { self.state().is_loaded() }
    pub fn is_not_loaded(&self) -> bool { self.state().is_not_loaded() }
    pub fn is_error(&self) -> bool { self.state().is_error() }

    pub fn persistance(&self) -> AssetReadGuard<'_, AssetPersistance>
    {
        AssetReadGuard{ guard: RwLockReadGuard::map(self.inner.read().unwrap(), |r| &r.persistance) }
    }
    pub fn set_persistance(&self, persistance: AssetPersistance) -> &Self
    {
        let r = self.persistance();
        if r.deref() == &persistance { return self; }
        drop(r);

        match persistance
        {
            AssetPersistance::Persistant(path) =>
            {
                let weak = self.downgrade();
                let mut w = self.inner.write().unwrap();
                let manager = Self::manager();
                let mut manager_w = manager.inner.write().unwrap();
                w.persistance = AssetPersistance::Persistant(path.clone());
                let old = manager_w.values.insert(path, AssetStorage::ReferenceCounted(weak));
                assert!(old.is_none());
            },
            AssetPersistance::Generated =>
            {
                let mut w = self.inner.write().unwrap();
                let manager = Self::manager();
                let mut manager_w = manager.inner.write().unwrap();

                let path = {
                    let path = std::mem::replace(&mut w.persistance, AssetPersistance::Generated);
                    match path
                    {
                        AssetPersistance::Persistant(path) => path,
                        AssetPersistance::Generated => unreachable!(),
                    }
                };
                let old = manager_w.values.remove(&path);
                assert!(old.is_some())
            },
        }
        self
    }
    pub fn is_persistant(&self) -> bool { self.persistance().is_persistant() }
    pub fn is_not_persistant(&self) -> bool { self.persistance().is_not_persistant() }

    pub fn path(&self) -> Option<PathBuf>
    {
        match self.persistance().deref()
        {
            AssetPersistance::Persistant(p) => Some(p.clone()),
            AssetPersistance::Generated => None,
        }
    }
}
impl<T> Asset<T> where T: Async
{
    /// Loads an asset from the specified file path.
    ///
    /// If the asset was previously loaded but encountered an error, this will attempt
    /// to reload.
    ///
    /// If the asset is already successfully loaded, returns the existing asset instance.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let texture = manager.load("textures/character");
    /// ```
    pub fn load<P>(path: P) -> Asset<T> where P: AsRef<Path>, T: Load { Self::manager().load(path) }


    /// Loads an asset or creates it if it doesn't exist.
    ///
    /// If the file exist and any error occur while reading it,
    /// the asset will be in an error state. (To avoid overriding a file)
    pub fn load_or_create<P,F,O>(path: P, init: F) -> Asset<T>
            where
            P: AsRef<Path>,
            F: FnOnce() -> O + 'static,
            O: Into<AssetInit<T>>,
            T: Load
    {
        Self::manager().load_or_create(path, init)
    }
    pub fn update_or_create<'a,P,I>(persistance: P, value: I) -> Asset<T>
        where
        P: Into<AssetPersistance<&'a Path>>,
        I: Into<AssetInit<T>>
    {
        Self::manager().update_or_create(persistance, value)
    }

    pub fn get_or_generate<'a,P,F,O>(persistance: P, init: F) -> Asset<T>
        where
        P: Into<AssetPersistance<&'a Path>>,
        F: FnOnce() -> O,
        O: Into<AssetInit<T>>
    {
        Self::manager().get_or_generate(persistance, init)
    }

    /// Save the asset if it is Persistant
    pub fn save(&mut self) -> IoResult where T: Save
    {
        let r = self.inner.read().unwrap();
        match &r.persistance
        {
            AssetPersistance::Persistant(path) =>
            {
                match &r.state
                {
                    AssetState::Loading => Err(IoError::new(path, EncodeError::NotLoaded).when_writing()),
                    AssetState::Loaded(value) => value.save(path),
                    AssetState::Error(io_error) => Err(io_error.clone()),
                }
            },
            AssetPersistance::Generated => Err(IoError::new("", EncodeError::NotPersistant).when_writing()),
        }
    }

    /// Hot reload the value, and return the old state.
    ///
    /// If the asset is not persistant, return Ok.
    pub fn hot_reload(&mut self) -> IoResult<Option<AssetState<T>>>
        where T: Load
    {
        if self.is_persistant()
        {
            let value = self.load_from_source();
            Ok(Some(std::mem::replace(self.state_mut().deref_mut(), value.into())))
        }else
        {
            Ok(None)
        }
    }

    /// Load the value from the file and return it, without updating the asset.
    ///
    /// If the asset is not persistant, return an error.
    pub fn load_from_source(&self) -> IoResult<T>
        where T: Load
    {
        match self.persistance().deref()
        {
            AssetPersistance::Persistant(path) => T::load(path),
            AssetPersistance::Generated => Err(IoError::new("", EncodeError::NotPersistant)),
        }
    }
}

#[derive(Debug)]
pub struct AssetWeak<T> where T: Async
{
    inner: Weak<RwLock<AssetData<T>>>,
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

    pub fn ptr_eq(&self, other: &AssetWeak<T>) -> bool
    {
        Weak::ptr_eq(&self.inner, &other.inner)
    }
    pub fn strong_count(&self) -> usize
    {
        Weak::strong_count(&self.inner)
    }
    pub fn weak_count(&self) -> usize
    {
        Weak::weak_count(&self.inner)
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct AssetData<T> where T: Async
{
    pub(crate) state: AssetState<T>,
    pub(crate) persistance: AssetPersistance,
}

impl<T> Drop for AssetData<T>  where T: Async
{
    fn drop(&mut self)
    {
        match &self.persistance
        {
            AssetPersistance::Persistant(path) =>
            {
                Asset::<T>::manager().inner.write().unwrap().values.remove(path);
            },
            AssetPersistance::Generated => {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssetPersistance<P=PathBuf>
{
    Persistant(P),
    Generated,
}
impl AssetPersistance
{
    pub const fn is_generated(&self) -> bool { matches!(self, Self::Generated) }
    pub const fn is_not_generated(&self) -> bool { !self.is_generated() }

    pub const fn is_persistant(&self) -> bool { matches!(self, Self::Persistant(_)) }
    pub const fn is_not_persistant(&self) -> bool { !self.is_persistant() }
}
impl<P> From<P> for AssetPersistance<PathBuf> where P: Into<PathBuf>
{
    fn from(value: P) -> Self {
        Self::Persistant(value.into())
    }
}
impl<'a,P> From<&'a P> for AssetPersistance<&'a Path> where P: AsRef<Path>
{
    fn from(value: &'a P) -> Self {
        Self::Persistant(value.as_ref())
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssetState<T> where T: Async
{
    Loading,
    Loaded(T),
    Error(IoError),
}
impl<T> From<IoResult<T>> for AssetState<T> where T: Async
{
    fn from(value: IoResult<T>) -> Self {
        match value
        {
            Ok(loaded) => Self::Loaded(loaded),
            Err(error) => Self::Error(error),
        }
    }
}
impl<T> AssetState<T> where T: Async
{
    pub const fn is_loading(&self) -> bool { matches!(self, Self::Loading) }
    pub const fn is_loaded(&self) -> bool { matches!(self, Self::Loaded(_)) }
    pub const fn is_not_loaded(&self) -> bool { !self.is_loaded() }
    pub const fn is_error(&self) -> bool { matches!(self, Self::Error(_)) }

    pub fn as_loaded(&self) -> Option<&T>
    {
        match self
        {
            Self::Loaded(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_loaded_mut(&mut self) -> Option<&mut T>
    {
        match self
        {
            Self::Loaded(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_error(&self) -> Option<&IoError>
    {
        match self
        {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }
    pub fn as_error_mut(&mut self) -> Option<&mut IoError>
    {
        match self
        {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AssetLifetime
{
    #[default]
    ReferenceCounted,
    Persistant,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssetInit<T> where T: Async
{
    pub state : AssetState<T>,
    /// Note, if the asset is generated, then it will always have a ReferenceCounted lifetime.
    pub lifetime: AssetLifetime,
}
impl<T> AssetInit<T> where T: Async
{
    pub const fn with_state(state: AssetState<T>) -> Self
    {
        Self { state, lifetime: AssetLifetime::ReferenceCounted }
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
impl<T> From<AssetState<T>> for AssetInit<T> where T: Async
{
    fn from(value: AssetState<T>) -> Self {
        Self::with_state(value)
    }
}

/*
pub struct PathData<T,P=PathBuf>
{
    pub path: P,
    pub data: T,
}
*/