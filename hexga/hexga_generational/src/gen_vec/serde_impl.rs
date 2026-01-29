use super::*;

use serde::de::{MapAccess, SeqAccess};


impl<T> serde::Serialize for EntryValue<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EntryValue::Occupied(value) => serializer.serialize_newtype_variant("EntryValue", 0, "Used", value),
            EntryValue::Vacant(idx) => {
                let opt = if idx.is_max() { None } else { Some(*idx) };
                serializer.serialize_newtype_variant("EntryValue", 1, "Next", &opt)
            }
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for EntryValue<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier)]
        enum Field { Used, Next }

        struct EntryValueVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for EntryValueVisitor<T>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = EntryValue<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("enum EntryValue")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                use serde::de::VariantAccess;

                match data.variant()? {
                    (Field::Used, v) => Ok(EntryValue::Occupied(v.newtype_variant()?)),
                    (Field::Next, v) => {
                        let opt: Option<usize> = v.newtype_variant()?;
                        Ok(EntryValue::Vacant(opt.unwrap_or(usize::MAX)))
                    }
                }
            }
        }

        deserializer.deserialize_enum(
            "EntryValue",
            &["Used", "Next"],
            EntryValueVisitor { marker: std::marker::PhantomData },
        )
    }
}



impl<T, C, Gen> Serialize for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View<'a> = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    C: Serialize, T: Serialize, Gen: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        let mut state = serializer.serialize_struct("GenVec", 2)?;
        state.serialize_field("values", &self.values)?;
        // need to be in the same order on all machine for determinist
        state.serialize_field("next", &Some(self.free))?;
        state.end()

        /*
        // Can't use it, because some deserializer that don't have a self-describing format (like bincode) can't deserialize_any().
        // The layout must be fixed
        if self.free.is_max()
        {
            self.values.serialize(serializer)
        }else
        {
            let mut state = serializer.serialize_struct("GenVec", 2)?;
            state.serialize_field("values", &self.values)?;
            // need to be in the same order on all machine for determinist
            state.serialize_field("next", &Some(self.free))?;
            state.end()
        }
        */
    }
}



impl<'de, T, C, Gen> Deserialize<'de> for GenVecOf<T,Gen,C>
    where
    C: for<'a> View<'a,View<'a> = &'a [Entry<T,Gen>]>, Gen:IGeneration,
    C: Deserialize<'de>, T: Deserialize<'de>, Gen: IGeneration + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct GenVec<T,Gen,C>
            where
            C: for<'a> View<'a,View<'a> = &'a [Entry<T,Gen>]>, Gen:IGeneration,
        {
            values: C,
            next: Option<usize>,
        }

        let GenVec{ values, next } = GenVec::deserialize(deserializer)?;
        GenVecOf::<T,Gen,C>::try_from_raw_parts(values, next.unwrap_or(usize::MAX)).map_err(serde::de::Error::custom)
    }
}
