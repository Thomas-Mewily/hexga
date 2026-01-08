use super::*;

/// Can be used to define default value,
/// and different deserializalzion based on a constant for non_exhaustive/missing field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithDefault<T,Policy>
    where Policy: Constant<T>
{
    pub value: T,
    phantom: PhantomData<Policy>,
}
impl<T,Policy> From<T> for WithDefault<T,Policy>
    where Policy: Constant<T>
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T,Policy> WithDefault<T,Policy>
    where Policy: Constant<T>
{
    pub const fn new(value: T) -> Self { Self { value, phantom: PhantomData }}
    pub fn into_value(self) -> T { self.value }
}

impl<T,Policy> Default for WithDefault<T,Policy>
    where Policy: Constant<T>
{
    fn default() -> Self {
        Self::new(Policy::CONSTANT)
    }
}

map_on_number_and_bool_and_char!(
    ($type_name: tt) =>
    {
        #[cfg(feature = "serde")]
        impl<Policy> $crate::serde::Serialize for WithDefault<$type_name, Policy>
        where
            $type_name: $crate::serde::Serialize,
            Policy: Constant<$type_name>,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                self.value.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, Policy> $crate::serde::Deserialize<'de> for WithDefault<$type_name, Policy>
        where
            $type_name: $crate::serde::Deserialize<'de>,
            Policy: Constant<$type_name>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                let value = $type_name::deserialize(deserializer).unwrap_or(<Policy as Constant<$type_name>>::CONSTANT);
                Ok(Self::new(value))
            }
        }
    }
);

/*
map_on_number_and_bool!(
    ($type_name: tt) =>
    {
        map_on_constant_policy!(
            ($policy: tt) =>
            {
                impl ::std::default::Default for WithDefault<$type_name, $policy <$type_name>>
                    where $policy <$type_name>: $crate::number::Constant<$type_name>
                {
                    fn default() -> Self
                    {
                        Self::new($policy :: <$type_name>::CONSTANT)
                    }
                }
            }
        );
    }
);
*/