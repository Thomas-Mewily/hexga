pub use std::iter::{Sum,Product};

pub trait IterExtension<'a, Item>: where Self: 'a, &'a Self: IntoIterator<Item = Item>
{
    fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
impl<'a,Item,T> IterExtension<'a,Item> for T where &'a T: IntoIterator<Item = Item> + 'a {}

pub trait IterMutExtension<'a, Item>: where Self: 'a, &'a mut Self: IntoIterator<Item = Item>
{
    fn iter_mut(&'a mut self) -> <&'a mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
impl<'a,Item,T> IterMutExtension<'a,Item> for T where &'a mut T: IntoIterator<Item = Item> + 'a {}