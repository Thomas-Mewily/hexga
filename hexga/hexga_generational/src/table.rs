use crate::multi_map::MultiMapOf;
use super::*;

pub mod prelude
{
    pub use super::{Table,TableID,CollectToTable};
}

pub type Table<V> = TableOf<V>;
pub type TableOf<V,Gen=Generation,S=RandomState> = MultiMapOf<String,V,Gen,S>;

pub type TableID<V> = TableIDOf<V,Generation>;
pub type TableIDOf<V,Gen=Generation,S=RandomState> = multi_map::MultiMapIDOf<String,V,Gen,S>;

pub trait CollectToTable<Keys,V> : CollectToMultiMap<Keys, String, V> where Keys: IntoIterator<Item = String>
{
    fn to_table(self) -> Table<V>
    {
        MultiMapOf::from_iter(self)
    }
}
impl<Keys,V,I> CollectToTable<Keys,V> for I where I : CollectToMultiMap<Keys, String, V>, Keys: IntoIterator<Item = String> {}