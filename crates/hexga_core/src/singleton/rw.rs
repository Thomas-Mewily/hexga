use super::*;

#[cfg(feature = "feature_mapped_lock_guards")]
fn map_identity_rw_read_guard<'a, T>(guard: RwLockReadGuard<'a, Identity<T>>) -> MappedRwLockReadGuard<'a, T>
{
    RwLockReadGuard::map(guard, |identity| &identity.value)
}

#[cfg(not(feature = "feature_mapped_lock_guards"))]
fn map_identity_rw_read_guard<'a, T>(guard: RwLockReadGuard<'a, Identity<T>>) -> DoubleDeref<RwLockReadGuard<'a, Identity<T>>> { DoubleDeref::from(guard) }

#[cfg(feature = "feature_mapped_lock_guards")]
fn map_identity_rw_write_guard<'a, T>(guard: RwLockWriteGuard<'a, Identity<T>>) -> MappedRwLockWriteGuard<'a, T>
{
    RwLockWriteGuard::map(guard, |identity| &mut identity.value)
}

#[cfg(not(feature = "feature_mapped_lock_guards"))]
fn map_identity_rw_write_guard<'a, T>(guard: RwLockWriteGuard<'a, Identity<T>>) -> DoubleDeref<RwLockWriteGuard<'a, Identity<T>>> { DoubleDeref::from(guard) }

#[cfg(feature = "feature_mapped_lock_guards")]
fn map_lazy_rw_read_guard<'a, T>(guard: RwLockReadGuard<'a, LazyLock<T>>) -> MappedRwLockReadGuard<'a, T> { RwLockReadGuard::map(guard, |lazy| &**lazy) }

#[cfg(not(feature = "feature_mapped_lock_guards"))]
fn map_lazy_rw_read_guard<'a, T>(guard: RwLockReadGuard<'a, LazyLock<T>>) -> DoubleDeref<RwLockReadGuard<'a, LazyLock<T>>> { DoubleDeref::from(guard) }

#[cfg(feature = "feature_mapped_lock_guards")]
fn map_lazy_rw_write_guard<'a, T>(guard: RwLockWriteGuard<'a, LazyLock<T>>) -> MappedRwLockWriteGuard<'a, T>
{
    RwLockWriteGuard::map(guard, |lazy| &mut **lazy)
}

#[cfg(not(feature = "feature_mapped_lock_guards"))]
fn map_lazy_rw_write_guard<'a, T>(guard: RwLockWriteGuard<'a, LazyLock<T>>) -> DoubleDeref<RwLockWriteGuard<'a, LazyLock<T>>> { DoubleDeref::from(guard) }

#[cfg(feature = "feature_mapped_lock_guards")]
fn map_option_rw_read_guard<'a, T>(guard: RwLockReadGuard<'a, Option<T>>) -> MappedRwLockReadGuard<'a, T>
{
    RwLockReadGuard::map(guard, |value| value.as_ref().expect("should not be none"))
}

#[cfg(not(feature = "feature_mapped_lock_guards"))]
fn map_option_rw_read_guard<'a, T>(guard: RwLockReadGuard<'a, Option<T>>) -> DoubleDerefOption<RwLockReadGuard<'a, Option<T>>, T>
{
    DoubleDerefOption::from(guard)
}

#[cfg(feature = "feature_mapped_lock_guards")]
fn map_option_rw_write_guard<'a, T>(guard: RwLockWriteGuard<'a, Option<T>>) -> MappedRwLockWriteGuard<'a, T>
{
    RwLockWriteGuard::map(guard, |value| value.as_mut().expect("should not be none"))
}

#[cfg(not(feature = "feature_mapped_lock_guards"))]
fn map_option_rw_write_guard<'a, T>(guard: RwLockWriteGuard<'a, Option<T>>) -> DoubleDerefOption<RwLockWriteGuard<'a, Option<T>>, T>
{
    DoubleDerefOption::from(guard)
}

/// Multi threaded singleton, where the value is initialized at compile time.
pub type SingletonRw<T> = SingletonOf<RwLock<Identity<T>>>;
/// Multi threaded singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonLazyRw<T> = SingletonOf<RwLock<LazyLock<T>>>;
/// Multi threaded singleton, where the value should be manually initialized at runtime.
pub type SingletonOptionRw<T> = SingletonOf<RwLock<Option<T>>>;

// Wasm is single threaded right now, so this is ok.
#[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
mod single_thread
{
    use super::*;
    unsafe impl<T> Sync for SingletonRw<T> {}
    unsafe impl<T> Send for SingletonRw<T> {}

    unsafe impl<T> Sync for SingletonLazyRw<T> {}
    unsafe impl<T> Send for SingletonLazyRw<T> {}

    unsafe impl<T> Sync for SingletonOptionRw<T> {}
    unsafe impl<T> Send for SingletonOptionRw<T> {}
}

impl<T> SingletonRw<T>
{
    pub const fn new(value: T) -> Self
    {
        Self {
            guarded: RwLock::new(Identity::new(value)),
        }
    }
}

impl<T> Debug for SingletonRw<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get()
        {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "Singleton<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonRw<T>
{
    #[cfg(feature = "feature_mapped_lock_guards")]
    type Guard<'a>
        = MappedRwLockReadGuard<'a, T>
    where
        Self: 'a;
    #[cfg(not(feature = "feature_mapped_lock_guards"))]
    type Guard<'a>
        = DoubleDeref<RwLockReadGuard<'a, Identity<T>>>
    where
        Self: 'a;
    type Error<'a>
        = PoisonError<RwLockReadGuard<'a, Identity<T>>>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        match self.try_get()
        {
            Ok(guard) => guard,
            Err(e) => panic!("SingletonRw<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.guarded.read().map(map_identity_rw_read_guard) }
}

impl<T> GuardedMut<T> for SingletonRw<T>
{
    #[cfg(feature = "feature_mapped_lock_guards")]
    type GuardMut<'a>
        = MappedRwLockWriteGuard<'a, T>
    where
        Self: 'a;
    #[cfg(not(feature = "feature_mapped_lock_guards"))]
    type GuardMut<'a>
        = DoubleDeref<RwLockWriteGuard<'a, Identity<T>>>
    where
        Self: 'a;

    type Error<'a>
        = TryLockError<RwLockWriteGuard<'a, Identity<T>>>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        match self.try_get_mut()
        {
            Ok(guard) => guard,
            Err(e) =>
            {
                panic!("SingletonRw<{}> can't be written: {:?}", std::any::type_name::<T>(), e)
            }
        }
    }

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.guarded.write().map(map_identity_rw_write_guard).map_err(Into::into) }
}
impl_singleton_methods!(SingletonRw);

impl<T> SingletonLazyRw<T>
{
    pub const fn new(f: fn() -> T) -> Self
    {
        Self {
            guarded: RwLock::new(LazyLock::new(f)),
        }
    }
}

impl<T> Debug for SingletonLazyRw<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.guarded.read()
        {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonLazyRw<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonLazyRw<T>
{
    #[cfg(feature = "feature_mapped_lock_guards")]
    type Guard<'a>
        = MappedRwLockReadGuard<'a, T>
    where
        Self: 'a;
    #[cfg(not(feature = "feature_mapped_lock_guards"))]
    type Guard<'a>
        = DoubleDeref<RwLockReadGuard<'a, LazyLock<T>>>
    where
        Self: 'a;
    type Error<'a>
        = PoisonError<RwLockReadGuard<'a, LazyLock<T>>>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        match self.try_get()
        {
            Ok(guard) => guard,
            Err(e) => panic!("SingletonLazyRw<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.guarded.read().map(map_lazy_rw_read_guard) }
}

impl<T> GuardedMut<T> for SingletonLazyRw<T>
{
    #[cfg(feature = "feature_mapped_lock_guards")]
    type GuardMut<'a>
        = MappedRwLockWriteGuard<'a, T>
    where
        Self: 'a;
    #[cfg(not(feature = "feature_mapped_lock_guards"))]
    type GuardMut<'a>
        = DoubleDeref<RwLockWriteGuard<'a, LazyLock<T>>>
    where
        Self: 'a;

    type Error<'a>
        = TryLockError<RwLockWriteGuard<'a, LazyLock<T>>>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        match self.try_get_mut()
        {
            Ok(guard) => guard,
            Err(e) =>
            {
                panic!("SingletonLazyRw<{}> can't be written: {:?}", std::any::type_name::<T>(), e)
            }
        }
    }

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.guarded.write().map(map_lazy_rw_write_guard).map_err(Into::into) }
}
impl_singleton_methods!(SingletonLazyRw);

impl<T> SingletonOptionRw<T>
{
    /// Creates an uninitialized singleton (None).
    pub const fn uninit() -> Self { Self::new(None) }

    /// Creates a pre-initialized singleton with the given value.
    pub const fn from_value(value: T) -> Self { Self::new(Some(value)) }

    /// Creates a singleton with the given option state.
    pub const fn new(value: Option<T>) -> Self { Self { guarded: RwLock::new(value) } }
    // Example of function to not code because of the sync/lock aspect:
    // If the user check `my_singleton.is_initialized()`, his next instinct will probably be to call `my_singleton.get()` which is wrong
    //pub fn is_initialized(&self) -> bool { self.try_get().is_ok() }
}

impl<T> SingletonOptionable<T> for SingletonOptionRw<T>
{
    fn init_from_fn<'a, F>(&'a self, init: F) -> Result<Self::GuardMut<'a>, F>
    where
        F: FnOnce() -> T,
    {
        match self.guarded.try_write()
        {
            Ok(mut guard) =>
            {
                if guard.is_none()
                {
                    *guard = Some(init());
                }
                Ok(map_option_rw_write_guard(guard))
            }
            Err(_) => Err(init),
        }
    }

    fn swap<'a>(&'a self, other: &mut Option<T>) -> Result<(), Self::Error<'a>>
    {
        match self.guarded.write()
        {
            Ok(mut guard) =>
            {
                std::mem::swap(&mut *guard, other);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl<T> Debug for SingletonOptionRw<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.guarded.read()
        {
            Ok(guard) => write!(f, "{:?}", guard.as_ref()),
            Err(e) => write!(f, "SingletonOptionRw<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonOptionRw<T>
{
    #[cfg(feature = "feature_mapped_lock_guards")]
    type Guard<'a>
        = MappedRwLockReadGuard<'a, T>
    where
        Self: 'a;
    #[cfg(not(feature = "feature_mapped_lock_guards"))]
    type Guard<'a>
        = DoubleDerefOption<RwLockReadGuard<'a, Option<T>>, T>
    where
        Self: 'a;
    type Error<'a>
        = NotInitErrorOr<PoisonError<RwLockReadGuard<'a, Option<T>>>>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        match self.try_get()
        {
            Ok(guard) => return guard,
            Err(e) => match e
            {
                NotInitErrorOr::NotInit => panic!("SingletonOptionRw<{}> not initialized: call init() first", std::any::type_name::<T>()),
                NotInitErrorOr::Other(guard) => panic!("SingletonOptionRw<{}> is poisoned: {:?}", std::any::type_name::<T>(), guard),
            },
        }
    }

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        let guard = self.guarded.read()?;
        if guard.is_none()
        {
            return Err(NotInitErrorOr::NotInit);
        }
        return Ok(map_option_rw_read_guard(guard));
    }
}

impl<T> GuardedMut<T> for SingletonOptionRw<T>
{
    #[cfg(feature = "feature_mapped_lock_guards")]
    type GuardMut<'a>
        = MappedRwLockWriteGuard<'a, T>
    where
        Self: 'a;
    #[cfg(not(feature = "feature_mapped_lock_guards"))]
    type GuardMut<'a>
        = DoubleDerefOption<RwLockWriteGuard<'a, Option<T>>, T>
    where
        Self: 'a;
    type Error<'a>
        = NotInitErrorOr<PoisonError<RwLockWriteGuard<'a, Option<T>>>>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        match self.try_get_mut()
        {
            Ok(guard) => return guard,
            Err(e) => match e
            {
                NotInitErrorOr::NotInit => panic!("SingletonOptionRw<{}> not initialized: call init() first", std::any::type_name::<T>()),
                NotInitErrorOr::Other(guard) => panic!("SingletonOptionRw<{}> is poisoned: {:?}", std::any::type_name::<T>(), guard),
            },
        }
    }

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        let guard = self.guarded.write()?;
        if guard.is_none()
        {
            return Err(NotInitErrorOr::NotInit);
        }
        return Ok(map_option_rw_write_guard(guard));
    }
}
impl_singleton_methods!(SingletonOptionRw);
