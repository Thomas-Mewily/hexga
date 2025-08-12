use crate::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization

use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::marker::PhantomData;
use std::fmt::{self, Formatter};

struct VecWithSizeHint<T> {
    len: usize,
    _marker: PhantomData<T>,
}

impl<'de, T> DeserializeSeed<'de> for VecWithSizeHint<T>
where
    T: serde::Deserialize<'de>,
{
    type Value = Vec<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HintVisitor<T> {
            len: usize,
            _marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for HintVisitor<T>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a sequence with {} pixels", self.len)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::with_capacity(self.len);
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }
                Ok(vec)
            }
        }

        deserializer.deserialize_seq(HintVisitor {
            len: self.len,
            _marker: PhantomData,
        })
    }
}


impl<'de, T, Idx> Deserialize<'de> for ImageBase<T, Idx>
    where
        Idx: Integer + Deserialize<'de>,
        T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ImageVisitor<T, Idx> {
            marker: std::marker::PhantomData<(T, Idx)>,
        }

        impl<'de, T, Idx> Visitor<'de> for ImageVisitor<T, Idx>
        where
            Idx: Integer + Deserialize<'de>,
            T: Deserialize<'de>,
        {
            type Value = ImageBase<T,Idx>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("An Image with an `size` and `pixels`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut size: Option<Vector2<Idx>> = None;
                let mut pixels: Option<Vec<T>> = None;

                while let Some(key) = map.next_key()?
                {
                    match key
                    {
                        "size" => {
                            if size.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size = Some(map.next_value()?);
                        }
                        "pixels" => {
                            if pixels.is_some() {
                                return Err(serde::de::Error::duplicate_field("pixels"));
                            }
                            if let Some(ref sz) = size
                            {
                                if sz.area_usize_checked().is_none()
                                {
                                    return Err(serde::de::Error::custom(ImageBaseError::<Idx>::ToBig(*sz).to_debug()));
                                }
                                let seed = VecWithSizeHint::<T> {
                                    len: sz.area_usize(),
                                    _marker: PhantomData,
                                };
                                pixels = Some(map.next_value_seed(seed)?);
                            } else {
                                pixels = Some(map.next_value()?);
                            }
                        }
                        _ => { let _ = map.next_value::<serde::de::IgnoredAny>()?; }
                    }
                }

                let size = size.ok_or_else(|| serde::de::Error::missing_field("size"))?;
                let pixels = pixels.ok_or_else(|| serde::de::Error::missing_field("pixels"))?;

                match ImageBase::try_from_vec(size, pixels)
                {
                    Ok(g) => Ok(g),
                    Err(e) => Err(serde::de::Error::custom(e.to_debug()))
                }
            }
        }

        deserializer.deserialize_struct("Image", &["size", "pixels"], ImageVisitor {
            marker: std::marker::PhantomData,
        })
    }
}