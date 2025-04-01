use super::*;

/// A stack that ALWAY have one element.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
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
    pub fn with_capacity(value : T, capacity : usize) -> Self { Self { last: value, stack: Vec::with_capacity(capacity) }}
    pub fn from_vec(mut stack : Vec<T>) -> Option<Self> { stack.pop().and_then(|last| Some(Self{ last, stack })) }

    pub const fn last(&self) -> &T { &self.last }
    pub const fn last_mut(&mut self) -> &mut T { &mut self.last }

    /// Clear the stack and keep the last element
    /// ```
    /// use hexga_tools::*;
    /// 
    /// let mut stack = NonEmptyStack::from_vec(vec![1,2,3,4]).unwrap();
    /// stack.clear_and_keep_last();
    /// assert_eq!(stack.len(), 1);
    /// assert_eq!(stack.last(), &4);
    /// ```
    pub fn clear_and_keep_last(&mut self) { self.stack.clear() }

    /// Clear the stack and keep the first element
    /// ```
    /// use hexga_tools::*;
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
    pub fn duplicate(&mut self) -> &mut Self where T : Clone { self.stack.push(self.last.clone()); self } 
    pub fn pop(&mut self) -> Option<T> { self.stack.pop().and_then(|mut v| { std::mem::swap(&mut v, &mut self.last); Some(v) }) }

    pub fn len(&self) -> usize  { self.stack.len() + 1 } 

    pub fn into_values(mut self) -> Vec<T> { self.stack.push(self.last); self.stack }
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

impl<T> have_len::HaveLen for NonEmptyStack<T>
{
    fn len(&self) -> usize { self.len() }

    fn is_empty(&self) -> bool { false }
    fn is_not_empty(&self) -> bool { true }
}

#[cfg(test)]
mod non_empty_stack_test
{
    use super::NonEmptyStack;

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