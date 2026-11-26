use super::*;

mod mesh_builder;
pub use mesh_builder::*;

mod mesh_geometry;
pub use mesh_geometry::*;

mod gmesh;
pub use gmesh::*;

mod mesh;
pub use mesh::*;

pub mod prelude {
    pub use super::gmesh::GMesh;
    pub use super::mesh::Mesh;
    pub use super::mesh_builder::MeshBuilder;
    pub use super::traits::*;
}

pub mod traits {
    pub use super::mesh_builder::BuilderMesh;
}
