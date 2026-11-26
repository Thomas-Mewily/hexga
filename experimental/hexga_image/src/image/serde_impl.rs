use super::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization

impl<C, Idx, A: Allocator + Clone> Serialize for ImageBaseOf<C, Idx, A>
where
    Idx: Integer + Serialize,
    Self: ToUrl,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_with_encoding(self)
    }
}

impl<'de, C, Idx, A: Allocator + Clone + Default> Deserialize<'de> for ImageBaseOf<C, Idx, A>
where
    Idx: Integer + for<'de2> Deserialize<'de2>,
    Self: FromUrl,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_with_encoding::<Self>()
    }
}
