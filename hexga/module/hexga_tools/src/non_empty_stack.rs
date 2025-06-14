use super::*;

/// A stack that ALWAY have one element.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NonEmptyStack<T>
{
    /// will be readed frequently
    last : T,
    stack : Vec<T>
}

impl<T> Deref for NonEmptyStack<T> { type Target = T; fn deref(&self) -> &Self::Target { &self.last }}
impl<T> DerefMut for NonEmptyStack<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.last }}

/// A stack that always have at least one element,
/// where the last element can be frequently accessed
#[allow(clippy::len_without_is_empty)]
impl<T> NonEmptyStack<T>
{
    pub const fn new(value : T) -> Self { Self { last: value, stack: Vec::new() }}
    pub fn with_capacity(value : T, capacity : usize) -> Self { Self { last: value, stack: Vec::with_capacity(capacity.saturating_sub(1)) }}
    pub fn from_vec(mut stack : Vec<T>) -> Option<Self> { stack.pop().and_then(|last| Some(Self{ last, stack })) }

    pub fn len(&self) -> usize { self.stack.len() + 1 }

    pub const fn last(&self) -> &T { &self.last }
    pub const fn last_mut(&mut self) -> &mut T { &mut self.last }

    /// Clear the stack and keep the last element
    /// ```
    /// use hexga_tools::prelude::*;
    ///
    /// let mut stack = NonEmptyStack::from_vec(vec![1,2,3,4]).unwrap();
    /// stack.clear_and_keep_last();
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.last(), &4);
    /// ```
    pub fn clear_and_keep_last(&mut self) { self.stack.clear() }

    /// Clear the stack and keep the first element
    /// ```
    /// use hexga_tools::prelude::*;
    ///
    /// let mut stack = NonEmptyStack::from_vec(vec![1,2,3,4]).unwrap();
    /// stack.clear_and_keep_first();
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.last(), &1);
    /// ```
    pub fn clear_and_keep_first(&mut self)
    {
        if self.stack.is_empty() { return; }
        std::mem::swap(&mut self.last, &mut self.stack[0]);
        self.stack.clear();
    }

    /// Replace the last value
    pub fn replace(&mut self, mut value : T) -> T { std::mem::swap(&mut self.last, &mut value); value }
    pub fn push(&mut self, mut value : T) { std::mem::swap(&mut self.last, &mut value); self.stack.push(value); }
    /// Clone the last element and push it
    pub fn duplicate(&mut self) -> &mut Self where T: Clone { self.stack.push(self.last.clone()); self }
    pub fn pop(&mut self) -> Option<T> { self.stack.pop().and_then(|mut v| { std::mem::swap(&mut v, &mut self.last); Some(v) }) }

    pub fn iter(&self) -> Iter<'_, T> { Iter { stack: self.stack.iter(), last: Some(&self.last) } }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { IterMut { stack: self.stack.iter_mut(), last: Some(&mut self.last) } }

    pub fn into_values(mut self) -> Vec<T> { self.stack.push(self.last); self.stack }
}

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    stack: std::vec::IntoIter<T>,
    last: Option<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.next().or_else(|| self.last.take())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.stack.len() + self.last.is_some() as usize;
        (len, Some(len))
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.last.is_some() && self.stack.len() == 0 {
            self.last.take()
        } else {
            self.stack.next_back()
        }
    }
}
impl<T> std::iter::FusedIterator for IntoIter<T> {}
impl<T> std::iter::ExactSizeIterator for IntoIter<T> { fn len(&self) -> usize { self.stack.len() + self.last.is_some() as usize } }

impl<T> IntoIterator for NonEmptyStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            stack: self.stack.into_iter(),
            last: Some(self.last),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    stack: std::slice::Iter<'a, T>,
    last: Option<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.next().or_else(|| self.last.take())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.stack.len() + self.last.is_some() as usize;
        (len, Some(len))
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.last.is_some() && self.stack.len() == 0 {
            self.last.take()
        } else {
            self.stack.next_back()
        }
    }
}
impl<'a,T> std::iter::FusedIterator for Iter<'a,T> {}
impl<'a,T> std::iter::ExactSizeIterator for Iter<'a,T> { fn len(&self) -> usize { self.stack.len() + self.last.is_some() as usize } }

impl<'a, T> IntoIterator for &'a NonEmptyStack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            stack: self.stack.iter(),
            last: Some(&self.last),
        }
    }
}



#[derive(Debug)]
pub struct IterMut<'a, T> {
    stack: std::slice::IterMut<'a, T>,
    last: Option<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.next().or_else(|| self.last.take())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.stack.len() + self.last.is_some() as usize;
        (len, Some(len))
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.last.is_some() && self.stack.len() == 0 {
            self.last.take()
        } else {
            self.stack.next_back()
        }
    }
}
impl<'a,T> std::iter::FusedIterator for IterMut<'a,T> {}
impl<'a,T> std::iter::ExactSizeIterator for IterMut<'a,T> { fn len(&self) -> usize { self.stack.len() + self.last.is_some() as usize } }

impl<'a, T> IntoIterator for &'a mut NonEmptyStack<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            stack: self.stack.iter_mut(),
            last: Some(&mut self.last),
        }
    }
}


impl<T> Index<usize> for NonEmptyStack<T>
{
    type Output=T;

    fn index(&self, index: usize) -> &Self::Output
    {
        if index + 1 == self.len()
        {
            &self.last
        }else
        {
            &self.stack[index]
        }
    }
}
impl<T> IndexMut<usize> for NonEmptyStack<T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index + 1 == self.len()
        {
            &mut self.last
        }else
        {
            &mut self.stack[index]
        }
    }
}

impl<T> Length for NonEmptyStack<T>
{
    fn len(&self) -> usize { self.len() }
    fn is_empty(&self) -> bool { false }
    fn is_not_empty(&self) -> bool { true }
}

impl<T> Capacity for NonEmptyStack<T>
{
    type Param=T;

    fn capacity(&self) -> usize { self.stack.capacity() + 1 }

    fn with_capacity_and_param(capacity: usize, value : Self::Param) -> Self { Self::with_capacity(value, capacity) }

    fn reserve(&mut self, additional: usize) { self.stack.reserve(additional.saturating_sub(1)); }
    fn reserve_exact(&mut self, additional: usize) { self.stack.reserve_exact(additional.saturating_sub(1)); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.stack.try_reserve(additional.saturating_sub(1)) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.stack.try_reserve_exact(additional.saturating_sub(1)) }
}

#[cfg(test)]
mod non_empty_stack_test
{
    use super::*;

    #[test]
    fn push() {
        let mut s = NonEmptyStack::new(42);
        assert_eq!(s.len(), 1);
        assert_eq!(s.last(), &42);

        s.push(50);
        assert_eq!(s.len(), 2);
        assert_eq!(s.last(), &50);

        s.duplicate();
        assert_eq!(s.len(), 3);
        assert_eq!(s.last(), &50);

        assert_eq!(s.pop(), Some(50));
        assert_eq!(s.len(), 2);

        assert_eq!(s.pop(), Some(50));
        assert_eq!(s.len(), 1);

        assert_eq!(s.pop(), None);
        assert_eq!(s.len(), 1);
    }
}