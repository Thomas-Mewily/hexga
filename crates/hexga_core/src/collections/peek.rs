use super::*;

pub trait Last<T>: Collection
{
    /// aka `peek()`, `peek_back()`, `back()`
    fn last(&self) -> Option<&T>;
}

impl<T> Last<T> for Vec<T>
{
    fn last(&self) -> Option<&T> { self.deref().last() }
}

impl<T, const N: usize> Last<T> for [T;N]
{
    fn last(&self) -> Option<&T> { self.as_ref().last() }
}

impl<T> Last<T> for &[T]
{
    fn last(&self) -> Option<&T> { self.as_ref().last() }
}
impl<T> Last<T> for &mut [T]
{
    fn last(&self) -> Option<&T> { self.as_ref().last() }
}

impl<T> Last<T> for VecDeque<T>
{
    fn last(&self) -> Option<&T> { self.back() }
}

impl<T> Last<T> for LinkedList<T>
{
    fn last(&self) -> Option<&T> { self.back() }
}

/*
last return by value, not by reference
impl PeekLast<char> for String
{
    fn last(&self) -> Option<&char> { self.back() }
}
impl PeekLast<char> for &str
{
    fn last(&self) -> Option<&char> { self.chars().last() }
}
*/

impl<T> Last<T> for Option<T>
{
    fn last(&self) -> Option<&T> { self.as_ref() }
}

impl<T,E> Last<T> for Result<T,E>
{
    fn last(&self) -> Option<&T> { self.as_ref().ok() }
}

impl<T> Last<T> for BinaryHeap<T>
where
    T: Ord,
{
    fn last(&self) -> Option<&T> { self.peek() }
}



pub trait First<T>: Collection
{
    /// aka `peek_front()`, `front()`
    fn first(&self) -> Option<&T>;
}

impl<T> First<T> for Vec<T>
{
    fn first(&self) -> Option<&T> { self.deref().first() }
}

impl<T, const N: usize> First<T> for [T;N]
{
    fn first(&self) -> Option<&T> { self.as_ref().first() }
}

impl<T> First<T> for &[T]
{
    fn first(&self) -> Option<&T> { self.get(0) }
}
impl<T> First<T> for &mut [T]
{
    fn first(&self) -> Option<&T> { self.get(0) }
}

impl<T> First<T> for VecDeque<T>
{
    fn first(&self) -> Option<&T> { self.front() }
}

impl<T> First<T> for LinkedList<T>
{
    fn first(&self) -> Option<&T> { self.front() }
}

impl<T> First<T> for Option<T>
{
    fn first(&self) -> Option<&T> { self.as_ref() }
}

impl<T,E> First<T> for Result<T,E>
{
    fn first(&self) -> Option<&T> { self.as_ref().ok() }
}



pub trait LastMut<T>: Last<T>
{
    /// aka `peek()`, `peek_back()`, `back()`
    fn last_mut(&mut self) -> Option<&mut T>;
}

impl<T> LastMut<T> for Vec<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.deref_mut().last_mut() }
}

impl<T, const N: usize> LastMut<T> for [T;N]
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut().last_mut() }
}

impl<T> LastMut<T> for &mut [T]
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut().last_mut() }
}

impl<T> LastMut<T> for VecDeque<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.back_mut() }
}

impl<T> LastMut<T> for LinkedList<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.back_mut() }
}

impl<T> LastMut<T> for Option<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut() }
}

impl<T,E> LastMut<T> for Result<T,E>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut().ok() }
}

/* 
// Not possible because BinaryHeap does not provide a mutable reference to the last element:
fn peek_mut(&mut self) -> Option<PeekMut<'_, T, A>>

It is wrapped because editing it can change the heap order.

impl<T> LastMut<T> for BinaryHeap<T>
where
    T: Ord; 
*/


pub trait FirstMut<T>: First<T>
{
    /// aka `peek_front()`, `front()`
    fn first_mut(&mut self) -> Option<&mut T>;
}

impl<T> FirstMut<T> for Vec<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.deref_mut().first_mut() }
}

impl<T, const N: usize> FirstMut<T> for [T;N]
{
    fn first_mut(&mut self) -> Option<&mut T> { self.as_mut().first_mut() }
}

impl<T> FirstMut<T> for &mut [T]
{
    fn first_mut(&mut self) -> Option<&mut T> { self.get_mut(0) }
}

impl<T> FirstMut<T> for VecDeque<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<T> FirstMut<T> for LinkedList<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<T> FirstMut<T> for Option<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.as_mut() }
}

impl<T,E> FirstMut<T> for Result<T,E>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.as_mut().ok() }
}