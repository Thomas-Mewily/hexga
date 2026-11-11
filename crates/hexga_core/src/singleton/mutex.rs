use std::sync::MutexGuard;
use super::*;

/// Multi threaded singleton, where the value is initialized at compile time.
pub type SingletonMutex<T> = SingletonOf<Mutex<Identity<T>>>;
/// Multi threaded singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonLazyMutex<T> = SingletonOf<Mutex<LazyLock<T>>>;
/// Multi threaded singleton, where the value should be manually initialized at runtime.
pub type SingletonOptionMutex<T> = SingletonOf<Mutex<Option<T>>>;

// Wasm is single threaded right now, so this is ok.
#[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
mod single_thread
{
    use super::*;

    unsafe impl<T> Sync for SingletonMutex<T> {}
    unsafe impl<T> Send for SingletonMutex<T> {}

    unsafe impl<T> Sync for SingletonLazyMutex<T> {}
    unsafe impl<T> Send for SingletonLazyMutex<T> {}

    unsafe impl<T> Sync for SingletonOptionMutex<T> {}
    unsafe impl<T> Send for SingletonOptionMutex<T> {}
}

impl<T> SingletonMutex<T>
{
    pub const fn new(value: T) -> Self
    {
        Self {
            guarded: Mutex::new(Identity::new(value)),
        }
    }
}

impl<T> Debug for SingletonMutex<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get_mut()
        {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonMutex<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}


impl<T> GuardedMut<T> for SingletonMutex<T>
{
    type GuardMut<'a> = DoubleDeref<MutexGuard<'a, Identity<T>>> where Self: 'a;
    type Error<'a> = PoisonError<MutexGuard<'a, Identity<T>>> where Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        match self.try_get_mut()
        {
            Ok(guard) => guard,
            Err(e) => panic!("SingletonMutex<{}> is poisoned: {:?}", std::any::type_name::<T>(), e),
        }
    }

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.guarded.lock().map(DoubleDeref::from) }
}
impl_singleton_methods_get_mut!(SingletonMutex);

impl<T> SingletonLazyMutex<T>
{
    pub const fn new(f: fn() -> T) -> Self
    {
        Self {
            guarded: Mutex::new(LazyLock::new(f)),
        }
    }
}

impl<T> Debug for SingletonLazyMutex<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get_mut()
        {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonLazyMutex<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> GuardedMut<T> for SingletonLazyMutex<T>
{
    type GuardMut<'a> = DoubleDeref<MutexGuard<'a, LazyLock<T>>> where Self: 'a;
    type Error<'a>
        = PoisonError<MutexGuard<'a, LazyLock<T>>>
    where
        Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a>
    {
        match self.try_get_mut()
        {
            Ok(guard) => guard,
            Err(e) => panic!("SingletonMutex<{}> is poisoned: {:?}", std::any::type_name::<T>(), e),
        }
    }

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.guarded.lock().map(DoubleDeref::from) }
}
impl_singleton_methods_get_mut!(SingletonLazyMutex);

impl<T> SingletonOptionMutex<T>
{
    /// Creates an uninitialized singleton (None).
    pub const fn uninit() -> Self { Self::new(None) }

    /// Creates a pre-initialized singleton with the given value.
    pub const fn from_value(value: T) -> Self { Self::new(Some(value)) }

    /// Creates a singleton with the given option state.
    pub const fn new(value: Option<T>) -> Self { Self { guarded: Mutex::new(value) } }
}

impl<T> SingletonOptionable<T> for SingletonOptionMutex<T>
{
    fn init_from_fn<'a, F>(&'a self, init: F) -> Result<DoubleDerefOption<MutexGuard<'a, Option<T>>, T>, F>
    where
        F: FnOnce() -> T,
    {
        match self.guarded.lock()
        {
            Ok(mut guard) =>
            {
                if guard.is_none()
                {
                    *guard = Some(init());
                }
                Ok(DoubleDerefOption::from(guard))
            }
            Err(_) => Err(init),
        }
    }

    fn swap<'a>(&'a self, other: &mut Option<T>) -> Result<(), Self::Error<'a>>
    {
        match self.guarded.lock()
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

impl<T> Debug for SingletonOptionMutex<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get_mut()
        {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonOptionMutex<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}


impl<T> GuardedMut<T> for SingletonOptionMutex<T>
{
    type GuardMut<'a>
        = DoubleDerefOption<MutexGuard<'a, Option<T>>, T>
    where
        Self: 'a;
    type Error<'a>
        = NotInitErrorOr<PoisonError<MutexGuard<'a, Option<T>>>>
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
                NotInitErrorOr::NotInit => panic!("SingletonOptionMutex<{}> not initialized: call init() first", std::any::type_name::<T>()),
                NotInitErrorOr::Other(guard) => panic!("SingletonOptionMutex<{}> is poisoned: {:?}", std::any::type_name::<T>(), guard),
            },
        }
    }

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>>
    {
        let guard = self.guarded.lock()?;
        if guard.is_none() { return Err(NotInitErrorOr::NotInit); }
        return Ok(DoubleDerefOption::from(guard));
    }
}
impl_singleton_methods_get_mut!(SingletonOptionMutex);
