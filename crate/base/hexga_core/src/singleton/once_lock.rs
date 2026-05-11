use super::*;


/// Multi threaded read only singleton, where the value is initialized at compile time.
pub type SingletonOnce<T> = SingletonOf<OnceLock<T>>;
/// Multi threaded read only singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonOnceLazy<T> = SingletonOf<LazyLock<T>>;

impl<T> SingletonOnce<T> {
    pub const fn uninit() -> Self {
        Self::from_guard(OnceLock::new())
    }

    pub const fn new() -> Self {
        Self::uninit()
    }
}


impl<T> Debug for SingletonOnce<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get()
        {
            Ok(guard) => write!(f, "{:?}", guard),
            Err(e) => write!(f, "SingletonOnce<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonOnce<T> {
    type Guard<'a> = &'a T where Self: 'a;

    #[inline]
    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        self.guarded.get().unwrap_or_else(|| {
            panic!("SingletonOnce<{}> not initialized", std::any::type_name::<T>())
        })
    }
    type Error<'a> = () where Self: 'a;

    
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> {
        self.guarded.get().ok_or(())
    }
}

impl<T> SingletonOnceable<T> for SingletonOnce<T> 
{
    fn init_from_fn<'a,F>(&'a self, init: F) -> Result<Self::Guard<'a>, F> 
        where F: FnOnce() -> T 
    {
        Ok(self.guarded.get_or_init(init))
    }
}

impl_singleton_methods_get!(SingletonOnce);



impl<T> SingletonOnceLazy<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self::from_guard(LazyLock::new(f))
    }

    pub const fn uninit() -> Self {
        Self::from_guard(LazyLock::new(|| panic!("LazyLock not initialized")))
    }
}

impl<T> Debug for SingletonOnceLazy<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get()
        {
            Ok(guard) => write!(f, "{:?}", guard),
            Err(e) => write!(f, "SingletonOnceLazy<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonOnceLazy<T> {
    type Guard<'a> = &'a T where Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        &self.guarded
    }

    type Error<'a> = std::convert::Infallible where Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> {
        Ok(&self.guarded)
    }
}
impl_singleton_methods_get!(SingletonOnceLazy);