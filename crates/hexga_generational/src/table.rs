use crate::multi_map::MultiHashMapOf;
use super::*;

pub mod prelude
{
    pub use super::{Table,TableID,CollectToTable};
}

pub type Table<V> = TableOf<V>;
pub type TableOf<V,Gen=Generation,S=RandomState> = MultiHashMapOf<String,V,Gen,S>;

pub type TableID = TableIDOf;
pub type TableIDOf<Gen=Generation> = multi_map::MultiHashMapIDOf<Gen>;

pub type Iter<'a,V,Gen=Generation,S=RandomState> = multi_map::Iter<'a,String,V,Gen,S>;
pub type IterMut<'a,V,Gen=Generation,S=RandomState> = multi_map::IterMut<'a,String,V,Gen,S>;
pub type IntoIter<V,Gen=Generation,S=RandomState> = multi_map::IntoIter<String,V,Gen,S>;


pub trait CollectToTable<Keys,V> : CollectToMultiHashMap<Keys, String, V> where Keys: IntoIterator<Item = String>
{
    fn to_table(self) -> Table<V>
    {
        MultiHashMapOf::from_iter(self)
    }
}
impl<Keys,V,I> CollectToTable<Keys,V> for I where I : CollectToMultiHashMap<Keys, String, V>, Keys: IntoIterator<Item = String> {}