use super::*;

mod length;
pub use length::*;

mod get;
pub use get::*;

mod capacity;
pub use capacity::*;

mod clearable;
pub use clearable::*;

mod sequence;
pub use sequence::*;

mod collect_to;
pub use collect_to::*;


pub mod prelude
{
    pub use crate::collections::{Length,Clearable,Capacity,TryGet,Get,TryGetMut,GetMut,GetManyMut,ManyMutError};
    pub use crate::collections::{IndexOutOfRange,MissingKey};
    pub use crate::collections::collect_to::prelude::*;
}