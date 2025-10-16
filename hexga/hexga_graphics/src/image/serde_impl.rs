use super::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization

use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::marker::PhantomData;
use std::fmt::{self, Formatter};


impl<'de, T, Idx> Deserialize<'de> for ImageBaseOf<T, Idx>
    where
        Idx: Integer + Deserialize<'de>,
        T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Image<C,Idx> where Idx : Integer
        {
            // Todo : Can use the size value is json/ron to pre allocate the right among of values in the vector
            // The size is will be deserialized first before the values,
            // then we can give a hint to serde about the size of the vector to alloc when deserializing
            size   : Vector2<Idx>,
            pixels : Vec<C>,
        }

        let Image { size, pixels } = Image::deserialize(deserializer)?;
        ImageBaseOf::try_from_vec(size, pixels).map_err(serde::de::Error::custom)
    }
}