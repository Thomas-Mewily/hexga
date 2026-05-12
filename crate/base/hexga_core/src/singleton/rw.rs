use super::*;

/// Multi threaded singleton, where the value is initialized at compile time.
pub type SingletonRw<T> = SingletonOf<RwLock<Identity<T>>>;
/// Multi threaded singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonLazyRw<T> = SingletonOf<RwLock<LazyLock<T>>>;
/// Multi threaded singleton, where the value should be manually initialized at runtime.
pub type SingletonOptionRw<T> = SingletonOf<RwLock<Option<T>>>;

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
            Err(e) => write!(
                f,
                "Singleton<{}> can't be read: {:?}",
                std::any::type_name::<T>(),
                e
            ),
        }
    }
}

impl<T> Guarded<T> for SingletonRw<T>
{
    type Guard<'a>
        = MappedRwLockReadGuard<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        let guard = match self.guarded.read() {
            Ok(guard) => guard,
            Err(e) => {
                panic!(
                    "SingletonRw<{}> can't be read: {:?}",
                    std::any::type_name::<T>(),
                    e
                )
            }
        };
        
        guard.guard_map(|v| v.as_ref())
    }

    type Error<'a>
        = TryLockError<RwLockReadGuard<'a, Identity<T>>>
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        Ok(self.guarded.read()?.guard_map(|v| v.as_ref()))
    }
}

impl<T> GuardedMut<T> for SingletonRw<T>
{
    type GuardMut<'a>
        = MappedRwLockWriteGuard<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        let guard = match self.guarded.write() {
            Ok(guard) => guard,
            Err(e) => {
                panic!(
                    "SingletonRw<{}> can't be written: {:?}",
                    std::any::type_name::<T>(),
                    e
                )
            }
        };
        
        guard.guard_map_mut(|v| v.as_mut())
    }

    type Error<'a>
        = TryLockError<RwLockWriteGuard<'a, Identity<T>>>
    where
        Self: 'a;

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        Ok(self.guarded.write()?.guard_map_mut(|v| v.as_mut()))
    }
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
            Err(e) => write!(
                f,
                "SingletonLazyRw<{}> can't be read: {:?}",
                std::any::type_name::<T>(),
                e
            ),
        }
    }
}

impl<T> Guarded<T> for SingletonLazyRw<T>
{
    type Guard<'a>
        = MappedRwLockReadGuard<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        let guard = match self.guarded.read() {
            Ok(guard) => guard,
            Err(e) => {
                panic!(
                    "SingletonLazyRw<{}> can't be read: {:?}",
                    std::any::type_name::<T>(),
                    e
                )
            }
        };
        
        guard.guard_map(|v| v.deref())
    }

    type Error<'a>
        = TryLockError<RwLockReadGuard<'a, LazyLock<T>>>
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        Ok(self.guarded.read()?.guard_map(|v| v.deref()))
    }
}

impl<T> GuardedMut<T> for SingletonLazyRw<T>
{
    type GuardMut<'a>
        = MappedRwLockWriteGuard<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        match self.guarded.write() {
            Ok(guard) => guard.guard_map_mut(|v| v.deref_mut()),
            Err(e) => {
                panic!(
                    "SingletonLazyRw<{}> can't be written: {:?}",
                    std::any::type_name::<T>(),
                    e
                )
            }
        }
    }

    type Error<'a>
        = TryLockError<RwLockWriteGuard<'a, LazyLock<T>>>
    where
        Self: 'a;

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        Ok(self.guarded.write()?.guard_map_mut(|v| v.deref_mut()))
    }
}
impl_singleton_methods!(SingletonLazyRw);

impl<T> SingletonOptionRw<T>
{
    /// Creates an uninitialized singleton (None).
    pub const fn uninit() -> Self { Self::new(None) }

    /// Creates a pre-initialized singleton with the given value.
    pub const fn from_value(value: T) -> Self { Self::new(Some(value)) }

    /// Creates a singleton with the given option state.
    pub const fn new(value: Option<T>) -> Self
    {
        Self {
            guarded: RwLock::new(value),
        }
    }
    // Example of function to not code because of the sync/lock aspect:
    // If the user check `my_singleton.is_initialized()`, his next instinct will probably be to call `my_singleton.get()` which is wrong
    //pub fn is_initialized(&self) -> bool { self.try_get().is_ok() }
}

impl<T> SingletonOptionable<T> for SingletonOptionRw<T>
{
    fn init_from_fn<'a, F>(&'a self, init: F) -> Result<MappedRwLockWriteGuard<'a, T>, F>
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
                Ok(RwLockWriteGuard::map(guard, |opt| opt.as_mut().unwrap()))
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
            Err(e) => write!(
                f,
                "SingletonOptionRw<{}> can't be read: {:?}",
                std::any::type_name::<T>(),
                e
            ),
        }
    }
}

impl<T> Guarded<T> for SingletonOptionRw<T>
{
    type Guard<'a>
        = MappedRwLockReadGuard<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        let guard = match self.guarded.read() {
            Ok(guard) => guard,
            Err(e) => {
                panic!(
                    "SingletonOptionRw<{}> can't be read: {:?}",
                    std::any::type_name::<T>(),
                    e
                )
            }
        };

        if guard.is_none() {
            panic!(
                "SingletonOptionRw<{}> not initialized: call init() first",
                std::any::type_name::<T>()
            );
        }

        RwLockReadGuard::map(guard, |opt| opt.as_ref().unwrap())
    }

    type Error<'a>
        = PoisonError<RwLockReadGuard<'a, Option<T>>>
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        let guard = self.guarded.read()?;
        if guard.is_none()
        {
            // FIXME: make some kind of proper error
            return Err(PoisonError::new(guard));
        }
        Ok(RwLockReadGuard::map(guard, |opt| opt.as_ref().unwrap()))
    }
}

impl<T> GuardedMut<T> for SingletonOptionRw<T>
{
    type GuardMut<'a>
        = MappedRwLockWriteGuard<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        let guard = match self.guarded.write() {
            Ok(guard) => guard,
            Err(e) => {
                panic!(
                    "SingletonOptionRw<{}> can't be written: {:?}",
                    std::any::type_name::<T>(),
                    e
                )
            }
        };

        if guard.is_none() {
            panic!(
                "SingletonOptionRw<{}> not initialized: call init() first",
                std::any::type_name::<T>()
            );
        }

        RwLockWriteGuard::map(guard, |opt| opt.as_mut().unwrap())
    }

    type Error<'a>
        = TryLockError<RwLockWriteGuard<'a, Option<T>>>
    where
        Self: 'a;

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        let guard = self.guarded.write()?;
        if guard.is_none()
        {
            return Err(TryLockError::WouldBlock);
        }
        Ok(RwLockWriteGuard::map(guard, |opt| opt.as_mut().unwrap()))
    }
}
impl_singleton_methods!(SingletonOptionRw);
