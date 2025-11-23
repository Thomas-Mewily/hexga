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

    pub fn is_loaded(&self) -> bool { self.get().is_some() }
    pub fn is_not_loaded(&self) -> bool { !self.is_loaded() }

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
    /// Attempts to load the asset from the specified path. If the asset doesn't exist
    /// or fails to load, creates a new asset using the provided initialization function.
    pub fn load_or_create<P,F,O>(path: P, init: F) -> Asset<T>
            where
            P: AsRef<Path>,
            F: FnOnce() -> O + 'static,
            O: Into<AssetInit<T>>,
            T: Load
    {
        Self::manager().load_or_create(path, init)
    }
    pub fn update_or_create<P,Persis,I>(persistance: Persis, value: I) -> Asset<T>
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        I: Into<AssetInit<T>>
    {
        Self::manager().update_or_create(persistance, value)
    }

    pub fn get_or_generate<P,Persis,F,O>(persistance: Persis, init: F) -> Asset<T>
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        F: FnOnce() -> O,
        O: Into<AssetInit<T>>
    {
        Self::manager().get_or_generate(persistance, init)
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
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