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
            EntryValue::Some(value) => serializer.serialize_newtype_variant("EntryValue", 0, "Some", value),
            EntryValue::Free(idx) => {
                let opt = if idx.is_max_value() { None } else { Some(*idx) };
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
        enum Field { Some, Next }

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
                    (Field::Some, v) => Ok(EntryValue::Some(v.newtype_variant()?)),
                    (Field::Next, v) => {
                        let opt: Option<usize> = v.newtype_variant()?;
                        Ok(EntryValue::Free(opt.unwrap_or(usize::MAX)))
                    }
                }
            }
        }

        deserializer.deserialize_enum(
            "EntryValue",
            &["Some", "Next"],
            EntryValueVisitor { marker: std::marker::PhantomData },
        )
    }
}



impl<T, Gen:IGeneration> Serialize for GenVecOf<T,Gen> where T:Serialize, Gen: Serialize
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
        if self.free.is_max_value()
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



impl<'de, T, Gen> Deserialize<'de> for GenVecOf<T,Gen>
    where
    T: Deserialize<'de>,
    Gen: IGeneration + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct GenVec<T,Gen:IGeneration=Generation>
        {
            values: Vec<Entry<T,Gen>>,
            next: Option<usize>,
        }

        let GenVec{ values, next } = GenVec::deserialize(deserializer)?;
        GenVecOf::<T,Gen>::from_entries_and_free(values, next.unwrap_or(usize::MAX)).map_err(serde::de::Error::custom)
    }
}
