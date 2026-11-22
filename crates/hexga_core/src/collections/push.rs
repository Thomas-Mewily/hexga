use super::*;

pub trait Push<T>: Collection
{
    type Output;
    /// Push an value and return it's index.
    fn push(&mut self, value: T) -> Self::Output;

    //fn push_mut(&mut self, value: T) -> &mut T { self.push_entry(value).1 }
    //fn push_entry(&mut self, value: T) -> (Self::Output, &mut T);

    /// Pushes a value and returns a mutable reference to the new last element.
    /// 
    /// The returned element is **not** guaranteed to be the same as the pushed value, it's depend on the collection type.
    /// For [`Vec`] and [`VecDeque`] the returned element is the same as the pushed value, but not for the [`BinaryHeap`] that return the next element in the heap order.
    fn push_mut(&mut self, value: T) -> Option<&mut T> where Self:  PeekLastMut<T>
    {
        Push::push(self, value);
        PeekLastMut::last_mut(self)
    }
}


pub trait TryPush<T>: Push<T>
{
    type Error;
    /// Push an value and return it's index.
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error>;

    /// Pushes a value and returns a mutable reference to the new last element.
    /// 
    /// The returned element is **not** guaranteed to be the same as the pushed value, it's depend on the collection type.
    /// For [`Vec`] and [`VecDeque`] the returned element is the same as the pushed value, but not for the [`BinaryHeap`] that return the next element in the heap order.
    fn try_push_mut(&mut self, value: T) -> Result<Option<&mut T>, Self::Error> where Self:  PeekLastMut<T>
    {
        let _ = TryPush::try_push(self, value)?;
        Ok(PeekLastMut::last_mut(self))
    }
}

pub trait PushFront<T>: Collection
{
    type Output;
    /// Push an value and return it's index.
    fn push_front(&mut self, value: T) -> Self::Output;

    /// Pushes a value and returns a mutable reference to the new first element.
    /// 
    /// The returned element is **not** guaranteed to be the same as the pushed value, it's depend on the collection type.
    fn push_front_mut(&mut self, value: T) -> Option<&mut T> where Self:  PeekFirstMut<T>
    {
        PushFront::push_front(self, value);
        PeekFirstMut::first_mut(self)
    }
}
pub trait TryPushFront<T>: PushFront<T>
{
    type Error;
    /// Push an value and return it's index.
    fn try_push_front(&mut self, value: T) -> Result<Self::Output, Self::Error>;

    /// Pushes a value and returns a mutable reference to the new first element.
    /// 
    /// The returned element is **not** guaranteed to be the same as the pushed value, it's depend on the collection type.
    fn try_push_front_mut(&mut self, value: T) -> Result<Option<&mut T>, Self::Error> where Self:  PeekFirstMut<T>
    {
        let _ = TryPushFront::try_push_front(self, value)?;
        Ok(PeekFirstMut::first_mut(self))
    }
}


impl<T> Push<T> for Vec<T>
{
    type Output = usize;
    #[inline(always)]
    fn push(&mut self, value: T) -> Self::Output
    {
        let l = self.len();
        self.push(value);
        l
    }
}
impl<T> TryPush<T> for Vec<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, value)) }
}

impl<T> Push<T> for VecDeque<T>
{
    type Output = usize;
    fn push(&mut self, value: T) -> Self::Output
    {
        let l = self.len();
        self.push_back(value);
        l
    }
}
impl<T> TryPush<T> for VecDeque<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}
impl<T> PushFront<T> for VecDeque<T>
{
    type Output = usize;
    fn push_front(&mut self, value: T) -> Self::Output
    {
        self.push_front(value);
        0
    }
}
impl<T> TryPushFront<T> for VecDeque<T>
{
    type Error = (); // #proper_error
    fn try_push_front(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

impl<T> Push<T> for LinkedList<T>
{
    type Output = ();
    fn push(&mut self, value: T) -> Self::Output { self.push_back(value); }
}
impl<T> TryPush<T> for LinkedList<T>
{
    type Error = ();

    #[allow(clippy::unit_arg)]
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}
impl<T> PushFront<T> for LinkedList<T>
{
    type Output = (); // #proper_error
    fn push_front(&mut self, value: T) -> Self::Output { self.push_front(value); }
}
impl<T> TryPushFront<T> for LinkedList<T>
{
    type Error = (); // #proper_error

    #[allow(clippy::unit_arg)]
    fn try_push_front(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

impl<T> Push<T> for BinaryHeap<T>
where
    T: Ord,
{
    type Output = ();
    fn push(&mut self, value: T) -> Self::Output { self.push(value); }
}
impl<T> TryPush<T> for BinaryHeap<T>
where
    T: Ord,
{
    type Error = (); // #proper_error

    #[allow(clippy::unit_arg)]
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

impl Push<char> for String
{
    type Output = ();
    fn push(&mut self, value: char) -> Self::Output { self.push(value); }
}
impl TryPush<char> for String
{
    type Error = (); // #proper_error

    #[allow(clippy::unit_arg)]
    fn try_push(&mut self, value: char) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

#[cfg(feature = "std")]
impl<'b> Push<&'b OsStr> for OsString
{
    type Output = ();
    fn push(&mut self, value: &'b OsStr) -> Self::Output { self.push(value); }
}
#[cfg(feature = "std")]
impl<'b> TryPush<&'b OsStr> for OsString
{
    type Error = (); // #proper_error

    #[allow(clippy::unit_arg)]
    fn try_push(&mut self, value: &'b OsStr) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

#[cfg(feature = "std")]
impl<P> Push<P> for PathBuf
where
    P: AsRef<Path>,
{
    type Output = ();
    fn push(&mut self, value: P) -> Self::Output { self.push(value); }
}
#[cfg(feature = "std")]
impl<P> TryPush<P> for PathBuf
where
    P: AsRef<Path>,
{
    type Error = (); // #proper_error

    #[allow(clippy::unit_arg)]
    fn try_push(&mut self, value: P) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

/*
Todo: Push /TryPush for Option & Result ?
*/
