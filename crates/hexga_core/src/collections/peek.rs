use super::*;



pub trait Last<T>
{
    type Guard<'a>: Deref<Target = T> where Self: 'a;

    /// aka `peek()`, `peek_back()`, `back()`
    fn last<'a>(&'a self) -> Option<Self::Guard<'a>>;
}

impl<S,T> Last<T> for &S where S: Last<T>
{
    type Guard<'a> = S::Guard<'a> where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> {
        (*self).last()
    }
}
impl<S,T> Last<T> for &mut S where S: Last<T>
{
    type Guard<'a> = S::Guard<'a> where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> {
        (**self).last()
    }
}

impl Last<char> for String
{
    type Guard<'a> = Identity<char> where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.deref().last() }
}

impl Last<char> for str
{
    type Guard<'a> = Identity<char> where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.chars().last().map(|c| Identity(c)) }
}

impl<T> Last<T> for Vec<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.deref().last() }
}

impl<T, const N: usize> Last<T> for [T;N]
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref().last() }
}

impl<T> Last<T> for &[T]
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref().last() }
}
impl<T> Last<T> for &mut [T]
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref().last() }
}

impl<T> Last<T> for VecDeque<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.back() }
}

impl<T> Last<T> for LinkedList<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.back() }
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
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref() }
}

impl<T,E> Last<T> for Result<T,E>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref().ok() }
}

impl<T> Last<T> for BinaryHeap<T>
where
    T: Ord,
{
    type Guard<'a> = &'a T where Self: 'a;

    fn last<'a>(&'a self) -> Option<Self::Guard<'a>> { self.peek() }
}



pub trait First<T>
{
    type Guard<'a>: Deref<Target = T> where Self: 'a;

    /// aka `peek_front()`, `front()`
    fn first<'a>(&'a self) -> Option<Self::Guard<'a>>;
}

impl<S,T> First<T> for &S where S: First<T>
{
    type Guard<'a> = S::Guard<'a> where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> {
        (*self).first()
    }
}
impl<S,T> First<T> for &mut S where S: First<T>
{
    type Guard<'a> = S::Guard<'a> where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> {
        (**self).first()
    }
}

impl First<char> for String
{
    type Guard<'a> = Identity<char> where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.deref().first() }
}

impl First<char> for str
{
    type Guard<'a> = Identity<char> where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.chars().next().map(|c| Identity(c)) }
}

impl<T> First<T> for Vec<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.deref().first() }
}

impl<T, const N: usize> First<T> for [T;N]
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref().first() }
}

impl<T> First<T> for &[T]
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.get(0) }
}
impl<T> First<T> for &mut [T]
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.get(0) }
}

impl<T> First<T> for VecDeque<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.front() }
}

impl<T> First<T> for LinkedList<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.front() }
}

impl<T> First<T> for Option<T>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref() }
}

impl<T,E> First<T> for Result<T,E>
{
    type Guard<'a> = &'a T where Self: 'a;

    fn first<'a>(&'a self) -> Option<Self::Guard<'a>> { self.as_ref().ok() }
}



pub trait LastMut<T>: Last<T>
{
    type GuardMut<'a>: DerefMut<Target = T> where Self: 'a;

    /// aka `peek()`, `peek_back()`, `back()`
    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>>;
}

impl<S,T> LastMut<T> for &mut S where S: LastMut<T>
{
    type GuardMut<'a> = S::GuardMut<'a> where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> {
        (**self).last_mut()
    }
}

impl<T> LastMut<T> for Vec<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.deref_mut().last_mut() }
}

impl<T, const N: usize> LastMut<T> for [T;N]
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut().last_mut() }
}

impl<T> LastMut<T> for &mut [T]
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut().last_mut() }
}

impl<T> LastMut<T> for VecDeque<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.back_mut() }
}

impl<T> LastMut<T> for LinkedList<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.back_mut() }
}

impl<T> LastMut<T> for Option<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut() }
}

impl<T,E> LastMut<T> for Result<T,E>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut().ok() }
}


impl<T> LastMut<T> for BinaryHeap<T> where
    T: Ord
{
    type GuardMut<'a> = collections::binary_heap::PeekMut<'a, T> where Self: 'a;

    fn last_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.peek_mut() }
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
    type GuardMut<'a>: DerefMut<Target = T> where Self: 'a;

    /// aka `peek_front()`, `front()`
    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>>;
}

impl<S,T> FirstMut<T> for &mut S where S: FirstMut<T>
{
    type GuardMut<'a> = S::GuardMut<'a> where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> {
        (**self).first_mut()
    }
}

impl<T> FirstMut<T> for Vec<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.deref_mut().first_mut() }
}

impl<T, const N: usize> FirstMut<T> for [T;N]
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut().first_mut() }
}

impl<T> FirstMut<T> for &mut [T]
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.get_mut(0) }
}

impl<T> FirstMut<T> for VecDeque<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.front_mut() }
}

impl<T> FirstMut<T> for LinkedList<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.front_mut() }
}

impl<T> FirstMut<T> for Option<T>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut() }
}

impl<T,E> FirstMut<T> for Result<T,E>
{
    type GuardMut<'a> = &'a mut T where Self: 'a;

    fn first_mut<'a>(&'a mut self) -> Option<Self::GuardMut<'a>> { self.as_mut().ok() }
}
