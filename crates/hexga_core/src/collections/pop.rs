use super::*;

pub trait Pop<T>: Collection
{
    /// Removes the last element from the collection and returns it, or [`None`] if it is empty.
    fn pop(&mut self) -> Option<T>;
}

pub trait PopIf<T> : Pop<T> + PeekLast<T>
{
    /// Removes and returns the last element from the collection if the predicate returns `true`,
    /// or [`None`] if the predicate returns false or the collection is empty (the predicate will not be called in that case).
    fn pop_if<F>(&mut self, predicate: F) -> Option<T>
    where
        F: FnOnce(&T) -> bool,
    {
        let Some(last) = self.last() else { return None; };
        if predicate(last)
        {
            return self.pop();
        }
        None
    }
}
impl<S,T> PopIf<T> for S where S: Pop<T> + PeekLast<T> {}



/*
pub trait TryPop<T> : Pop<T>
{
    type Error;
    /// Removes the last element from the collection and returns it, or an error.
    fn try_pop(&mut self) -> Result<T, Self::Error>;
}
*/
pub trait PopFront<T>: Collection
{
    /// Removes the first element and returns it, or [`None`] if the collection is empty.
    fn pop_front(&mut self) -> Option<T>;
}

pub trait PopFrontIf<T> : Pop<T> + PeekFirst<T>
{
    /// Removes and returns the first element from the collection if the predicate returns `true`,
    /// or [`None`] if the predicate returns false or the collection is empty (the predicate will not be called in that case).
    fn pop_front_if<F>(&mut self, predicate: F) -> Option<T>
    where
        F: FnOnce(&T) -> bool,
    {
        let Some(first) = self.first() else { return None; };
        if predicate(first)
        {
            return self.pop();
        }
        None
    }
}
impl<S,T> PopFrontIf<T> for S where S: Pop<T> + PeekFirst<T> {}



/*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct EmptyError;
*/

impl<T> Pop<T> for Vec<T>
{
    fn pop(&mut self) -> Option<T> { self.pop() }
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
}
impl<T> PopFront<T> for VecDeque<T>
{
    fn pop_front(&mut self) -> Option<T> { self.pop_front() }
}
/*
impl<T> TryPop<T> for VecDeque<T>
{
    type Error=EmptyError;
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(EmptyError) }
}
*/

impl<T> Pop<T> for BinaryHeap<T>
where
    T: Ord,
{
    fn pop(&mut self) -> Option<T> { self.pop() }
}

impl<T> Pop<T> for LinkedList<T>
{
    fn pop(&mut self) -> Option<T> { self.pop_back() }
}
impl<T> PopFront<T> for LinkedList<T>
{
    fn pop_front(&mut self) -> Option<T> { self.pop_front() }
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
    /*
    fn pop_if<F>(&mut self, predicate: F) -> Option<char>
    where
        F: FnOnce(&mut char) -> bool,
    {
        let Some(mut v) = self.pop()
        else
        {
            return None;
        };
        if predicate(&mut v)
        {
            return Some(v);
        }
        self.push(v);
        None
    }
    */
}
/*
impl<T> TryPop<T> for String<T>
{
    type Error=EmptyError;
}
*/
