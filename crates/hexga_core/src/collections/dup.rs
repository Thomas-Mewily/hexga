use super::*;


/// Duplicates the last value and push it.
pub trait Dup<T>: Collection
{
    /// Duplicates the last value and push it if possible.
    fn dup(&mut self) -> bool;
}

impl<T, C> Dup<T> for C
where
    T: Clone,
    C: PeekLast<T> + Push<T>,
{
    fn dup(&mut self) -> bool
    {
        let Some(value) = PeekLast::last(self).cloned() else { return false };
        Push::push(self, value);
        true
    }
}


/// Duplicates the first value and push it..
pub trait DupFirst<T>: Collection
{
    /// Duplicates the first value and push it if possible.
    fn dup_first(&mut self) -> bool;
}
impl<T, C> DupFirst<T> for C
where
    T: Clone,
    C: PeekFirst<T> + Push<T>,
{
    fn dup_first(&mut self) -> bool
    {
        let Some(value) = PeekFirst::first(self).cloned() else { return false };
        Push::push(self, value);
        true
    }
}




/// Duplicates the last value and push_front it.
pub trait DupBack<T>: Collection
{
    /// Duplicates the last value and push_front it if possible.
    fn dup_front(&mut self) -> bool;
}

impl<T, C> DupBack<T> for C
where
    T: Clone,
    C: PeekLast<T> + PushFront<T>,
{
    fn dup_front(&mut self) -> bool
    {
        let Some(value) = PeekLast::last(self).cloned() else { return false };
        PushFront::push_front(self, value);
        true
    }
}


/// Duplicates the first value and push_front it.
pub trait DupFirstBack<T>: Collection
{
    /// Duplicates the first value and push_front it if possible.
    fn dup_front(&mut self) -> bool;
}

impl<T, C> DupFirstBack<T> for C
where
    T: Clone,
    C: PeekFirst<T> + PushFront<T>,
{
    fn dup_front(&mut self) -> bool
    {
        let Some(value) = PeekFirst::first(self).cloned() else { return false };
        PushFront::push_front(self, value);
        true
    }
}