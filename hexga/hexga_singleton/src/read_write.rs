use std::ops::{Deref, DerefMut};

pub trait SingletonRead
{
    type Target;
    type ReadGuard : Deref<Target=Self::Target>;

    fn try_read() -> Option<Self::ReadGuard>;
    #[inline(always)]
    #[track_caller]
    fn read() -> Self::ReadGuard { Self::try_read().expect("not init") }

    fn is_init() -> bool { Self::try_read().is_some() }
    fn is_not_init() -> bool { !Self::is_init() }
}
pub trait SingletonWrite: SingletonRead
{
    type WriteGuard : DerefMut<Target=Self::Target>;
    fn try_write() -> Option<Self::WriteGuard>;
    #[inline(always)]
    #[track_caller]
    fn write() -> Self::WriteGuard { Self::try_write().expect("not init") }
}


#[macro_export]
macro_rules! singleton_single_thread {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $static_name:ident, $init:expr) => {
        static $static_name: $crate::SingletonSingleThread<$inner> = $crate::SingletonSingleThread::new($init);

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Target = $inner;
            type ReadGuard = <$crate::cell::SingleThread<$inner> as $crate::guard::ReadGuard>::ReadGuard<'static>;

            fn try_read() -> Option<Self::ReadGuard>
            {
                <$crate::SingletonSingleThread<$inner> as $crate::guard::TryReadGuard>::try_read(&$static_name)
                    .map(|g| $crate::guard::map(g, |lazy_cell| ::std::cell::LazyCell::force(lazy_cell))).ok()
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type WriteGuard = <$crate::cell::SingleThread<$inner> as $crate::guard::WriteGuard>::WriteGuard<'static>;

            fn try_write() -> Option<Self::WriteGuard>
            {
                <$crate::SingletonSingleThread<$inner> as $crate::guard::TryWriteGuard>::try_write(&$static_name)
                    .map(|g| $crate::guard::map_mut(g, |lazy_cell| $crate::features::lazy_cell_force_mut(lazy_cell))).ok()
            }
        }

        impl std::ops::Deref for $wrapper {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                <$wrapper as $crate::SingletonRead>::read().inner_reference
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$wrapper as $crate::SingletonWrite>::write().inner_reference
            }
        }
    };
}

/*
#[macro_export]
macro_rules! singleton_single_thread_access {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $try_read:block, $try_write:block) => {

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Error = <$crate::cell::SingleThread<$inner> as $crate::guard::TryReadGuard>::Error<'static>;
            type Target = $inner;
            type ReadGuard = $crate::guard::ReferenceReadGuard<'static, $inner>;

            fn try_read() -> Result<Self::ReadGuard, Self::Error> {
                let reference = $try_read?;
                Ok($crate::guard::ReferenceReadGuard::new(reference))
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type Error = <$crate::cell::SingleThread<$inner> as $crate::guard::TryWriteGuard>::Error<'static>;
            type WriteGuard = <$crate::cell::SingleThread<$inner> as $crate::guard::WriteGuard>::WriteGuard<'static>;

            fn try_write() -> Result<Self::WriteGuard, <Self as $crate::SingletonWrite>::Error> {
                let reference_mut = $try_write?;
                Ok($crate::guard::ReferenceWriteGuard::new(reference_mut))
            }
        }

        impl std::ops::Deref for $wrapper {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                <$wrapper as $crate::SingletonRead>::read().value
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$wrapper as $crate::SingletonWrite>::write().value
            }
        }
    };
}
*/

#[macro_export]
macro_rules! singleton_multi_thread {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $static_name:ident, $init:expr) => {
        static $static_name: $crate::SingletonMultiThread<$inner> =
            $crate::SingletonMultiThread::new($init);

$(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Target = $inner;
            type ReadGuard = $crate::features::MappedRwLockReadGuard<'static, $inner>;

            fn try_read() -> Option<Self::ReadGuard>
            {
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryReadGuard>::try_read(&$static_name)
                    .map(|g| $crate::guard::map(g, |l| ::std::sync::LazyLock::force(l))).ok()
            }
        }

        impl $crate::SingletonWrite for $wrapper
        {
            type WriteGuard = $crate::features::MappedRwLockWriteGuard<'static, $inner>;

            fn try_write() -> Option<Self::WriteGuard>
            {
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryWriteGuard>::try_write(&$static_name)
                    .map(|g| $crate::guard::map_mut(g, |lazy_lock| $crate::features::lazy_lock_force_mut(lazy_lock))).ok()
            }
        }
    };
}

#[macro_export]
macro_rules! singleton_single_thread_access {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $try_read:block, $try_write:block) => {

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Target = $inner;
            type ReadGuard = $crate::guard::ReferenceReadGuard<'static, $inner>;
            fn try_read() -> Option<Self::ReadGuard> { $try_read }
        }

        impl $crate::SingletonWrite for $wrapper {
            type WriteGuard = $crate::guard::ReferenceWriteGuard<'static, $inner>;
            fn try_write() -> Option<Self::WriteGuard> { $try_write }
        }

        impl std::ops::Deref for $wrapper {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                <$wrapper as $crate::SingletonRead>::read().inner_reference
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$wrapper as $crate::SingletonWrite>::write().inner_reference
            }
        }
    };
}


#[macro_export]
macro_rules! singleton_single_thread_project {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $parent:ident, $field:ident) => {
        $crate::singleton_single_thread_access!(
            $(#[$attr])*
            $vis $wrapper,
            $inner,

            {
                {
                    let parent = $parent::try_read()?;
                    Some($crate::guard::ReferenceReadGuard::new(
                        &parent.inner_reference.$field
                    ))
                }
            },

            {
                {
                    let parent = $parent::try_write()?;
                    Some($crate::guard::ReferenceWriteGuard::new(
                        &mut parent.inner_reference.$field
                    ))
                }
            }
        );
    };
}


#[macro_export]
macro_rules! singleton_multi_thread_project {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $parent:ident, $field:ident) => {

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Target = $inner;
            type ReadGuard = $crate::features::MappedRwLockReadGuard<'static, $inner>;

            fn try_read() -> Option<Self::ReadGuard> {
                let guard = $parent::try_read()?;
                Some($crate::guard::map(guard, |v| &v.$field))
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type WriteGuard = $crate::features::MappedRwLockWriteGuard<'static, $inner>;

            fn try_write() -> Option<Self::WriteGuard> {
                let guard = $parent::try_write()?;
                Some($crate::guard::map_mut(guard, |v| &mut v.$field))
            }
        }
    };
}



/*
TODO: how to not expose the mapped guard
#[macro_export]
macro_rules! singleton_multi_thread_access {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $try_read:block, $try_write:block) => {

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Error =
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryReadGuard>::Error<'static>;

            type Target = $inner;

            type ReadGuard =
                $crate::features::MappedRwLockReadGuard<'static, $inner>;

            fn try_read() -> Result<Self::ReadGuard, Self::Error> {
                $try_read
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type Error =
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryWriteGuard>::Error<'static>;

            type WriteGuard =
                $crate::features::MappedRwLockWriteGuard<'static, $inner>;

            fn try_write()
                -> Result<Self::WriteGuard, <Self as $crate::SingletonWrite>::Error>
            {
                $try_write
            }
        }
    };
}
*/


// Used insde the macro, since some feature are unstable
#[doc(hidden)]
pub mod features
{
    pub type MappedRwLockReadGuard<'a,T> = ::std::sync::MappedRwLockReadGuard<'a, T>;
    pub type MappedRwLockWriteGuard<'a,T> = ::std::sync::MappedRwLockWriteGuard<'a, T>;

    #[must_use]
    #[inline(always)]
    pub fn lazy_lock_force_mut<T,F>(this: &mut std::sync::LazyLock<T, F>) -> &mut T
        where F: FnOnce() -> T
    {
        std::sync::LazyLock::force_mut(this)
    }

    #[must_use]
    #[inline(always)]
    pub fn lazy_cell_force_mut<T,F>(this: &mut std::cell::LazyCell<T, F>) -> &mut T
        where F: FnOnce() -> T
    {
        std::cell::LazyCell::force_mut(this)
    }
}