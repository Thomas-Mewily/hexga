//! Traits common to shared pointer (`rc` and `sync`)

pub mod prelude
{
    pub use super::traits::*;
}
pub mod traits
{
    pub use super::{SharedCount, SharedDowngrade, SharedUpgrade};
}

pub trait SharedCount
{
    /// Gets the number of strong pointers pointing to this allocation.
    ///
    /// If `self` was created using [`Weak::new`], this will return 0.
    fn strong_count(&self) -> usize;

    /// Gets the number of `Weak` pointers pointing to this allocation.
    ///
    /// If no strong pointers remain, this will return zero.
    fn weak_count(&self) -> usize;
}

pub trait SharedUpgrade: SharedCount
{
    type Output: SharedCount + SharedDowngrade; //+ SharedDowngrade<Ouput = Self>;
    /// Attempts to upgrade the `Weak` pointer to an `Strong` one, delaying
    /// dropping of the inner value if successful.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    fn upgrade(&self) -> Option<Self::Output>;
}

pub trait SharedDowngrade: SharedCount
{
    type Ouput: SharedCount + SharedUpgrade; //+ SharedUpgrade<Output = Self>;
    /// Creates a new [`Weak`] pointer to this allocation.
    fn downgrade(&self) -> Self::Ouput;
}

#[cfg(feature = "std")]
mod std_impl
{
    use super::{SharedCount, SharedDowngrade, SharedUpgrade};

    impl<T: ?Sized> SharedCount for std::boxed::Box<T>
    {
        fn strong_count(&self) -> usize { 1 }
        fn weak_count(&self) -> usize { 0 }
    }

    impl<T: ?Sized> SharedCount for std::rc::Weak<T>
    {
        fn strong_count(&self) -> usize { Self::strong_count(self) }

        fn weak_count(&self) -> usize { Self::weak_count(self) }
    }
    impl<T: ?Sized> SharedCount for std::rc::Rc<T>
    {
        fn strong_count(&self) -> usize { Self::strong_count(self) }

        fn weak_count(&self) -> usize { Self::weak_count(self) }
    }

    impl<T: ?Sized> SharedCount for std::sync::Weak<T>
    {
        fn strong_count(&self) -> usize { Self::weak_count(self) }

        fn weak_count(&self) -> usize { Self::weak_count(self) }
    }
    impl<T: ?Sized> SharedCount for std::sync::Arc<T>
    {
        fn strong_count(&self) -> usize { Self::weak_count(self) }

        fn weak_count(&self) -> usize { Self::weak_count(self) }
    }

    impl<T: ?Sized> SharedUpgrade for std::rc::Weak<T>
    {
        type Output = std::rc::Rc<T>;

        fn upgrade(&self) -> Option<Self::Output> { Self::upgrade(self) }
    }

    impl<T: ?Sized> SharedDowngrade for std::rc::Rc<T>
    {
        type Ouput = std::rc::Weak<T>;
        fn downgrade(&self) -> Self::Ouput { Self::downgrade(self) }
    }

    impl<T: ?Sized> SharedUpgrade for std::sync::Weak<T>
    {
        type Output = std::sync::Arc<T>;

        fn upgrade(&self) -> Option<Self::Output> { Self::upgrade(self) }
    }
    impl<T: ?Sized> SharedDowngrade for std::sync::Arc<T>
    {
        type Ouput = std::sync::Weak<T>;
        fn downgrade(&self) -> Self::Ouput { Self::downgrade(self) }
    }
}
