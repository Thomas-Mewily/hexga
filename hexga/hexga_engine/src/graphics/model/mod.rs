use super::*;

mod vertex;
pub use vertex::*;

pub(crate) mod prelude
{
    pub use super::vertex::{UV,Vertex,VertexIndex};
}