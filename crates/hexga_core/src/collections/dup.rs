use super::*;


/// Duplicates the last value and push it.
pub trait Dup<Idx>: Collection<Idx>
{
    /// Duplicates the last value and push it if possible.
    fn dup(&mut self) -> bool;
}

impl<Idx, C> Dup<Idx> for C
where
    C: Last<Idx> + Push<<C as Collection<Idx>>::Output>,
    <C as Collection<Idx>>::Output: Clone,
{
    fn dup(&mut self) -> bool
    {
        let Some(value) = Last::last(self).map(|value| value.deref().clone()) else
        {
            return false;
        };
        Push::push(self, value);
        true
    }
}


/// Duplicates the first value and push it..
pub trait DupFirst<Idx>: Collection<Idx>
{
    /// Duplicates the first value and push it if possible.
    fn dup_first(&mut self) -> bool;
}
impl<Idx, C> DupFirst<Idx> for C
where
    C: First<Idx> + Push<<C as Collection<Idx>>::Output>,
    <C as Collection<Idx>>::Output: Clone,
{
    fn dup_first(&mut self) -> bool
    {
        let Some(value) = First::first(self).map(|value| value.deref().clone()) else
        {
            return false;
        };
        Push::push(self, value);
        true
    }
}




/// Duplicates the last value and push_front it.
pub trait DupBack<Idx>: Collection<Idx>
{
    /// Duplicates the last value and push_front it if possible.
    fn dup_front(&mut self) -> bool;
}

impl<Idx, C> DupBack<Idx> for C
where
    C: Last<Idx> + PushFront<<C as Collection<Idx>>::Output>,
    <C as Collection<Idx>>::Output: Clone,
{
    fn dup_front(&mut self) -> bool
    {
        let Some(value) = Last::last(self).map(|value| value.deref().clone()) else
        {
            return false;
        };
        PushFront::push_front(self, value);
        true
    }
}


/// Duplicates the first value and push_front it.
pub trait DupFirstBack<Idx>: Collection<Idx>
{
    /// Duplicates the first value and push_front it if possible.
    fn dup_front(&mut self) -> bool;
}

impl<Idx, C> DupFirstBack<Idx> for C
where
    C: First<Idx> + PushFront<<C as Collection<Idx>>::Output>,
    <C as Collection<Idx>>::Output: Clone,
{
    fn dup_front(&mut self) -> bool
    {
        let Some(value) = First::first(self).map(|value| value.deref().clone()) else
        {
            return false;
        };
        PushFront::push_front(self, value);
        true
    }
}
