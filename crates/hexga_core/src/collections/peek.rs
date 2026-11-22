use super::*;

pub trait PeekLast<T>: Collection
{
    /// aka `peek()`, `peek_back()`, `back()`
    fn last(&self) -> Option<&T>;
}

impl<T> PeekLast<T> for Vec<T>
{
    fn last(&self) -> Option<&T> { self.deref().last() }
}

impl<T, const N: usize> PeekLast<T> for [T;N]
{
    fn last(&self) -> Option<&T> { self.as_ref().last() }
}

impl<T> PeekLast<T> for &[T]
{
    fn last(&self) -> Option<&T> { self.as_ref().last() }
}
impl<T> PeekLast<T> for &mut [T]
{
    fn last(&self) -> Option<&T> { self.as_ref().last() }
}

impl<T> PeekLast<T> for VecDeque<T>
{
    fn last(&self) -> Option<&T> { self.back() }
}

impl<T> PeekLast<T> for LinkedList<T>
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

impl<T> PeekLast<T> for Option<T>
{
    fn last(&self) -> Option<&T> { self.as_ref() }
}

impl<T,E> PeekLast<T> for Result<T,E>
{
    fn last(&self) -> Option<&T> { self.as_ref().ok() }
}

impl<T> PeekLast<T> for BinaryHeap<T>
where
    T: Ord,
{
    fn last(&self) -> Option<&T> { self.peek() }
}



pub trait PeekFirst<T>: Collection
{
    /// aka `peek_front()`, `front()`
    fn first(&self) -> Option<&T>;
}

impl<T> PeekFirst<T> for Vec<T>
{
    fn first(&self) -> Option<&T> { self.deref().first() }
}

impl<T, const N: usize> PeekFirst<T> for [T;N]
{
    fn first(&self) -> Option<&T> { self.as_ref().first() }
}

impl<T> PeekFirst<T> for &[T]
{
    fn first(&self) -> Option<&T> { self.get(0) }
}
impl<T> PeekFirst<T> for &mut [T]
{
    fn first(&self) -> Option<&T> { self.get(0) }
}

impl<T> PeekFirst<T> for VecDeque<T>
{
    fn first(&self) -> Option<&T> { self.front() }
}

impl<T> PeekFirst<T> for LinkedList<T>
{
    fn first(&self) -> Option<&T> { self.front() }
}

impl<T> PeekFirst<T> for Option<T>
{
    fn first(&self) -> Option<&T> { self.as_ref() }
}

impl<T,E> PeekFirst<T> for Result<T,E>
{
    fn first(&self) -> Option<&T> { self.as_ref().ok() }
}



pub trait PeekLastMut<T>: PeekLast<T>
{
    /// aka `peek()`, `peek_back()`, `back()`
    fn last_mut(&mut self) -> Option<&mut T>;
}

impl<T> PeekLastMut<T> for Vec<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.deref_mut().last_mut() }
}

impl<T, const N: usize> PeekLastMut<T> for [T;N]
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut().last_mut() }
}

impl<T> PeekLastMut<T> for &mut [T]
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut().last_mut() }
}

impl<T> PeekLastMut<T> for VecDeque<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.back_mut() }
}

impl<T> PeekLastMut<T> for LinkedList<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.back_mut() }
}

impl<T> PeekLastMut<T> for Option<T>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut() }
}

impl<T,E> PeekLastMut<T> for Result<T,E>
{
    fn last_mut(&mut self) -> Option<&mut T> { self.as_mut().ok() }
}

/* 
// Not possible because BinaryHeap does not provide a mutable reference to the last element:
fn peek_mut(&mut self) -> Option<PeekMut<'_, T, A>>

It is wrapped because editing it can change the heap order.

impl<T> PeekLastMut<T> for BinaryHeap<T>
where
    T: Ord; 
*/


pub trait PeekFirstMut<T>: PeekFirst<T>
{
    /// aka `peek_front()`, `front()`
    fn first_mut(&mut self) -> Option<&mut T>;
}

impl<T> PeekFirstMut<T> for Vec<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.deref_mut().first_mut() }
}

impl<T, const N: usize> PeekFirstMut<T> for [T;N]
{
    fn first_mut(&mut self) -> Option<&mut T> { self.as_mut().first_mut() }
}

impl<T> PeekFirstMut<T> for &mut [T]
{
    fn first_mut(&mut self) -> Option<&mut T> { self.get_mut(0) }
}

impl<T> PeekFirstMut<T> for VecDeque<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<T> PeekFirstMut<T> for LinkedList<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<T> PeekFirstMut<T> for Option<T>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.as_mut() }
}

impl<T,E> PeekFirstMut<T> for Result<T,E>
{
    fn first_mut(&mut self) -> Option<&mut T> { self.as_mut().ok() }
}