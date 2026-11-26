use super::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization

use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

impl<T, Idx, const N: usize, A: Allocator> Serialize for GridOf<T, Idx, N, A>
where
    Idx: Integer + Serialize,
    T: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut grid = serializer.serialize_struct("Grid", 2)?;
        grid.serialize_field("size", &self.size)?;
        grid.serialize_field("values", self.values.as_slice())?;
        grid.end()
    }
}

impl<'de, T, Idx, const N: usize, A: Allocator + Clone + Default> Deserialize<'de> for GridOf<T, Idx, N, A>
where
    Idx: Integer + Deserialize<'de>,
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Grid<T, Idx, const N: usize>
        where
            Idx: Integer,
        {
            // TODO : Can use the `size` value in json/ron to pre allocate the right among of values in the vector
            // The size is will be deserialized first before the values,
            // then we can give a hint to serde about the size of the vector to alloc when deserializing
            size: Vector<Idx, N>,
            values: Vec<T>,
        }

        let Grid { size, values } = Grid::deserialize(deserializer)?;
        let mut storage = Vec::with_capacity_in(values.len(), A::default());
        storage.extend(values);
        Self::try_from_vec(size, storage).map_err(serde::de::Error::custom)
    }
}
