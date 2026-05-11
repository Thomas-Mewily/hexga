use std::sync::MutexGuard;

use super::*;

/// Multi threaded singleton, where the value is initialized at compile time.
pub type SingletonMutex<T> = SingletonOf<Mutex<Identity<T>>>;
/// Multi threaded singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonLazyMutex<T> = SingletonOf<Mutex<LazyLock<T>>>;
/// Multi threaded singleton, where the value should be manually initialized at runtime.
pub type SingletonOptionMutex<T> = SingletonOf<Mutex<Option<T>>>;



impl<T> SingletonMutex<T> {
    pub const fn new(value: T) -> Self { 
        Self { guarded: Mutex::new(Identity::new(value)) }
    }
}

impl<T> Debug for SingletonMutex<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get_mut() {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonMutex<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> GuardedMut<T> for SingletonMutex<T> {
    type GuardMut<'a> = MappedMutexGuard<'a, T> where Self: 'a;

    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        self.guarded
            .lock()
            .unwrap_or_else(|e| panic!("SingletonMutex<{}> poisoned: {:?}", std::any::type_name::<T>(), e))
            .guard_map_mut(|v| v.as_mut())
    }
    type Error<'a> = PoisonError<MutexGuard<'a, Identity<T>>> where Self: 'a;

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        
        Ok(self.guarded
            .lock()?
            .guard_map_mut(|v| v.as_mut()))
    }
}
impl_singleton_methods_get_mut!(SingletonMutex);




impl<T> SingletonLazyMutex<T> {
    pub const fn new(f: fn() -> T) -> Self { 
        Self { guarded: Mutex::new(LazyLock::new(f)) }
    }
}

impl<T> Debug for SingletonLazyMutex<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get_mut() {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonLazyMutex<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> GuardedMut<T> for SingletonLazyMutex<T> {
    type GuardMut<'a> = MappedMutexGuard<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        self.guarded
            .lock()
            .unwrap_or_else(|e| panic!("SingletonLazyMutex<{}> poisoned: {:?}", std::any::type_name::<T>(), e))
            .guard_map_mut(|v| v.deref_mut())
    }

    type Error<'a> = TryLockError<MutexGuard<'a, LazyLock<T>>> where Self: 'a;
    
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        Ok(
            self.guarded
                .lock()?
                .guard_map_mut(|v| v.deref_mut())
        )
    }
}
impl_singleton_methods_get_mut!(SingletonLazyMutex);



impl<T> SingletonOptionMutex<T> {
    /// Creates an uninitialized singleton (None).
    pub const fn uninit() -> Self {
        Self::new(None)
    }

    /// Creates a pre-initialized singleton with the given value.
    pub const fn from_value(value: T) -> Self {
        Self::new(Some(value))
    }

    /// Creates a singleton with the given option state.
    pub const fn new(value: Option<T>) -> Self {
        Self { guarded: Mutex::new(value) }
    }
}


impl<T> SingletonOptionable<T> for SingletonOptionMutex<T> 
{
    fn init_from_fn<'a, F>(&'a self, init: F) -> Result<MappedMutexGuard<'a, T>, F> 
    where 
        F: FnOnce() -> T
    {
        match self.guarded.lock() {
            Ok(mut guard) => {
                if guard.is_none() {
                    *guard = Some(init());
                }
                Ok(MutexGuard::map(guard, |opt| opt.as_mut().unwrap()))
            }
            Err(_) => Err(init),
        }
    }

    fn swap<'a>(&'a self, other: &mut Option<T>) -> Result<(), Self::Error<'a>> {
        match self.guarded.lock() {
            Ok(mut guard) => {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get_mut() {
            Ok(guard) => write!(f, "{:?}", guard.deref()),
            Err(e) => write!(f, "SingletonOptionMutex<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> GuardedMut<T> for SingletonOptionMutex<T> {
    type GuardMut<'a> = MappedMutexGuard<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        let guard = self.guarded.lock().unwrap_or_else(|e| {
            panic!("SingletonOptionMutex<{}> poisoned: {:?}", std::any::type_name::<T>(), e)
        });
        
        if guard.is_none() {
            panic!("SingletonOptionMutex<{}> not initialized: call init() first", std::any::type_name::<T>());
        }
        
        MutexGuard::map(guard, |opt| opt.as_mut().unwrap())
    }

    type Error<'a> = TryLockError<MutexGuard<'a, Option<T>>> where Self: 'a;
    
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        let guard = self.guarded.lock()?;
        if guard.is_none() {
            return Err(TryLockError::WouldBlock);
        }
        Ok(MutexGuard::map(guard, |opt| opt.as_mut().unwrap()))
    }
}
impl_singleton_methods_get_mut!(SingletonOptionMutex);