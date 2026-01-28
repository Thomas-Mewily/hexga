use super::*;


pub trait Push<T> : Collection
{
    type Output;
    /// Push an item and return it's index.
    fn push(&mut self, item: T) -> Self::Output;

    //fn push_mut ...
}
pub trait PushFront<T> : Collection
{
    type Output;
    /// Push an item and return it's index.
    fn push_front(&mut self, item: T) -> Self::Output;
}
/*
pub trait TryPush<T>: Push<T>
{
    type Error: Debug;
    /// Push an item and return it's index.
    fn try_push(&mut self, item: T) -> Result<Self::Output, Self::Error>;
}
*/

impl<T> Push<T> for Vec<T>
{
    type Output = usize;
    #[inline(always)]
    fn push(&mut self, item: T) -> Self::Output
    {
        let l = self.len();
        self.push(item);
        l
    }
}

/*
impl<T> TryPush<T> for Vec<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, item)) }
}
*/

impl<T> Push<T> for VecDeque<T>
{
    type Output = usize;
    fn push(&mut self, item: T) -> Self::Output
    {
        let l = self.len();
        self.push_back(item);
        l
    }
}
impl<T> PushFront<T> for VecDeque<T>
{
    type Output = usize;
    fn push_front(&mut self, item: T) -> Self::Output
    {
        self.push_front(item);
        0
    }
}
/*
impl<T> TryPush<T> for VecDeque<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, item)) }
}
*/

impl<T> Push<T> for LinkedList<T>
{
    type Output = ();
    fn push(&mut self, item: T) -> Self::Output {
        self.push_back(item);
    }
}
impl<T> PushFront<T> for LinkedList<T>
{
    type Output = ();
    fn push_front(&mut self, item: T) -> Self::Output {
        self.push_front(item);
    }
}
/*
impl<T> TryPush<T> for LinkedList<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: T) -> Result<Self::Output, Self::Error> {
        self.push(item);
        Ok(())
    }
}
*/

impl<T> Push<T> for BinaryHeap<T> where T: Ord
{
    type Output = ();
    fn push(&mut self, item: T) -> Self::Output {
        self.push(item);
    }
}
/*
impl<T> TryPush<T> for BinaryHeap<T> where T: Ord
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: T) -> Result<Self::Output, Self::Error> {
        self.push(item);
        Ok(())
    }
}
*/

impl Push<char> for String
{
    type Output = ();
    fn push(&mut self, item: char) -> Self::Output {
        self.push(item);
    }
}
/*
impl TryPush<char> for String
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: char) -> Result<Self::Output, Self::Error> {
        self.push(item);
        Ok(())
    }
}
*/

impl<'b> Push<&'b OsStr> for OsString
{
    type Output = ();
    fn push(&mut self, item: &'b OsStr) -> Self::Output {
        self.push(item);
    }
}
/*
impl<'b> TryPush<&'b OsStr> for OsString
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: &'b OsStr) -> Result<Self::Output, Self::Error> {
        self.push(item);
        Ok(())
    }
}
*/

impl<P> Push<P> for PathBuf where P: AsRef<Path>
{
    type Output = ();
    fn push(&mut self, item: P) -> Self::Output {
        self.push(item);
    }
}
/*
impl<P> TryPush<P> for PathBuf where P: AsRef<Path>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, item: P) -> Result<Self::Output, Self::Error> {
        self.push(item);
        Ok(())
    }
}
*/