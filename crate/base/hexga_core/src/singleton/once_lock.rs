use super::*;

// TODO: impl them
/*
/// Multi threaded read only singleton, where the value is initialized at compile time.
pub type SingletonOnce<T> = SingletonOf<OnceLock<T>>;
/// Multi threaded read only singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonOnceLazy<T> = SingletonOf<LazyLock<T>>;
*/

/// Multi threaded read only singleton, where the value is initialized at compile time.
pub type SingletonOnce<T> = SingletonOf<OnceLock<T>>;

/*
impl<T> SingletonOnce<T> {
    /// Creates an uninitialized singleton.
    pub const fn uninit() -> Self {
        Self::from_guard(OnceLock::new())
    }

    pub const fn new(value: T) -> Self {
        Self::from_guard(OnceLock::new())
    }


    /// Sets the value if not already set.
    pub fn set(&self, value: T) -> Result<(), T> {
        self.guarded.set(value)
    }

    /// Returns true if initialized.
    pub fn is_initialized(&self) -> bool {
        self.guarded.get().is_some()
    }

    /// Gets or initializes with a function.
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.guarded.get_or_init(f)
    }

    /// Gets or initializes with a fallible function.
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        self.guarded.get_or_try_init(f)
    }
}

impl<T> Default for SingletonOnce<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for SingletonOnce<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.guarded.get() {
            Some(v) => write!(f, "Some({:?})", v),
            None => write!(f, "None"),
        }
    }
}

impl<T> Guarded<T> for SingletonOnce<T> {
    type Guard<'a> = &'a T where Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        self.guarded.get().unwrap_or_else(|| {
            panic!("SingletonOnce<{}> not initialized", std::any::type_name::<T>())
        })
    }
}

impl<T> TryGuarded<T> for SingletonOnce<T> {
    type Error<'a> = () where Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> {
        self.guarded.get().ok_or(())
    }
}

impl<T> SingletonOnceable<T> for SingletonOnce<T> {
    fn init_from_fn<F>(&self, f: F) -> Result<Self::Guard<'_>, (Self::Error<'_>, F)>
    where
        F: FnOnce() -> T,
    {
        // OnceLock::get_or_init never fails and returns &T
        // Our Error type is (), so we can't represent failure here
        Ok(self.guarded.get_or_init(f))
    }
}

impl_singleton_methods_get!(SingletonOnce);
*/