use crate::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization

#[cfg(feature = "serde")]
impl<'de, T, Idx, const N : usize> Deserialize<'de> for GridBase<T, Idx, N>
    where
        Idx: Integer,
        Vector<Idx,N>: Deserialize<'de>,
        T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GridVisitor<T, Idx, const N : usize> {
            marker: std::marker::PhantomData<(T, Idx)>,
        }

        impl<'de, T, Idx, const N : usize> Visitor<'de> for GridVisitor<T, Idx, N>
        where
            Idx: Integer,
            Vector<Idx,N>: Deserialize<'de>,
            T: Deserialize<'de>,
        {
            type Value = GridBase<T,Idx,N>;

            fn expecting(&self, formatter: &mut Formatter) -> DResult {
                formatter.write_str("A Grid with an `size` and `values`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut size: Option<Vector<Idx,N>> = None;
                let mut values: Option<Vec<T>> = None;

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
                        "values" => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(map.next_value()?);
                        }
                        _ => { let _ = map.next_value::<serde::de::IgnoredAny>()?; }
                    }
                }

                let size = size.ok_or_else(|| serde::de::Error::missing_field("size"))?;
                let values = values.ok_or_else(|| serde::de::Error::missing_field("values"))?;

                match GridBase::try_from_vec(size, values)
                {
                    Ok(g) => Ok(g),
                    Err(e) => Err(serde::de::Error::custom(
                        match e
                        {
                            GridBaseError::NegativeSize(vector) => format!("Area component of the grid can't be negative : {:?}", vector),
                            GridBaseError::WrongDimension(vector, got) => format!("The area of the grid ({:?} => {} values) does not match the number of values ({})", vector, vector.area_usize(), got),
                        }
                    ))
                }
            }
        }

        deserializer.deserialize_struct("Grid", &["size", "values"], GridVisitor {
            marker: std::marker::PhantomData,
        })
    }
}