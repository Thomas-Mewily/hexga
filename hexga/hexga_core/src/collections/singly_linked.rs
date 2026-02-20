use super::*;

pub mod prelude
{
    pub use super::SinglyLinkedNode;
}

#[derive(Debug)]
pub struct SinglyLinkedNode<T>
{
    pub next: Option<Box<Self>>,
    pub value: T,
}

impl<T> Collection for SinglyLinkedNode<T> {}

impl<T> Push<T> for SinglyLinkedNode<T>
{
    type Output = ();
    fn push(&mut self, value: T) -> Self::Output { self.push(value); }
}
impl<T> TryPush<T> for SinglyLinkedNode<T>
{
    type Error = Never;
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error>
    {
        self.push(value);
        Ok(())
    }
}

impl<T> Deref for SinglyLinkedNode<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for SinglyLinkedNode<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl<T> From<T> for SinglyLinkedNode<T>
{
    fn from(value: T) -> Self { Self::new(value) }
}

impl<T> SinglyLinkedNode<T>
{
    pub fn new(value: T) -> Self { Self { next: None, value } }

    pub fn iter<'a>(head: &'a Option<Box<Self>>) -> Iter<'a, T>
    {
        Iter {
            current: head.as_deref(),
        }
    }

    pub fn iter_mut<'a>(head: &'a mut Option<Box<Self>>) -> IterMut<'a, T>
    {
        IterMut {
            current: head.as_deref_mut(),
        }
    }

    pub fn replace(&mut self, value: T) -> T { std::mem::replace(&mut self.value, value) }

    /// Replace the current value and push the old value as the next node
    pub fn push(&mut self, value: T)
    {
        let old_value = std::mem::replace(&mut self.value, value);
        self.push_next(old_value);
    }

    /// Push a new node immediately after this node
    pub fn push_next(&mut self, value: T)
    {
        let mut new_node = Box::new(Self::new(value));
        new_node.next = self.next.take();
        self.next = Some(new_node);
    }

    /// Pop the node immediately after this one, returning its value
    pub fn pop_next(&mut self) -> Option<T>
    {
        self.next.take().map(|mut node| {
            self.next = node.next.take();
            node.value
        })
    }

    pub fn pop(self) -> (T, Option<Self>) { (self.value, self.next.map(|v| *v)) }
}

pub struct Iter<'a, T>
{
    current: Option<&'a SinglyLinkedNode<T>>,
}
impl<'a, T> Clone for Iter<'a, T>
{
    fn clone(&self) -> Self
    {
        Self {
            current: self.current.clone(),
        }
    }
}

impl<'a, T> Iter<'a, T>
{
    pub fn new(head: &'a SinglyLinkedNode<T>) -> Self
    {
        Self {
            current: Some(head),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some(&node.value)
    }
}

pub struct IterMut<'a, T>
{
    current: Option<&'a mut SinglyLinkedNode<T>>,
}

impl<'a, T> IterMut<'a, T>
{
    pub fn new(head: &'a mut SinglyLinkedNode<T>) -> Self
    {
        Self {
            current: Some(head),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let node = self.current.take()?;
        self.current = node.next.as_deref_mut();
        Some(&mut node.value)
    }
}

pub struct IntoIter<T>
{
    current: Option<Box<SinglyLinkedNode<T>>>,
}

impl<T> IntoIter<T>
{
    pub fn new(head: Option<Box<SinglyLinkedNode<T>>>) -> Self { Self { current: head } }
}

impl<T> Iterator for IntoIter<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut node = self.current.take()?;
        self.current = node.next.take();
        Some(node.value)
    }
}
