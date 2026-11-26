use super::*;

/*
Todo: make a trait IndexLast, IndexFirst ? (to return the index)
*/

pub trait Last<Idx>: CollectionIndex<Idx> {
    /// aka `peek()`, `peek_back()`, `back()`
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a;
}

impl<S: ?Sized, Idx> Last<Idx> for &S
where
    S: Last<Idx>,
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        (*self).last()
    }
}
impl<S: ?Sized, Idx> Last<Idx> for &mut S
where
    S: Last<Idx>,
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        (**self).last()
    }
}

impl Last<usize> for String {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.deref().last()
    }
}

impl Last<usize> for str {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.chars().last().map(|c| Identity(c))
    }
}

impl<T, A: Allocator> Last<usize> for Vec<T, A> {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.deref().last()
    }
}

impl<T, const N: usize> Last<usize> for [T; N] {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.as_ref().last()
    }
}

impl<T> Last<usize> for [T] {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        <[T]>::last(self)
    }
}

impl<T> Last<usize> for VecDeque<T> {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.back()
    }
}

impl<T> Last<usize> for LinkedList<T> {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.back()
    }
}

impl<T> Last<()> for Option<T> {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.as_ref()
    }
}

impl<T, E> Last<ResultOk> for Result<T, E> {
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.as_ref().ok()
    }
}

impl<T> Last<()> for BinaryHeap<T>
where
    T: Ord,
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.peek()
    }
}

pub trait First<Idx>: CollectionIndex<Idx> {
    /// aka `peek_front()`, `front()`
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a;
}

impl<S: ?Sized, Idx> First<Idx> for &S
where
    S: First<Idx>,
{
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        (*self).first()
    }
}
impl<S: ?Sized, Idx> First<Idx> for &mut S
where
    S: First<Idx>,
{
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        (**self).first()
    }
}

impl First<usize> for String {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.deref().first()
    }
}

impl First<usize> for str {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.chars().next().map(|c| Identity(c))
    }
}

impl<T, A: Allocator> First<usize> for Vec<T, A> {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.deref().first()
    }
}

impl<T, const N: usize> First<usize> for [T; N] {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.as_ref().first()
    }
}

impl<T> First<usize> for [T] {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        <[T]>::first(self)
    }
}

impl<T> First<usize> for VecDeque<T> {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.front()
    }
}

impl<T> First<usize> for LinkedList<T> {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.front()
    }
}

impl<T> First<()> for Option<T> {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.as_ref()
    }
}

impl<T, E> First<ResultOk> for Result<T, E> {
    fn first<'a>(&'a self) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.as_ref().ok()
    }
}

pub trait LastMut<Idx>: Last<Idx> + CollectionIndexMut<Idx> {
    /// aka `peek()`, `peek_back()`, `back()`
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a;
}

impl<S: ?Sized, Idx> LastMut<Idx> for &mut S
where
    S: LastMut<Idx>,
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        (**self).last_mut()
    }
}

impl<T, A: Allocator> LastMut<usize> for Vec<T, A> {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.deref_mut().last_mut()
    }
}

impl<T, const N: usize> LastMut<usize> for [T; N] {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.as_mut().last_mut()
    }
}

impl<T> LastMut<usize> for [T] {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        <[T]>::last_mut(self)
    }
}

impl<T> LastMut<usize> for VecDeque<T> {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.back_mut()
    }
}

impl<T> LastMut<usize> for LinkedList<T> {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.back_mut()
    }
}

impl<T> LastMut<()> for Option<T> {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.as_mut()
    }
}

impl<T, E> LastMut<ResultOk> for Result<T, E> {
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.as_mut().ok()
    }
}

impl<T> LastMut<()> for BinaryHeap<T>
where
    T: Ord,
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.peek_mut()
    }
}

pub trait FirstMut<Idx>: First<Idx> + CollectionIndexMut<Idx> {
    /// aka `peek_front()`, `front()`
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a;
}

impl<S: ?Sized, Idx> FirstMut<Idx> for &mut S
where
    S: FirstMut<Idx>,
{
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        (**self).first_mut()
    }
}

impl<T, A: Allocator> FirstMut<usize> for Vec<T, A> {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.deref_mut().first_mut()
    }
}

impl<T, const N: usize> FirstMut<usize> for [T; N] {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.as_mut().first_mut()
    }
}

impl<T> FirstMut<usize> for [T] {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        <[T]>::first_mut(self)
    }
}

impl<T> FirstMut<usize> for VecDeque<T> {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.front_mut()
    }
}

impl<T> FirstMut<usize> for LinkedList<T> {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.front_mut()
    }
}

impl<T> FirstMut<()> for Option<T> {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.as_mut()
    }
}

impl<T, E> FirstMut<ResultOk> for Result<T, E> {
    fn first_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.as_mut().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushes_and_pops_return_the_indexed_guard() {
        let mut values = Vec::from([1]);
        *Push::push_mut(&mut values, 2).unwrap() = 3;
        assert!(Dup::<usize>::dup(&mut values));
        assert_eq!(Last::last(&values), Some(&3));
        *values.pop_mut().unwrap() = 4;
        assert_eq!(values, [1, 4]);
    }

    #[test]
    fn peeking_preserves_non_reference_guards() {
        assert_eq!(First::first("é🦀"), Some(Identity('é')));
        assert_eq!(Last::last("é🦀"), Some(Identity('🦀')));

        let mut heap = BinaryHeap::from([2, 3]);
        *LastMut::last_mut(&mut heap).unwrap() = 1;
        assert_eq!(Last::last(&heap), Some(&2));
    }
}
