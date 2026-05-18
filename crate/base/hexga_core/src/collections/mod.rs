use super::*;

pub mod singly_linked;
pub use singly_linked::prelude::*;

re_export_item_from_std_or_alloc!(collections);

/*
#[cfg(feature = "std")]
mod default_associatif
{
    use super::*;

    pub type Map<K,V> = HashMap<K,V>;
    pub type Set<T> = HashSet<T>;
}
#[cfg(not(feature = "std"))]
mod default_associatif
{
    use super::*;

    pub type Map<K,V> = alloc::collections::btree::map::BTreeMap<K,V>;
    pub type Set<T> = alloc::collections::btree::map::BTreeSet<T>;
}
pub use default_associatif::*;
*/

use core::ops::Range;

mod length;
pub use length::*;

mod get;
pub use get::*;

mod index;
pub use index::*;

mod capacity;
pub use capacity::*;

mod clear;
pub use clear::*;

mod remove;
pub use remove::*;

mod push;
pub use push::*;

mod insert;
pub use insert::*;

mod pop;
pub use pop::*;

mod matches;
pub use matches::*;

/*
mod insert_pop;
pub use insert_pop::*;
*/

/*
mod sequence;
pub use sequence::*;
*/

/*
// Generic view trait that also work for grid (because slice type some pointer metadata hack in deref)
mod view;
pub use view::*;

mod view_mut;
pub use view_mut::*;
*/

mod collection;
pub use collection::*;

pub mod prelude
{
    #[allow(unused)]
    pub(crate) use super::collections::{BinaryHeap, LinkedList};

    pub use super::collections::{BTreeMap, BTreeSet, VecDeque};
    pub use crate::{string::String, vec::Vec};
    //pub use vec::Vec;

    #[cfg(feature = "std")]
    pub use super::collections::{HashMap, HashSet};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{
        capacity::*, clear::*, collection::*, get::*, index::*, insert::*, length::*, matches::*,
        pop::*, push::*, remove::*,
    };
}
