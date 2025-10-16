use crate::multi_map::MultiHashMapOf;
use super::*;

pub mod prelude
{
    pub use super::{Table,TableID,CollectToTable};
}

pub type Table<V> = TableOf<V>;
pub type TableOf<V,Gen=Generation,S=RandomState> = MultiHashMapOf<String,V,Gen,S>;

pub type TableID<V> = TableIDOf<V,Generation>;
pub type TableIDOf<V,Gen=Generation,S=RandomState> = multi_map::MultiHashMapIDOf<String,V,Gen,S>;

pub trait CollectToTable<Keys,V> : CollectToMultiHashMap<Keys, String, V> where Keys: IntoIterator<Item = String>
{
    fn to_table(self) -> Table<V>
    {
        MultiHashMapOf::from_iter(self)
    }
}
impl<Keys,V,I> CollectToTable<Keys,V> for I where I : CollectToMultiHashMap<Keys, String, V>, Keys: IntoIterator<Item = String> {}