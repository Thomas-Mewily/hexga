use super::*;


/// Duplicates the last value and push it.
pub trait Dup<T>: Collection<T>
{
    /// Duplicates the last value and push it if possible.
    fn dup(&mut self) -> bool;
}

impl<T, C> Dup<T> for C
where
    T: Clone,
    C: Last<T> + Push<T>,
{
    fn dup(&mut self) -> bool
    {
        let Some(value) = Last::last(self).map(|value| value.deref().clone()) else { return false };
        Push::push(self, value);
        true
    }
}


/// Duplicates the first value and push it..
pub trait DupFirst<T>: Collection<T>
{
    /// Duplicates the first value and push it if possible.
    fn dup_first(&mut self) -> bool;
}
impl<T, C> DupFirst<T> for C
where
    T: Clone,
    C: First<T> + Push<T>,
{
    fn dup_first(&mut self) -> bool
    {
        let Some(value) = First::first(self).map(|value| value.deref().clone()) else { return false };
        Push::push(self, value);
        true
    }
}




/// Duplicates the last value and push_front it.
pub trait DupBack<T>: Collection<T>
{
    /// Duplicates the last value and push_front it if possible.
    fn dup_front(&mut self) -> bool;
}

impl<T, C> DupBack<T> for C
where
    T: Clone,
    C: Last<T> + PushFront<T>,
{
    fn dup_front(&mut self) -> bool
    {
        let Some(value) = Last::last(self).map(|value| value.deref().clone()) else { return false };
        PushFront::push_front(self, value);
        true
    }
}


/// Duplicates the first value and push_front it.
pub trait DupFirstBack<T>: Collection<T>
{
    /// Duplicates the first value and push_front it if possible.
    fn dup_front(&mut self) -> bool;
}

impl<T, C> DupFirstBack<T> for C
where
    T: Clone,
    C: First<T> + PushFront<T>,
{
    fn dup_front(&mut self) -> bool
    {
        let Some(value) = First::first(self).map(|value| value.deref().clone()) else { return false };
        PushFront::push_front(self, value);
        true
    }
}
