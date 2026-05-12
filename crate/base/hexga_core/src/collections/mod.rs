#[allow(unused_imports)]
use super::*;

pub mod singly_linked;
pub use singly_linked::prelude::*;

re_export_item_from_std_or_alloc!(collections);

pub mod prelude
{
    #[allow(unused)]
    pub(crate) use super::collections::{BinaryHeap, LinkedList};

    pub use super::collections::{BTreeMap, BTreeSet, VecDeque};
    //pub use vec::Vec;

    #[cfg(feature = "std")]
    pub use super::collections::{HashMap, HashSet};
}
