use super::*;



pub trait Pop<T> : Collection
{
    /// Removes the last element from the collection and returns it, or [`None`] if it is empty.
    fn pop(&mut self) -> Option<T>;
    /// Removes and returns the last element from the collection if the predicate returns `true``,
    /// or [`None`] if the predicate returns false or the collection is empty (the predicate will not be called in that case).
    fn pop_if<F>(&mut self, predicate: F) -> Option<T> where F: FnOnce(&mut T) -> bool;
}
/*
pub trait TryPop<T> : Pop<T>
{
    type Error: Debug;
    /// Removes the last element from the collection and returns it, or an error.
    fn try_pop(&mut self) -> Result<T, Self::Error>;
}
*/
pub trait PopFront<T> : Collection
{
    /// Removes the first element and returns it, or [`None`] if the collection is empty.
    fn pop_front(&mut self) -> Option<T>;
    /// Removes and returns the first element from the collection if the predicate returns `true``,
    /// or [`None`] if the predicate returns false or the collection is empty (the predicate will not be called in that case).
    fn pop_front_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T>;
}

/*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct EmptyError;
*/


impl<T> Pop<T> for Vec<T>
{
    fn pop(&mut self) -> Option<T> { self.pop() }
    fn pop_if<F>(&mut self, predicate: F) -> Option<T> where F: FnOnce(&mut T) -> bool { self.pop_if(predicate) }
}
/*
impl<T> TryPop<T> for Vec<T>
{
    type Error=EmptyError;
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop().ok_or(EmptyError) }
}
*/


impl<T> Pop<T> for VecDeque<T>
{
    fn pop(&mut self) -> Option<T> { self.pop_back() }
    fn pop_if<F>(&mut self, predicate: F) -> Option<T> where F: FnOnce(&mut T) -> bool { self.pop_back_if(predicate) }

}
impl<T> PopFront<T> for VecDeque<T>
{
    fn pop_front(&mut self) -> Option<T> { self.pop_front() }
    fn pop_front_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> { self.pop_front_if(predicate) }
}
/*
impl<T> TryPop<T> for VecDeque<T>
{
    type Error=EmptyError;
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(EmptyError) }
}
*/

impl<T> Pop<T> for BinaryHeap<T> where T: Ord
{
    fn pop(&mut self) -> Option<T> { self.pop() }
    fn pop_if<F>(&mut self, predicate: F) -> Option<T> where F: FnOnce(&mut T) -> bool
    {
        let Some(mut v) = self.pop() else { return None; };
        if predicate(&mut v)
        {
            return Some(v);
        }
        self.push(v);
        None
    }

}


impl<T> Pop<T> for LinkedList<T>
{
    fn pop(&mut self) -> Option<T> { self.pop_back() }
    fn pop_if<F>(&mut self, predicate: F) -> Option<T> where F: FnOnce(&mut T) -> bool
    {
        let Some(mut v) = self.pop_back() else { return None; };
        if predicate(&mut v)
        {
            return Some(v);
        }
        self.push_back(v);
        None
    }
}
impl<T> PopFront<T> for LinkedList<T>
{
    fn pop_front(&mut self) -> Option<T> { self.pop_front() }
    fn pop_front_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        let Some(mut v) = self.pop_front() else { return None; };
        if predicate(&mut v)
        {
            return Some(v);
        }
        self.push_front(v);
        None
    }
}
/*
impl<T> TryPop<T> for LinkedList<T>
{
    type Error=EmptyError;
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(()) }
}
*/



impl Pop<char> for String
{
    fn pop(&mut self) -> Option<char> { self.pop() }
    fn pop_if<F>(&mut self, predicate: F) -> Option<char> where F: FnOnce(&mut char) -> bool {
        let Some(mut v) = self.pop() else { return None; };
        if predicate(&mut v)
        {
            return Some(v);
        }
        self.push(v);
        None
    }
}
/*
impl<T> TryPop<T> for String<T>
{
    type Error=EmptyError;
}
*/