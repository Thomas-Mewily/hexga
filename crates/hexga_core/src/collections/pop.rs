use super::*;

pub trait Pop<T> {
    /// Removes the last element from the collection and returns it, or [`None`] if it is empty.
    fn pop(&mut self) -> Option<T>;

    /// Pops a value and returns a mutable reference to the new last element.
    fn pop_mut<'a, Idx>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self: LastMut<Idx, Output = T>,
        T: 'a,
    {
        let _old = Pop::pop(self)?;
        LastMut::last_mut(self)
    }

    /// Removes and returns the last element from the collection if the predicate returns `true`,
    /// or [`None`] if the predicate returns false or the collection is empty (the predicate will not be called in that case).
    fn pop_if<Idx, F>(&mut self, predicate: F) -> Option<T>
    where
        Self: Last<Idx, Output = T>,
        F: FnOnce(&T) -> bool,
    {
        if predicate(self.last()?.deref()) {
            return self.pop();
        }
        None
    }
}

/*
pub trait TryPop<T> : Pop<T>
{
    type Error;
    /// Removes the last element from the collection and returns it, or an error.
    fn try_pop(&mut self) -> Result<T, Self::Error>;
}
*/
pub trait PopFront<T> {
    /// Removes the first element and returns it, or [`None`] if the collection is empty.
    fn pop_front(&mut self) -> Option<T>;

    /// Pops a value and returns a mutable reference to the new first element.
    fn pop_front_mut<'a, Idx>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self: FirstMut<Idx, Output = T>,
        T: 'a,
    {
        let _old = PopFront::pop_front(self)?;
        FirstMut::first_mut(self)
    }

    /// Removes and returns the first element from the collection if the predicate returns `true`,
    /// or [`None`] if the predicate returns false or the collection is empty (the predicate will not be called in that case).
    fn pop_front_if<Idx, F>(&mut self, predicate: F) -> Option<T>
    where
        Self: First<Idx, Output = T>,
        F: FnOnce(&T) -> bool,
    {
        if predicate(self.first()?.deref()) {
            return self.pop_front();
        }
        None
    }
}

/*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct EmptyError;
*/

impl<T, A: Allocator> Pop<T> for Vec<T, A> {
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
}
/*
impl<T> TryPop<T> for Vec<T>
{
    type Error=EmptyError;
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop().ok_or(EmptyError) }
}
*/

impl<T> Pop<T> for VecDeque<T> {
    fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }
}
impl<T> PopFront<T> for VecDeque<T> {
    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
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
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T> Pop<T> for LinkedList<T> {
    fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }
}
impl<T> PopFront<T> for LinkedList<T> {
    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
}
/*
impl<T> TryPop<T> for LinkedList<T>
{
    type Error=EmptyError;
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(()) }
}
*/

impl Pop<char> for String {
    fn pop(&mut self) -> Option<char> {
        self.pop()
    }
    /*
    fn pop_if<Idx, F>(&mut self, predicate: F) -> Option<char>
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
