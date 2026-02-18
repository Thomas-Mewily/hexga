#[allow(unused_imports)]
use super::*;

pub mod singly_linked;
pub use singly_linked::prelude::*;

#[allow(unused_imports)]
#[cfg(feature = "std")]
pub use std::collections::*;

#[cfg(not(feature = "std"))]
pub use ::alloc::collections::*;

pub mod prelude
{
    #[cfg(feature = "std")]
    pub use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque};
    #[allow(unused_imports)]
    #[cfg(feature = "std")]
    pub(crate) use std::collections::LinkedList;

    #[cfg(not(feature = "std"))]
    pub use ::alloc::collections::{BTreeMap, BTreeSet, VecDeque};
    #[cfg(not(feature = "std"))]
    pub use ::alloc::{boxed::Box, string::String, vec::Vec};
}