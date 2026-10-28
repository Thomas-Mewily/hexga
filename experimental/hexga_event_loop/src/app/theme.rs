use super::*;

/// The theme variant to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AppTheme {
    /// Use the light variant.
    Light,

    /// Use the dark variant.
    Dark,
}