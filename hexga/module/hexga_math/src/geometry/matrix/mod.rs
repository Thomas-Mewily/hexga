use crate::*;

pub mod prelude;
use prelude::*;

/// NxN matrix
pub type SquareMatrix<T, const N : usize> = Matrix<T, N, N>;

/// ROW rows, COL columns.
///
/// Can be indexed `matrix[row][col]`
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Matrix<T, const ROW : usize, const COL : usize>
{
    pub columns : Vector<Vector<T, ROW>,COL>,
}

//#[cfg(feature = "hexga_io")]


impl<T, const ROW : usize, const COL : usize> Deref for Matrix<T, ROW, COL>
{
    type Target=Vector<Vector<T, ROW>,COL>;
    fn deref(&self) -> &Self::Target { &self.columns }
}

impl<T, const ROW : usize, const COL : usize> DerefMut for Matrix<T, ROW, COL>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.columns }
}

impl<T, const ROW : usize, const COL : usize>  Matrix<T, ROW, COL>
{
    fn _fmt(&self, f: &mut Formatter<'_>, d : impl Fn(&T, &mut Formatter<'_>) -> DResult) -> DResult
    {
        for c in 0..COL
        {
            for r in 0..ROW
            {
                d(&self[c][r], f)?;
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

use std::fmt::*;
impl<T, const ROW : usize, const COL : usize> Display  for Matrix<T, ROW, COL> where T : Display  { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> Debug    for Matrix<T, ROW, COL> where T : Debug    { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> Octal    for Matrix<T, ROW, COL> where T : Octal    { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> Binary   for Matrix<T, ROW, COL> where T : Binary   { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> LowerHex for Matrix<T, ROW, COL> where T : LowerHex { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> UpperHex for Matrix<T, ROW, COL> where T : UpperHex { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> LowerExp for Matrix<T, ROW, COL> where T : LowerExp { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> UpperExp for Matrix<T, ROW, COL> where T : UpperExp { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> Pointer  for Matrix<T, ROW, COL> where T : Pointer  { fn fmt(&self, f: &mut Formatter<'_>) -> DResult  { self._fmt(f, T::fmt) } }

impl<T, const ROW : usize, const COL : usize> Default  for Matrix<T,ROW,COL>   where Vector<Vector<T, ROW>,COL> : Default
{
    fn default() -> Self {
        Self { columns: ___() }
    }
}

impl<T, const ROW : usize> Matrix<T,ROW,1>
{
    /// From a `Mx1` vector
    pub const fn from_vector(vector : Vector<T, ROW>) -> Self
    {
        Self::from_col(Vector::from_array([vector]))
    }
}

impl<T, const ROW : usize> From<Vector<T, ROW>> for Matrix<T,ROW,1>
{
    fn from(value: Vector<T, ROW>) -> Self {
        Self::from_vector(value)
    }
}
impl<T, const ROW : usize> From<Matrix<T,ROW,1>> for Vector<T, ROW> where Vector<T,ROW> : Default
{
    fn from(mut value: Matrix<T,ROW,1>) -> Self
    {
        std::mem::take(&mut value.columns[0])
    }
}

impl<T, const ROW : usize, const COL : usize> Matrix<T,ROW,COL>
{
    /// Create a matrix from a `fn((columns,row))`
    pub fn from_fn<F>(mut column_then_row : F) -> Self where F : FnMut(Point2) -> T
    {
        Self::from_col_array(std::array::from_fn(|column| std::array::from_fn(|row| column_then_row(point2(column as _,row as _)))))
    }

    pub fn from_col_fn<F>(mut column_then_row : F) -> Self where F : FnMut(usize, usize) -> T
    {
        Self::from_col_array(std::array::from_fn(|column| std::array::from_fn(|row| column_then_row(column,row))))
    }

    pub fn from_row_fn<F>(mut row_then_column : F) -> Self where F : FnMut(usize, usize) -> T
    {
        Self::from_col_array(std::array::from_fn(|column| std::array::from_fn(|row| row_then_column(row,column))))
    }

    pub const fn from_col(columns : Vector<Vector<T, ROW>,COL>) -> Self { Self { columns }}
    pub fn from_col_array(columns : [[T;ROW];COL]) -> Self
    {
        Self
        {
            columns :  Vector::from_array(columns.map(|array| Vector::from_array(array)))
        }
    }

    pub fn from_row(rows : Vector<Vector<T, COL>,ROW>) -> Self where T : Copy
    {
        Matrix::from_col(rows).transpose()
    }
    pub fn from_row_array(rows : [[T;COL];ROW]) -> Self where T : Copy
    {
        Self::from_row(Vector::from_array(rows.map(|array| Vector::from_array(array))))
    }

    pub fn col(&self) -> Vector<Vector<T, ROW>,COL> where T : Copy { self.columns }
    pub fn row(&self) -> Vector<Vector<T, COL>,ROW> where T : Copy { self.transpose().columns }

    pub fn iter_col(&self) -> impl Iterator<Item = Vector<T, ROW>> where Vector<Vector<T, ROW>,COL> : Copy, T : Copy { self.columns.into_iter()}
    pub fn iter_row(&self) -> impl Iterator<Item = Vector<T, COL>> where Vector<Vector<T, ROW>,COL> : Copy, T : Copy { self.transpose().into_iter()}

    /// Transpose the matrix
    ///
    /// ```rust
    /// use hexga_math::prelude::*;
    ///
    /// assert_eq!(
    ///     Mat2::from_col(vector2(vec2(1., 2.), vec2(3., 4.))).transpose(),
    ///     Mat2::from_col(vector2(vec2(1., 3.), vec2(2., 4.)))
    /// );
    ///
    /// assert_eq!(
    ///     Mat::<3,2>::from_col(vector2(vec3(1., 2., 3.), vec3(4., 5., 6.))).transpose(),
    ///     Mat::<2,3>::from_col(vector3(vec2(1., 4.), vec2(2., 5.), vec2(3., 6.)))
    /// );
    /// ```
    pub fn transpose(&self) -> Matrix<T,COL,ROW> where T : Copy
    {
        Matrix::from_col(Vector::from_fn(|x| Vector::from_fn(|y| self[y][x])))
    }
}

// 2D
impl<T> SquareMatrix<T,2>
    where
    Self : Copy,
    SquareMatrix<T, 2> : Mul<Vector<T, 2>, Output = Vector<T, 2>>,
    Vector<T, 2> : Into< Vector<T, 1>>
{
    pub fn transform_relative(self, relative : Vector<T, 1>) -> Vector<T, 1> where T : Zero
    {
        self.transform(relative.with_y(T::ZERO)).into()
    }

    pub fn transform_position(self, position : Vector<T, 1>) -> Vector<T, 1> where T : One
    {
        self.transform(position.with_y(T::ONE)).into()
    }
}

// 3D
impl<T> SquareMatrix<T,3>
    where
    Self : Copy,
    SquareMatrix<T, 3> : Mul<Vector<T, 3>, Output = Vector<T, 3>>,
    Vector<T, 3> : Into< Vector<T, 2>>
{
    pub fn transform_relative(self, relative : Vector<T, 2>) -> Vector<T, 2> where T : Zero
    {
        self.transform(relative.with_z(T::ZERO)).into()
    }

    pub fn transform_position(self, position : Vector<T, 2>) -> Vector<T, 2> where T : One
    {
        self.transform(position.with_z(T::ONE)).into()
    }
}

// 4D
impl<T> SquareMatrix<T,4>
    where
    Self : Copy,
    SquareMatrix<T, 4> : Mul<Vector<T, 4>, Output = Vector<T, 4>>,
    Vector<T, 4> : Into< Vector<T, 3>>
{
    pub fn transform_relative(self, relative : Vector<T, 3>) -> Vector<T, 3> where T : Zero
    {
        self.transform(relative.with_w(T::ZERO)).into()
    }

    pub fn transform_position(self, position : Vector<T, 3>) -> Vector<T, 3> where T : One
    {
        self.transform(position.with_w(T::ONE)).into()
    }
}


impl<T, const N : usize> SquareMatrix<T,N>
    where
    Self : Copy,
    SquareMatrix<T, N>: Mul<Vector<T, N>, Output = Vector<T, N>>
{
    /*
    /// The last value/component will be replaced by 0 because it is relative/a vector in math terms.
    pub fn transform_relative(self, relative : Vector<T, N>) -> Vector<T, N> where T : Zero
    {
        self.transform(relative.with(N-1, T::ZERO))
    }

    /// The last value/component will be replaced by 1 because it is fixed/point in math terms.
    pub fn transform_fixed(self, fixed : Vector<T, N>) -> Vector<T, N> where T : Zero
    {
        self.transform(fixed.with(N-1, T::ZERO))
    }
    */
    /// The last value/component is 0 if it is a relative/vector/delta, 1 if it is fixed/point/position in math terms.
    pub fn transform(self, value : Vector<T, N>) -> Vector<T, N>
    {
        self * value
    }
}




impl<T, const ROW : usize, const COL : usize> Zero for Matrix<T,ROW,COL> where Vector<Vector<T,ROW>,COL> : Zero
{
    const ZERO : Self = Self::from_col(Vector::<Vector::<T, ROW>, COL>::ZERO);
}

impl<T, const N : usize> SquareMatrix<T,N> where T : One + Zero + Copy
{
    /// Same as [One]
    pub const IDENTITY : Self = Self::ONE;
}


impl<T, const N : usize> One for SquareMatrix<T,N> where T : One + Zero + Copy
{
    const ONE : Self =
    {
        let mut col = [[T::ZERO; N]; N];
        let mut i = 0;
        while i < N
        {
            col[i][i] = T::ONE;
            i+=1;
        }
        Self::from_col(unsafe { std::mem::transmute_copy(&col) })
    };
}

impl<T, const N : usize> MinusOne for SquareMatrix<T,N> where T : MinusOne + Zero + Copy
{
    const MINUS_ONE : Self =
    {
        let mut col = [[T::ZERO; N]; N];
        let mut i = 0;
        while i < N
        {
            col[i][i] = T::MINUS_ONE;
            i+=1;
        }
        Self::from_col(unsafe { std::mem::transmute_copy(&col) })
    };
}

impl<T, const N : usize> Half for SquareMatrix<T,N> where T : Half + Zero + Copy
{
    const HALF : Self =
    {
        let mut col = [[T::ZERO; N]; N];
        let mut i = 0;
        while i < N
        {
            col[i][i] = T::HALF;
            i+=1;
        }
        Self::from_col(unsafe { std::mem::transmute_copy(&col) })
    };
}

map_on!(
    (
        (MinValue, MIN),
        (MaxValue, MAX),
        (NaNValue, NAN)
    ),
    (($trait_name : ident, $constant_name : ident)) =>
    {
        impl<T, const ROW : usize, const COL : usize> $trait_name for Matrix<T,ROW,COL> where T : $trait_name + Copy
        {
            const $constant_name : Self = Self::from_col(Vector::from_array([Vector::from_array([T::$constant_name; ROW]); COL]));
        }
    }
);

impl<T, const ROW : usize, const COL : usize> Add<Self> for Matrix<T,ROW,COL>
    where
    Vector<Vector<T, ROW>, COL> : Add<Vector<Vector<T, ROW>, COL>,Output = Vector<Vector<T, ROW>, COL>>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        Self::from_col(self.columns.add(rhs.columns))
    }
}

impl<T, const ROW : usize, const COL : usize> AddAssign<Self> for Matrix<T,ROW,COL>
    where
    Self : Add<Self,Output = Self> + Copy
{
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self).add(rhs);
    }
}


impl<T, const ROW : usize, const COL : usize> Sub<Self> for Matrix<T,ROW,COL>
    where
    Vector<Vector<T, ROW>, COL> : Sub<Vector<Vector<T, ROW>, COL>,Output = Vector<Vector<T, ROW>, COL>>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        Self::from_col(self.columns.sub(rhs.columns))
    }
}

impl<T, const ROW : usize, const COL : usize> SubAssign<Self> for Matrix<T,ROW,COL>
    where
    Self : Sub<Self,Output = Self> + Copy
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self).sub(rhs);
    }
}


impl<T, const ROW : usize, const COL : usize> Sum for Matrix<T,ROW,COL> where Self : Zero + Add<Self,Output = Self>
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<T, const COL : usize> Product for SquareMatrix<T,COL> where Self : One + Mul<Self,Output = Self>
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Self::mul)
    }
}

/// ```
/// use hexga_math::prelude::*;
///
/// let m1  = Mat2P::from_col((point2(7, 6), point2(5, 3)).into());
/// let m2  = Mat2P::from_col((point2(2, 5), point2(1, 1)).into());
/// let m3  = Mat2P::from_col((point2(39, 27), point2(12, 9)).into());
///
/// assert_eq!(m1 * m2, m3);
///
///
/// let m1 = Matrix::<i32,2,3>::from_col(vector3(vector2(1, 4), vector2(2, 5), vector2(3, 6)));
/// let m2 = Matrix::<i32,3,2>::from_col(vector2(vector3(7, 9, 11), vector3(8, 10, 12)));
/// let m3 = Matrix::<i32,2,2>::from_col(vector2(vector2(1*7+2*9+3*11, 4*7+5*9+6*11), vector2(1*8+2*10+3*12, 4*8+5*10+6*12)));
///
/// assert_eq!(m1 * m2, m3);
/// ```
impl<T, const ROW : usize, const COL : usize, const COL2 : usize> Mul<Matrix<T,COL,COL2>> for Matrix<T,ROW,COL>
    where
    T : NumberArithmetic,
{
    type Output = Matrix<T, ROW, COL2>;

    fn mul(self, rhs: Matrix<T,COL,COL2>) -> Self::Output
    {
        // Todo : match on ROW / COL, COL2
        // and specialize it for 1x1, 2x2, 3x3 and 4x4 matrix
        Matrix::from_col(Vector::from_fn(|c| Vector::from_fn(|r| (0..COL).map(|k| self[k][r] * rhs[c][k]).sum())))
    }
}


impl<T, const ROW : usize, const COL : usize> Mul<Vector<T,COL>> for Matrix<T,ROW,COL>
    where
    T : NumberArithmetic,
    //Matrix<T,ROW,COL> : Mul<Matrix<T,ROW,1>>,
    Self : Mul<Matrix<T,COL,1>, Output = Matrix<T,ROW,1>>,
    Vector<T,COL> : From<Matrix::<T,ROW,1>>
{
    type Output = Vector<T,COL>;

    fn mul(self, rhs: Vector<T,COL>) -> Self::Output
    {
        let m : Matrix::<T,COL,1> = rhs.into();
        let r = self * m;
        r.into()
    }
}

impl<T, const COL : usize> MulAssign<Self> for SquareMatrix<T,COL>
    where
    Self : Mul<Self,Output = Self> + Copy
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T, const ROW : usize, const COL : usize> Mul<T> for Matrix<T,ROW,COL> where T : Copy + Mul<T>
{
    type Output = Matrix<T::Output,ROW,COL>;

    fn mul(self, rhs: T) -> Self::Output
    {
        Matrix::from_col(self.columns.map(|v| v.map(|i| i.mul(rhs))))
    }
}

impl<T, const ROW : usize, const COL : usize> MulAssign<T> for Matrix<T,ROW,COL> where T : Copy + Mul<T,Output = T>
{
    fn mul_assign(&mut self, rhs: T) {
        *self = (*self).mul(rhs);
    }
}

impl<T, const ROW : usize, const COL : usize> Div<T> for Matrix<T,ROW,COL> where T : Copy + Div<T>
{
    type Output = Matrix<T::Output,ROW,COL>;

    fn div(self, rhs: T) -> Self::Output
    {
        Matrix::from_col(self.columns.map(|v| v.map(|i| i.div(rhs))))
    }
}

impl<T, const ROW : usize, const COL : usize> DivAssign<T> for Matrix<T,ROW,COL> where T : Copy + Div<T,Output = T>
{
    fn div_assign(&mut self, rhs: T) {
        *self = (*self).div(rhs);
    }
}



impl<T, const N: usize> SquareMatrix<T, N>
where
    T: Float,
{
    /// Inverse the matrix, or return `None` if the matrix is singular.
    ///
    /// ```
    /// use hexga_math::prelude::*;
    ///
    /// assert_eq!(Mat2::IDENTITY.inverse(), Some(Mat2::IDENTITY));
    ///
    /// // Non inversible matrix
    /// let mut m2 = Mat2::IDENTITY;
    /// m2.set_y(vec2(0., 0.));
    /// assert_eq!(m2.inverse(), None);
    ///
    /// let m2 = Mat2::from_col(vector2(vec2(1., 3.), vec2(2., 4.)));
    /// let m2_inv = m2.inverse().unwrap();
    ///
    /// assert_eq!(m2 * m2_inv, Mat2::IDENTITY);
    /// ```
    pub fn inverse(mut self) -> Option<Self>
    {
        let mut augmented = Self::IDENTITY;

        for i in 0..N
        {
            if self[i][i] == T::ZERO { return None; }

            let pivot = self[i][i];

            for j in 0..N
            {
                self[i][j] /= pivot;
                augmented[i][j] /=  pivot;
            }

            for k in (i + 1)..N
            {
                let factor = self[k][i];
                for j in 0..N
                {
                    self[k][j] = self[k][j] - factor * self[i][j];
                    let a = augmented[i][j];
                    augmented[k][j] -= factor * a;
                }
            }
        }

        for i in (0..N).rev()
        {
            for k in 0..i
            {
                let factor = self[k][i];
                for j in 0..N
                {
                    let a = self[i][j];
                    self[k][j] -= factor * a;
                    let a = augmented[i][j];
                    augmented[k][j] -= factor * a;
                }
            }
        }

        Some(augmented)
    }
}


// Todo : impl det in a generic way once const generic will be here.
// Do a match on N, and hardcode the case for N in 0..=4

// There is no generic way to compute the determinant of a matrix, because it required calculaing the submatrix, but at the moment const generic operation are not possible
// I also don't want any heap allocation when computing the determinant
impl<T> SquareMatrix<T, 0> where T: NumberArithmetic + One,
{
    pub fn det(&self) -> T
    {
        T::ONE
    }
}

impl<T> SquareMatrix<T, 1> where T: NumberArithmetic + One,
{
    pub fn det(&self) -> T
    {
        self[0][0]
    }
}

impl<T> SquareMatrix<T, 2> where T: NumberArithmetic + One,
{
    pub fn det(&self) -> T
    {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<T> SquareMatrix<T, 3> where T: NumberArithmetic + One,
{
    pub fn det(&self) -> T
    {
          self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
        - self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0])
        + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }
}

impl<T> SquareMatrix<T, 4> where T: NumberArithmetic + One,
{
    pub fn det(&self) -> T
    {
        let a = self[2][2] * self[3][3] - self[2][3] * self[3][2];
        let b = self[2][1] * self[3][3] - self[2][3] * self[3][1];
        let c = self[2][1] * self[3][2] - self[2][2] * self[3][1];
        let d = self[2][0] * self[3][3] - self[2][3] * self[3][0];
        let e = self[2][0] * self[3][2] - self[2][2] * self[3][0];
        let f = self[2][0] * self[3][1] - self[2][1] * self[3][0];

        self[0][0] *
        (
              self[1][1] * a
            - self[1][2] * b
            + self[1][3] * c
        )

        - self[0][1] *
        (
            self[1][0] * a
          - self[1][2] * d
          + self[1][3] * e
        )

        + self[0][2] *
        (
            self[1][0] * b
          - self[1][1] * d
          + self[1][3] * f
        )

        - self[0][3] *
        (
            self[1][0] * c
          - self[1][1] * e
          + self[1][2] * f
        )
    }
}


macro_rules! matrix_have_x {
    ($dim : expr) =>
    {
        impl<T> HaveX<Vector<T,$dim>> for SquareMatrix<T,$dim>
        {
            fn iter_x<'a>(&'a self) -> impl Iterator<Item=&'a Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array().as_slice()[0..=Self::X_INDEX].iter()
            }

            fn iter_x_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array_mut().as_mut_slice()[0..=Self::X_INDEX].iter_mut()
            }
        }
    };
}

matrix_have_x!(1);
matrix_have_x!(2);
matrix_have_x!(3);
matrix_have_x!(4);

macro_rules! matrix_have_y {
    ($dim : expr) =>
    {
        impl<T> HaveY<Vector<T,$dim>> for SquareMatrix<T,$dim>
        {
            fn iter_xy<'a>(&'a self) -> impl Iterator<Item=&'a Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array().as_slice()[0..=Self::Y_INDEX].iter()
            }

            fn iter_xy_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array_mut().as_mut_slice()[0..=Self::Y_INDEX].iter_mut()
            }
        }
    };
}

matrix_have_y!(2);
matrix_have_y!(3);
matrix_have_y!(4);

macro_rules! matrix_have_z {
    ($dim : expr) =>
    {
        impl<T> HaveZ<Vector<T,$dim>> for SquareMatrix<T,$dim>
        {
            fn iter_xyz<'a>(&'a self) -> impl Iterator<Item=&'a Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array().as_slice()[0..=Self::Z_INDEX].iter()
            }

            fn iter_xyz_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array_mut().as_mut_slice()[0..=Self::Z_INDEX].iter_mut()
            }
        }
    };
}

matrix_have_z!(3);
matrix_have_z!(4);

macro_rules! matrix_have_w {
    ($dim : expr) =>
    {
        impl<T> HaveW<Vector<T,$dim>> for SquareMatrix<T,$dim>
        {
            fn iter_xyzw<'a>(&'a self) -> impl Iterator<Item=&'a Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array().as_slice()[0..=Self::W_INDEX].iter()
            }

            fn iter_xyzw_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut Vector<T,$dim>> where Vector<T,$dim>: 'a {
                self.columns.as_array_mut().as_mut_slice()[0..=Self::W_INDEX].iter_mut()
            }
        }
    };
}

matrix_have_w!(4);

impl<T, const N : usize> RotationX<T> for SquareMatrix<T,N>
    where
        Self : HaveZ<Vector<T,N>> + Zero + Mul<Self, Output = Self>,
        T : Float
{
    fn rotate_x(&mut self, angle : AngleOf<T>) -> &mut Self
    {
        *self = Self::from_rotation_x(angle) * (*self);
        self
    }
}

impl<T, const N : usize> RotationY<T> for SquareMatrix<T,N>
    where
        Self : HaveZ<Vector<T,N>> + Zero + Mul<Self, Output = Self>,
        T : Float
{
    fn rotate_y(&mut self, angle : AngleOf<T>) -> &mut Self
    {
        *self = Self::from_rotation_y(angle) * (*self);
        self
    }
}

impl<T, const N : usize> RotationZ<T> for SquareMatrix<T,N>
    where
        Self : HaveY<Vector<T,N>> + Zero + Mul<Self, Output = Self>,
        T : Float
{
    fn rotate_z(&mut self, angle : AngleOf<T>) -> &mut Self
    {
        *self = Self::from_rotation_z(angle) * (*self);
        self
    }
}

impl<T, const N : usize> Position<Vector<T,N>,N> for SquareMatrix<T,N> where T : Copy
{
    fn pos(&self) -> Vector<Vector<T,N>,N> {
        self.col()
    }

    fn set_pos(&mut self, pos : Vector<Vector<T,N>,N>) -> &mut Self {
        *self = Self::from_col(pos);
        self
    }
}

impl<T, const N : usize> Position<T,N> for SquareMatrix<T,N> where T : Copy
{
    fn pos(&self) -> Vector<T,N> {
        self.columns[N-1]
    }

    fn set_pos(&mut self, pos : Vector<T,N>) -> &mut Self {
        self.columns[N-1] = pos;
        self
    }
}


/// the rotation code is mainly based on glam code
impl<T, const N : usize> SquareMatrix<T,N> where Self : HaveZ<Vector<T,N>> + Zero, T : Float
{
    pub fn from_rotation_axis(axis: Vector3<T>, angle : AngleOf<T>) -> Self
    {
        let (sin, cos) = T::sin_cos(angle.radian());
        let axis_sin = axis.mul(sin);
        let axis_sq = axis.mul(axis);
        let omc = T::ONE - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;

        let mut r = Self::ZERO;
        r[0][0] = axis_sq.x * omc + cos;
        r[0][1] = xyomc + axis_sin.z;
        r[0][2] = xzomc - axis_sin.y;

        r[1][0] = xyomc - axis_sin.z;
        r[1][1] = axis_sq.y * omc + cos;
        r[1][2] = yzomc + axis_sin.x;

        r[2][0] = xzomc + axis_sin.y;
        r[2][1] = yzomc - axis_sin.x;
        r[2][2] = axis_sq.z * omc + cos;

        r
    }

    pub fn from_rotation_x(angle : AngleOf<T>) -> Self
    {
        let (sina, cosa) = T::sin_cos(angle.radian());
        let mut r = Self::ZERO;

        r[Self::Y_INDEX][Self::Y_INDEX] =  cosa;
        r[Self::Y_INDEX][Self::Z_INDEX] =  sina;
        r[Self::Z_INDEX][Self::Y_INDEX] = -sina;
        r[Self::Z_INDEX][Self::Z_INDEX] =  cosa;
        r
    }

    pub fn from_rotation_y(angle : AngleOf<T>) -> Self
    {
        let (sina, cosa) = T::sin_cos(angle.radian());
        let mut r = Self::ZERO;

        r[Self::X_INDEX][Self::X_INDEX] =  cosa;
        r[Self::X_INDEX][Self::Z_INDEX] = -sina;
        r[Self::Z_INDEX][Self::X_INDEX] =  sina;
        r[Self::Z_INDEX][Self::Z_INDEX] =  cosa;
        r
    }
}

impl<T, const N : usize> SquareMatrix<T,N> where Self : HaveY<Vector<T,N>> + Zero, T : Float
{
    pub fn from_rotation_z(angle : AngleOf<T>) -> Self
    {
        let (sina, cosa) = T::sin_cos(angle.radian());
        let mut r = Self::ZERO;

        r[Self::X_INDEX][Self::X_INDEX] =  cosa;
        r[Self::X_INDEX][Self::Y_INDEX] =  sina;
        r[Self::Y_INDEX][Self::X_INDEX] = -sina;
        r[Self::Y_INDEX][Self::Y_INDEX] =  cosa;
        r
    }
}

impl<T, const N : usize> SquareMatrix<T,N> where Self : HaveZ<Vector<T,N>> + One, T : Copy + Zero + One
{
    /// Put an [One] for the last component
    pub fn from_scale(scale : Vector<T,N>) -> Self
    {
        //let scale = scale.to_vector_filled(T::ONE);

        let mut r = Self::IDENTITY;
        for i in 0..N
        {
            r[i][i] = scale[i];
        }
        r
    }
    /// Put an [Zero] for the last component
    pub fn from_translation(translation: Vector<T,N>) -> Self where T : Zero + AddAssign<T>
    {
        //let translation = translation.to_vector_filled(T::ZERO);
        let mut r = Self::IDENTITY;
        for i in 0..N
        {
            r[N-1][i] += translation[i];
        }
        r
    }
}


#[cfg(test)]
mod test_matrix
{
    use crate::*;

    #[test]
    fn rotation_zero_on_z()
    {
        let m = Mat2::from_col_array([[1.,2.], [3.,4.]]);
        assert_eq!(m.rotated_z(0.degree()), m);
    }
}