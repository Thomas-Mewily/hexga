use super::*;

// TODO: impl them + Do a trait SingletonOnceable ?

/*
/// Single threaded read only singleton, where the value is initialized at compile time.
pub type SingletonOnceCell<T> = SingletonOf<SingleThreadCell<OnceCell<T>>>;
/// Single threaded read only singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonOnceLazyCell<T> = SingletonOf<SingleThreadCell<LazyLock<T>>>;

*/