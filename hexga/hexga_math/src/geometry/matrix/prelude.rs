use crate::*;

pub use super::{Matrix,SquareMatrix};

/// 1x1 matrix
pub type Matrix1<T> = SquareMatrix<T,1>;
/// 2x2 matrix
pub type Matrix2<T> = SquareMatrix<T,2>;
/// 3x3 matrix
pub type Matrix3<T> = SquareMatrix<T,3>;
/// 4x4 matrix
pub type Matrix4<T> = SquareMatrix<T,4>;

pub type Mat<const ROW : usize, const COL : usize> = Matrix<float,ROW,COL>;
/// 1x1 matrix
pub type Mat1 = Mat<1,1>;
/// 2x2 matrix
pub type Mat2 = Mat<2,2>;
/// 3x3 matrix
pub type Mat3 = Mat<3,3>;
/// 4x4 matrix
pub type Mat4 = Mat<4,4>;

pub type MatP<const COL : usize> = SquareMatrix<int,COL>;
/// 1x1 matrix
pub type Mat1P = MatP<1>;
/// 2x2 matrix
pub type Mat2P = MatP<2>;
/// 3x3 matrix
pub type Mat3P = MatP<3>;
/// 4x4 matrix
pub type Mat4P = MatP<4>;

