```rs

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
```