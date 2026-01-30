pub use std::iter::{Product, Sum};

pub trait IterExtension<'a, Item>
where
    Self: 'a,
    &'a Self: IntoIterator<Item = Item>,
{
    fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter { self.into_iter() }
}
impl<'a, Item, T> IterExtension<'a, Item> for T where &'a T: IntoIterator<Item = Item> + 'a {}

pub trait IterPredicate<Item>: IntoIterator<Item = Item> + Sized
{
    #[inline(always)]
    fn any<P>(self, p: P) -> bool
    where
        P: FnMut(Item) -> bool,
    {
        self.into_iter().any(p)
    }
    #[inline(always)]
    fn all<P>(self, p: P) -> bool
    where
        P: FnMut(Item) -> bool,
    {
        self.into_iter().all(p)
    }
    //#[inline(always)]
    //fn for_each<F>(self, f: F) where F: FnMut(Item) { self.into_iter().for_each(f); }

    #[inline(always)]
    fn any_with<P, O>(self, other: O, mut p: P) -> bool
    where
        P: FnMut(Item, Item) -> bool,
        O: IntoIterator<Item = Item>,
    {
        let it_a = self.into_iter();
        let it_b = other.into_iter();
        it_a.zip(it_b).any(|v| p(v.0, v.1))
    }

    #[inline(always)]
    fn all_with<P, O>(self, other: O, mut p: P) -> bool
    where
        P: FnMut(Item, Item) -> bool,
        O: IntoIterator<Item = Item>,
    {
        let it_a = self.into_iter();
        let it_b = other.into_iter();
        it_a.zip(it_b).all(|v| p(v.0, v.1))
    }
}
impl<Item, T> IterPredicate<Item> for T where T: IntoIterator<Item = Item> + Sized {}

pub trait IterMutExtension<'a, Item>
where
    Self: 'a + IterExtension<'a, Item>,
    &'a Self: IntoIterator<Item = Item>,
    &'a mut Self: IntoIterator<Item = Item>,
{
    fn iter_mut(&'a mut self) -> <&'a mut Self as IntoIterator>::IntoIter { self.into_iter() }

    //fn for_each_mut<F>(&'a mut self, f: F) where F: FnMut(Item) { self.iter_mut().for_each(f); }
}
impl<'a, Item, T> IterMutExtension<'a, Item> for T
where
    &'a mut T: IntoIterator<Item = Item> + 'a,
    &'a Self: IntoIterator<Item = Item>,
{
}
