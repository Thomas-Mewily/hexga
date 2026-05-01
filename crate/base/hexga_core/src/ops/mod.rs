use super::*;

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

/*
mod insert_pop;
pub use insert_pop::*;
*/

/*
mod sequence;
pub use sequence::*;
*/

/*
mod view;
pub use view::*;

mod view_mut;
pub use view_mut::*;
*/

pub mod prelude
{
    pub use super::{
        capacity::*, clear::*, get::*, index::*, insert::*, length::*, pop::*, push::*,
        remove::*,
    };
}
