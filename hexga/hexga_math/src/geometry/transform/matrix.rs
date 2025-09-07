use super::*;

pub mod prelude
{
    pub use super::{GetMatrix,SetMatrix};
}

pub trait GetMatrix<T,const ROW: usize, const COL: usize>
{
    fn matrix(&self) -> Matrix<T,ROW,COL>;
}

pub trait SetMatrix<T,const ROW: usize, const COL: usize> : GetMatrix<T,ROW,COL>
{
    fn set_matrix(&mut self, matrix : Matrix<T,ROW,COL>) -> &mut Self;
}
