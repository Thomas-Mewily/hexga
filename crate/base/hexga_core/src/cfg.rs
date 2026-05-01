#[allow(unused_imports)]
use super::*;

#[cfg(not(feature = "serde"))]
pub trait CfgSerialize {}
#[cfg(not(feature = "serde"))]
impl<T> CfgSerialize for T {}

#[cfg(feature = "serde")]
pub trait CfgSerialize: Serialize {}
#[cfg(feature = "serde")]
impl<T> CfgSerialize for T where T: Serialize + ?Sized {}

#[cfg(not(feature = "serde"))]
pub trait CfgDeserialize<'de> {}
#[cfg(not(feature = "serde"))]
impl<'de, T> CfgDeserialize<'de> for T {}

#[cfg(feature = "serde")]
pub trait CfgDeserialize<'de>: Deserialize<'de> {}
#[cfg(feature = "serde")]
impl<'de, T> CfgDeserialize<'de> for T where T: Deserialize<'de> + ?Sized {}
