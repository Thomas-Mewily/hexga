use super::*;

pub mod prelude
{
    pub use super::{Table,TableID};
}

pub type Table<V> = GenMultiMap<String,V>;
pub type TableID<V> = gen_multi_map::GenMultiMapIDOf<String,V,Generation>;