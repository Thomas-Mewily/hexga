use super::*;

pub trait Push<T>: Collection
{
    type Output;
    /// Push an value and return it's index.
    fn push(&mut self, value: T) -> Self::Output;

    //fn push_mut ...
}
pub trait TryPush<T>: Push<T>
{
    type Error: Debug;
    /// Push an value and return it's index.
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error>;
}

pub trait PushFront<T>: Collection
{
    type Output;
    /// Push an value and return it's index.
    fn push_front(&mut self, value: T) -> Self::Output;
}
pub trait TryPushFront<T>: PushFront<T>
{
    type Error: Debug;
    /// Push an value and return it's index.
    fn try_push_front(&mut self, value: T) -> Result<Self::Output, Self::Error>;
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
    type Error=(); // #proper_error
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
    type Error=(); // #proper_error
    fn try_push_front(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}



impl<T> Push<T> for LinkedList<T>
{
    type Output = ();
    fn push(&mut self, value: T) -> Self::Output { self.push_back(value); }
}
impl<T> TryPush<T> for LinkedList<T>
{
    type Error=();
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}
impl<T> PushFront<T> for LinkedList<T>
{
    type Output = (); // #proper_error
    fn push_front(&mut self, value: T) -> Self::Output { self.push_front(value); }
}
impl<T> TryPushFront<T> for LinkedList<T>
{
    type Error=(); // #proper_error
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
    type Error=(); // #proper_error
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
    fn try_push(&mut self, value: char) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

impl<'b> Push<&'b OsStr> for OsString
{
    type Output = ();
    fn push(&mut self, value: &'b OsStr) -> Self::Output { self.push(value); }
}
impl<'b> TryPush<&'b OsStr> for OsString
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value: &'b OsStr) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}

impl<P> Push<P> for PathBuf
where
    P: AsRef<Path>,
{
    type Output = ();
    fn push(&mut self, value: P) -> Self::Output { self.push(value); }
}
impl<P> TryPush<P> for PathBuf where P: AsRef<Path>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value: P) -> Result<Self::Output, Self::Error> { Ok(self.push(value)) }
}
