use hexga_core::prelude::*;
use hexga_number::prelude::*;

pub mod prelude
{
    //pub use super::traits::*;
    pub use super::{BitSet, Bit, BitIndex};
}
pub mod traits
{

}

pub type BitIndex = u8;
pub type Bit = bool;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BitSet<T>(pub T);

impl<T> Collection<BitIndex> for BitSet<T> where T: Integer
{
    type Output = Bit;
    type Guard<'a> = Identity<Bit> where Self: 'a;
}
/*
impl<T> Get<BitIndex> for BitSet<T> where T: Integer 
{
    fn get<'a>(&'a self, index: BitIndex) -> Option<Self::Guard<'a>> where Self::Output: 'a {
        todo!()
    }
}
*/