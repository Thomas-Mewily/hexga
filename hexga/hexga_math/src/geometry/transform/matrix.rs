use super::*;

pub trait GetMatrix<T = float, const ROW: usize = 4, const COL: usize = 4>
{
    fn matrix(&self) -> Matrix<T, ROW, COL>;
}

pub trait SetMatrix<T = float, const ROW: usize = 4, const COL: usize = 4>:
    GetMatrix<T, ROW, COL>
{
    fn set_matrix(&mut self, matrix: Matrix<T, ROW, COL>) -> &mut Self;
}

pub trait GetMatrixMulExtension<T = float, const ROW: usize = 4, const COL: usize = 4>:
    GetMatrix<T, ROW, COL> + SetMatrix<T, ROW, COL>
{
    fn matrix_mul<O>(&mut self, lhs: O) -> &mut Self
    where
        Matrix<T, ROW, COL>: Mul<O, Output = Matrix<T, ROW, COL>>,
    {
        let m = self.matrix();
        self.set_matrix(m * lhs)
    }

    fn matrix_left_mul<O>(&mut self, rhs: O) -> &mut Self
    where
        O: Mul<Matrix<T, ROW, COL>, Output = Matrix<T, ROW, COL>>,
    {
        let m = self.matrix();
        self.set_matrix(rhs * m)
    }
}
impl<T, const ROW: usize, const COL: usize, S> GetMatrixMulExtension<T, ROW, COL> for S where
    S: GetMatrix<T, ROW, COL> + SetMatrix<T, ROW, COL>
{
}
