use super::*;


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




/// NxN matrix
pub type SquareMatrix<T, const N : usize> = Matrix<T, N, N>;

/// ROW rows, COL columns.
///
/// Can be indexed `matrix[row][col]`
///
/// The [(0,0)] index is in the top left corner
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct Matrix<T, const ROW : usize, const COL : usize>
{
    pub columns : Vector<Vector<T, ROW>,COL>,
}


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
    fn _fmt(&self, f: &mut Formatter<'_>, d : impl Fn(&T, &mut Formatter<'_>) -> FmtResult) -> FmtResult
    {
        const SEP :&'static str = " ";

        let mut strings: Vec<String> = Vec::with_capacity(ROW * COL);

        let mut width = 0;

        for c in 0..COL
        {
            for r in 0..ROW
            {
                strings.push(___());
                let mut tmp_f = std::fmt::Formatter::new(strings.last_mut().unwrap(), ___());
                d(&self[c][r], &mut tmp_f)?;
                width = max(width, strings.last().unwrap().len());
            }
        }

        for c in 0..COL
        {
            for r in 0..ROW
            {
                let idx = r * COL + c;
                write!(f, "{:>width$}{}", strings[idx], SEP, width = width)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

use std::fmt;
impl<T, const ROW : usize, const COL : usize> fmt::Display  for Matrix<T, ROW, COL> where T: fmt::Display  { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::Debug    for Matrix<T, ROW, COL> where T: fmt::Debug    { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::Octal    for Matrix<T, ROW, COL> where T: fmt::Octal    { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::Binary   for Matrix<T, ROW, COL> where T: fmt::Binary   { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::LowerHex for Matrix<T, ROW, COL> where T: fmt::LowerHex { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::UpperHex for Matrix<T, ROW, COL> where T: fmt::UpperHex { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::LowerExp for Matrix<T, ROW, COL> where T: fmt::LowerExp { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::UpperExp for Matrix<T, ROW, COL> where T: fmt::UpperExp { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }
impl<T, const ROW : usize, const COL : usize> fmt::Pointer  for Matrix<T, ROW, COL> where T: fmt::Pointer  { fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult  { self._fmt(f, T::fmt) } }

impl<T, const ROW : usize, const COL : usize> Default for Matrix<T,ROW,COL> where Vector<Vector<T, ROW>,COL> : Default
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
    pub fn from_fn<F>(mut column_then_row : F) -> Self where F : FnMut(Vec2i) -> T
    {
        Self::from_col_array(std::array::from_fn(|column| std::array::from_fn(|row| column_then_row(vec2i(column as _,row as _)))))
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

    pub fn from_row(rows : Vector<Vector<T, COL>,ROW>) -> Self where T: Copy
    {
        Matrix::from_col(rows).transpose()
    }
    pub fn from_row_array(rows : [[T;COL];ROW]) -> Self where T: Copy
    {
        Self::from_row(Vector::from_array(rows.map(|array| Vector::from_array(array))))
    }

    pub fn col(&self) -> Vector<Vector<T, ROW>,COL> where T: Copy { self.columns }
    pub fn row(&self) -> Vector<Vector<T, COL>,ROW> where T: Copy { self.transpose().columns }

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
    pub fn transpose(&self) -> Matrix<T,COL,ROW> where T: Copy
    {
        Matrix::from_col(Vector::from_fn(|x| Vector::from_fn(|y| self[y][x])))
    }
}


impl<T, const ROW : usize, const COL : usize> GetMatrix<T,ROW,COL> for Matrix<T,ROW,COL> where Self: Copy
{
    fn matrix(&self) -> Matrix<T,ROW,COL> {
        *self
    }
}

impl<T, const ROW : usize, const COL : usize> SetMatrix<T,ROW,COL> for Matrix<T,ROW,COL> where Self: Copy
{
    fn set_matrix(&mut self, matrix : Matrix<T,ROW,COL>) -> &mut Self {
        *self = matrix; self
    }
}


// 2D
impl<T> SquareMatrix<T,2>
    where
    Self : Copy,
    SquareMatrix<T, 2> : Mul<Vector<T, 2>, Output = Vector<T, 2>>,
    Vector<T, 2> : Into< Vector<T, 1>>
{
    pub fn transform_relative(self, relative : Vector<T, 1>) -> Vector<T, 1> where T: Zero
    {
        self.transform(relative.with_y(T::ZERO)).into()
    }

    pub fn transform_position(self, position : Vector<T, 1>) -> Vector<T, 1> where T: One
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
    pub fn transform_relative(self, relative : Vector<T, 2>) -> Vector<T, 2> where T: Zero
    {
        self.transform(relative.with_z(T::ZERO)).into()
    }

    pub fn transform_position(self, position : Vector<T, 2>) -> Vector<T, 2> where T: One
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
    pub fn transform_relative(self, relative : Vector<T, 3>) -> Vector<T, 3> where T: Zero
    {
        self.transform(relative.with_w(T::ZERO)).into()
    }

    pub fn transform_position(self, position : Vector<T, 3>) -> Vector<T, 3> where T: One
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
    pub fn transform_relative(self, relative : Vector<T, N>) -> Vector<T, N> where T: Zero
    {
        self.transform(relative.with(N-1, T::ZERO))
    }

    /// The last value/component will be replaced by 1 because it is fixed/point in math terms.
    pub fn transform_fixed(self, fixed : Vector<T, N>) -> Vector<T, N> where T: Zero
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

impl<T, const N : usize> SquareMatrix<T,N> where T: One + Zero + Copy
{
    /// Same as [One]
    pub const IDENTITY : Self = Self::ONE;
}


impl<T, const N : usize> One for SquareMatrix<T,N> where T: One + Zero + Copy
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

impl<T, const N : usize> MinusOne for SquareMatrix<T,N> where T: MinusOne + Zero + Copy
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

impl<T, const N : usize> Half for SquareMatrix<T,N> where T: Half + Zero + Copy
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


impl<T, const N : usize> SquareMatrix<T,N> where T: Copy
{
    /// Return the diagonal
    pub fn diag(&self) -> Vector<T,N> { Vector::from_fn(|i| self[i][i]) }
    /// Set the value on the diagonal
    pub fn set_diag(&mut self, diag: Vector<T,N>)
    {
        for i in 0..N
        {
            self[i][i] = diag[i];
        }
    }
}

map_on!(
    (
        (MinValue, MIN),
        (MaxValue, MAX),
        (NaNValue, NAN)
    ),
    (($trait_name : ident, $constant_name : ident)) =>
    {
        impl<T, const ROW : usize, const COL : usize> $trait_name for Matrix<T,ROW,COL> where T: $trait_name + Copy
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
/// let m1  = Mat2P::from_col((vec2i(7, 6), vec2i(5, 3)).into());
/// let m2  = Mat2P::from_col((vec2i(2, 5), vec2i(1, 1)).into());
/// let m3  = Mat2P::from_col((vec2i(39, 27), vec2i(12, 9)).into());
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
    T : Numeric,
{
    type Output = Matrix<T, ROW, COL2>;

    fn mul(self, rhs: Matrix<T,COL,COL2>) -> Self::Output
    {
        // I'm not sure if the unrolling is necessary

        match (ROW, COL, COL2) {
            // 2x2 case (manual unroll)
            (2, 2, 2) => {
                let mut r = Self::Output::ZERO;

                r[0][0] = self[0][0]*rhs[0][0] + self[1][0]*rhs[0][1];
                r[0][1] = self[0][1]*rhs[0][0] + self[1][1]*rhs[0][1];

                r[1][0] = self[0][0]*rhs[1][0] + self[1][0]*rhs[1][1];
                r[1][1] = self[0][1]*rhs[1][0] + self[1][1]*rhs[1][1];

                r
            }

            // 3x3 case (manual unroll)
            (3, 3, 3) => {
                let mut r = Self::Output::ZERO;

                r[0][0] = self[0][0]*rhs[0][0] + self[1][0]*rhs[0][1] + self[2][0]*rhs[0][2];
                r[0][1] = self[0][1]*rhs[0][0] + self[1][1]*rhs[0][1] + self[2][1]*rhs[0][2];
                r[0][2] = self[0][2]*rhs[0][0] + self[1][2]*rhs[0][1] + self[2][2]*rhs[0][2];

                r[1][0] = self[0][0]*rhs[1][0] + self[1][0]*rhs[1][1] + self[2][0]*rhs[1][2];
                r[1][1] = self[0][1]*rhs[1][0] + self[1][1]*rhs[1][1] + self[2][1]*rhs[1][2];
                r[1][2] = self[0][2]*rhs[1][0] + self[1][2]*rhs[1][1] + self[2][2]*rhs[1][2];

                r[2][0] = self[0][0]*rhs[2][0] + self[1][0]*rhs[2][1] + self[2][0]*rhs[2][2];
                r[2][1] = self[0][1]*rhs[2][0] + self[1][1]*rhs[2][1] + self[2][1]*rhs[2][2];
                r[2][2] = self[0][2]*rhs[2][0] + self[1][2]*rhs[2][1] + self[2][2]*rhs[2][2];

                r
            }

            // 4x4 case (manual unroll)
            (4, 4, 4) => {
                let mut r = Self::Output::ZERO;

                r[0][0] = self[0][0]*rhs[0][0] + self[1][0]*rhs[0][1] + self[2][0]*rhs[0][2] + self[3][0]*rhs[0][3];
                r[0][1] = self[0][1]*rhs[0][0] + self[1][1]*rhs[0][1] + self[2][1]*rhs[0][2] + self[3][1]*rhs[0][3];
                r[0][2] = self[0][2]*rhs[0][0] + self[1][2]*rhs[0][1] + self[2][2]*rhs[0][2] + self[3][2]*rhs[0][3];
                r[0][3] = self[0][3]*rhs[0][0] + self[1][3]*rhs[0][1] + self[2][3]*rhs[0][2] + self[3][3]*rhs[0][3];

                r[1][0] = self[0][0]*rhs[1][0] + self[1][0]*rhs[1][1] + self[2][0]*rhs[1][2] + self[3][0]*rhs[1][3];
                r[1][1] = self[0][1]*rhs[1][0] + self[1][1]*rhs[1][1] + self[2][1]*rhs[1][2] + self[3][1]*rhs[1][3];
                r[1][2] = self[0][2]*rhs[1][0] + self[1][2]*rhs[1][1] + self[2][2]*rhs[1][2] + self[3][2]*rhs[1][3];
                r[1][3] = self[0][3]*rhs[1][0] + self[1][3]*rhs[1][1] + self[2][3]*rhs[1][2] + self[3][3]*rhs[1][3];

                r[2][0] = self[0][0]*rhs[2][0] + self[1][0]*rhs[2][1] + self[2][0]*rhs[2][2] + self[3][0]*rhs[2][3];
                r[2][1] = self[0][1]*rhs[2][0] + self[1][1]*rhs[2][1] + self[2][1]*rhs[2][2] + self[3][1]*rhs[2][3];
                r[2][2] = self[0][2]*rhs[2][0] + self[1][2]*rhs[2][1] + self[2][2]*rhs[2][2] + self[3][2]*rhs[2][3];
                r[2][3] = self[0][3]*rhs[2][0] + self[1][3]*rhs[2][1] + self[2][3]*rhs[2][2] + self[3][3]*rhs[2][3];

                r[3][0] = self[0][0]*rhs[3][0] + self[1][0]*rhs[3][1] + self[2][0]*rhs[3][2] + self[3][0]*rhs[3][3];
                r[3][1] = self[0][1]*rhs[3][0] + self[1][1]*rhs[3][1] + self[2][1]*rhs[3][2] + self[3][1]*rhs[3][3];
                r[3][2] = self[0][2]*rhs[3][0] + self[1][2]*rhs[3][1] + self[2][2]*rhs[3][2] + self[3][2]*rhs[3][3];
                r[3][3] = self[0][3]*rhs[3][0] + self[1][3]*rhs[3][1] + self[2][3]*rhs[3][2] + self[3][3]*rhs[3][3];

                r
            }
            _ => Matrix::from_col(Vector::from_fn(|c| Vector::from_fn(|r| (0..COL).map(|k| self[k][r] * rhs[c][k]).sum()))),
        }
    }
}


impl<T, const ROW : usize, const COL : usize> Mul<Vector<T,COL>> for Matrix<T,ROW,COL>
    where
    T : Numeric,
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

impl<T, const ROW : usize, const COL : usize> Mul<T> for Matrix<T,ROW,COL> where T: Copy + Mul<T>
{
    type Output = Matrix<T::Output,ROW,COL>;

    fn mul(self, rhs: T) -> Self::Output
    {
        Matrix::from_col(self.columns.map(|v| v.map(|i| i.mul(rhs))))
    }
}

impl<T, const ROW : usize, const COL : usize> MulAssign<T> for Matrix<T,ROW,COL> where T: Copy + Mul<T,Output = T>
{
    fn mul_assign(&mut self, rhs: T) {
        *self = (*self).mul(rhs);
    }
}

impl<T, const ROW : usize, const COL : usize> Div<T> for Matrix<T,ROW,COL> where T: Copy + Div<T>
{
    type Output = Matrix<T::Output,ROW,COL>;

    fn div(self, rhs: T) -> Self::Output
    {
        Matrix::from_col(self.columns.map(|v| v.map(|i| i.div(rhs))))
    }
}

impl<T, const ROW : usize, const COL : usize> DivAssign<T> for Matrix<T,ROW,COL> where T: Copy + Div<T,Output = T>
{
    fn div_assign(&mut self, rhs: T) {
        *self = (*self).div(rhs);
    }
}


impl<T, const ROW : usize, const COL : usize> MapIntern for Matrix<T,ROW,COL>
{
    type Item=T;
    fn map_intern<F>(self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item
    {
        self.map(f)
    }

}
impl<T, const ROW : usize, const COL : usize> MapWithIntern for Matrix<T,ROW,COL>
{
    fn map_with_intern<F>(self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        self.map_with(other, f)
    }
}
impl<T, const ROW : usize, const COL : usize> Map for Matrix<T,ROW,COL>
{
    type WithType<R> = Matrix<R,ROW,COL>;

    fn map<R,F>(self, mut f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R {
        let mut it = self.columns.into_iter();
        let cols = std::array::from_fn(|_| Map::map(it.next().unwrap(), &mut f));
        Self::WithType::<R>::from_col(Vector::from_array(cols))
    }
}
impl<T, const ROW : usize, const COL : usize> MapWith for Matrix<T,ROW,COL>
{
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, mut f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        let mut it1 = self.columns.into_iter();
        let mut it2 = other.columns.into_iter();
        let cols = std::array::from_fn(|_| MapWith::map_with(it1.next().unwrap(), it2.next().unwrap(), &mut f));
        Self::WithType::<R>::from_col(Vector::from_array(cols))
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
impl<T> SquareMatrix<T, 0> where T: Numeric + One,
{
    pub fn det(&self) -> T
    {
        T::ONE
    }
}

impl<T> SquareMatrix<T, 1> where T: Numeric + One,
{
    pub fn det(&self) -> T
    {
        self[0][0]
    }
}

impl<T> SquareMatrix<T, 2> where T: Numeric + One,
{
    pub fn det(&self) -> T
    {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<T> SquareMatrix<T, 3> where T: Numeric + One,
{
    pub fn det(&self) -> T
    {
          self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
        - self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0])
        + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }
}

impl<T> SquareMatrix<T, 4> where T: Numeric + One,
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

impl<T, const N : usize> RotateX<T> for SquareMatrix<T,N>
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

impl<T, const N : usize> RotateY<T> for SquareMatrix<T,N>
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

impl<T, const N : usize> RotateZ<T> for SquareMatrix<T,N>
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

/*
impl<T, const N : usize> GetPosition<Vector<T,N>,N> for SquareMatrix<T,N> where T: Copy
{
    fn pos(&self) -> Vector<Vector<T,N>,N> {
        self.col()
    }
}
impl<T, const N : usize> SetPosition<Vector<T,N>,N> for SquareMatrix<T,N> where T: Copy
{
    fn set_pos(&mut self, pos : Vector<Vector<T,N>,N>) -> &mut Self {
        *self = Self::from_col(pos);
        self
    }
}
*/

impl<T> GetPosition<T,2> for SquareMatrix<T,3> where T: Copy
{
    fn pos(&self) -> Vector<T,2> { self.columns[2].into() }
}
impl<T> SetPosition<T,2> for SquareMatrix<T,3> where T: Copy
{
    fn set_pos(&mut self, pos : Vector<T,2>) -> &mut Self { self.columns[2] = pos.with_z(self[2][2]); self }
}
impl<T> GetPosition<T,3> for SquareMatrix<T,4> where T: Copy
{
    fn pos(&self) -> Vector<T,3> { self.columns[3].into() }
}
impl<T> SetPosition<T,3> for SquareMatrix<T,4> where T: Copy
{
    fn set_pos(&mut self, pos : Vector<T,3>) -> &mut Self { self.columns[3] = pos.with_w(self[3][3]); self }
}

impl<T> GetScale<T,2> for SquareMatrix<T,3> where T: Float
{
    fn scale(&self) -> Vector<T,2> { Vector::from_fn(|i| Vector::<T,2>::from(self[i]).length() ) }
}
impl<T> SetScale<T,2> for SquareMatrix<T,3> where T: Float
{
    fn set_scale(&mut self, scale : Vector<T,2>) -> &mut Self {
        for i in 0..2
        {
            let len = Vector::<T, 2>::from(self[i]).length();
            if len.is_non_zero()
            {
                let factor = scale[i] / len;
                for j in 0..2
                {
                    self[i][j] = self[i][j] * factor;
                }
            }
        }
        self
    }
}
impl<T> GetScale<T,3> for SquareMatrix<T,4> where T: Float
{
    fn scale(&self) -> Vector<T,3> { Vector::from_fn(|i| Vector::<T,3>::from(self[i]).length() ) }
}
impl<T> SetScale<T,3> for SquareMatrix<T,4> where T: Float
{
    fn set_scale(&mut self, scale : Vector<T,3>) -> &mut Self {
        for i in 0..3
        {
            let len = Vector::<T, 3>::from(self[i]).length();
            if len.is_non_zero()
            {
                let factor = scale[i] / len;
                for j in 0..3
                {
                    self[i][j] = self[i][j] * factor;
                }
            }
        }
        self
    }
}

/// the rotation code is mainly based on glam code
impl<T, const N : usize> SquareMatrix<T,N> where Self : HaveZ<Vector<T,N>> + Zero, T : Float
{
    pub fn from_rotation_axis(axis: Vector3<T>, angle : AngleOf<T>) -> Self
    {
        let (sin, cos) = angle.sin_cos();
        let axis = axis.normalized();
        let axis_sin = axis * sin;
        let axis_sq = axis * axis;
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
        let (sina, cosa) = angle.sin_cos();
        let mut r = Self::IDENTITY;

        r[Self::Y_INDEX][Self::Y_INDEX] =  cosa;
        r[Self::Y_INDEX][Self::Z_INDEX] =  sina;
        r[Self::Z_INDEX][Self::Y_INDEX] = -sina;
        r[Self::Z_INDEX][Self::Z_INDEX] =  cosa;
        r
    }

    pub fn from_rotation_y(angle : AngleOf<T>) -> Self
    {
        let (sina, cosa) = angle.sin_cos();
        let mut r = Self::IDENTITY;

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
        let (sina, cosa) = angle.sin_cos();
        let mut r = Self::IDENTITY;

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
    pub fn from_translation(translation: Vector<T,N>) -> Self where T: Zero + AddAssign<T>
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


// Impl based on the glam crate
impl<T> Matrix4<T>
{
    /// Creates a right-handed view matrix.
    ///
    /// View space: +X = right, +Y = up, +Z = back (camera looks toward -Z).
    #[inline]
    #[must_use]
    pub fn look_at_rh(position: Vector3<T>, center: Vector3<T>, up: Vector3<T>) -> Self where T: Float
    {
        Self::look_to_rh(position, center - position, up)
    }

    /// Creates a right-handed view matrix from a direction vector.
    ///
    /// View space: +X = right, +Y = up, +Z = back (camera looks toward -Z).
    #[inline]
    #[must_use]
    pub fn look_to_rh(position: Vector3<T>, dir: Vector3<T>, up: Vector3<T>) -> Self where T: Float
    {
        let f = dir.normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(f);

        Self::from_col(
            vector4
            (
                vector4(s.x, u.x, -f.x, zero()),
                vector4(s.y, u.y, -f.y, zero()),
                vector4(s.z, u.z, -f.z, zero()),
                vector4(-position.dot(s), position.dot(u), position.dot(f), one())
            )
        )
    }


    /// Creates a left-handed view matrix.
    ///
    /// View space: +X = right, +Y = up, +Z = forward (camera looks toward +Z).
    #[inline]
    #[must_use]
    pub fn look_at_lh(position: Vector3<T>, center: Vector3<T>, up: Vector3<T>) -> Self where T: Float,
    {
        Self::look_to_lh(position, center - position, up)
    }

    /// Creates a left-handed view matrix from a direction vector.
    ///
    /// View space: +X = right, +Y = up, +Z = forward (camera looks toward +Z).
    #[inline]
    #[must_use]
    pub fn look_to_lh(position: Vector3<T>, dir: Vector3<T>, up: Vector3<T>) -> Self where T: Float,
    {
        let f = dir.normalized();
        let s = up.cross(f).normalized();
        let u = f.cross(s);

        Self::from_col(
            vector4(
                vector4(s.x, u.x, f.x, zero()),
                vector4(s.y, u.y, f.y, zero()),
                vector4(s.z, u.z, f.z, zero()),
                vector4(-position.dot(s), -position.dot(u), -position.dot(f), one()),
            )
        )
    }
}


#[cfg(test)]
mod test_matrix
{
    use super::*;

    #[test]
    fn rotation_zero_on_z()
    {
        let m = Mat2::from_row_array([[1.,2.], [3.,4.]]);
        assert_eq!(m.rotated_z(0.degree()), m);
    }

    #[test]
    fn multiplication_2x2_basic()
    {
        let a = Mat2P::from_row_array([[1, 2], [3, 4]]);
        let b = Mat2P::from_row_array([[5, 6], [7, 8]]);
        let expected = Mat2P::from_row_array([[19, 22], [43, 50]]);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplication_2x2_identity()
    {
        let a = Mat2P::from_row_array([[2, 3], [4, 5]]);
        let identity = Mat2P::IDENTITY;

        assert_eq!(a * identity, a);
        assert_eq!(identity * a, a);
    }

    #[test]
    fn multiplication_2x2_zero()
    {
        let a = Mat2P::from_row_array([[1, 2], [3, 4]]);
        let zero = Mat2P::ZERO;

        assert_eq!(a * zero, zero);
        assert_eq!(zero * a, zero);
    }

    #[test]
    fn multiplication_2x2_float()
    {
        let a = Mat2::from_row_array([[1.5, 2.5], [3.5, 4.5]]);
        let b = Mat2::from_row_array([[0.5, 1.5], [2.5, 3.5]]);
        let expected = Mat2::from_row_array([[7.0, 11.0], [13.0, 21.0]]);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplication_2x2_negative()
    {
        let a = Mat2P::from_row_array([[-1, -2], [-3, -4]]);
        let b = Mat2P::from_row_array([[1, 2], [-3, -4]]);
        let expected = Mat2P::from_row_array([[5, 6], [9, 10]]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplication_2x2_rotation()
    {
        let rot90 = Mat2::from_row_array([[0.0, 1.0], [-1.0, 0.0]]);
        let rot180 = Mat2::from_row_array([[-1.0, 0.0], [0.0, -1.0]]);
        assert_eq!(rot90 * rot90, rot180);
    }

    #[test]
    fn multiplication_3x3_basic()
    {
        let a = Mat3P::from_row_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let b = Mat3P::from_row_array([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
        let expected = Mat3P::from_row_array([[30, 24, 18], [84, 69, 54], [138, 114, 90]]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplication_3x3_identity()
    {
        let a = Mat3P::from_row_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let identity = Mat3P::IDENTITY;

        assert_eq!(a * identity, a);
        assert_eq!(identity * a, a);
    }

    #[test]
    fn multiplication_3x3_zero()
    {
        let a = Mat3P::from_row_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let zero = Mat3P::ZERO;

        assert_eq!(a * zero, zero);
        assert_eq!(zero * a, zero);
    }

    #[test]
    fn multiplication_3x3_rotation()
    {
        let rot90 = Mat3::from_row_array([[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        let rot180 = Mat3::from_row_array([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]]);

        // 90° rotation twice should equal 180° rotation
        assert_eq!(rot90 * rot90, rot180);
    }

    #[test]
    fn multiplication_4x4_basic()
    {
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
        let b = Mat4P::from_row_array([[16, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]]);
        let expected = Mat4P::from_row_array([[80, 70, 60, 50], [240, 214, 188, 162], [400, 358, 316, 274], [560, 502, 444, 386]]);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplication_4x4_identity()
    {
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
        let identity = Mat4P::IDENTITY;

        assert_eq!(a * identity, a);
        assert_eq!(identity * a, a);
    }

    #[test]
    fn multiplication_4x4_zero()
    {
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
        let zero = Mat4P::ZERO;

        assert_eq!(a * zero, zero);
        assert_eq!(zero * a, zero);
    }


    #[test]
    fn multiplication_4x4_translation()
    {
        let translate1 = Mat4::from_translation(vec3(1.0, 2.0, 3.0).into());
        let translate2 = Mat4::from_translation(vec3(4.0, 5.0, 6.0).into());
        let expected = Mat4::from_translation(vec3(5.0, 7.0, 9.0).into());

        assert_eq!(translate1 * translate2, expected);
    }

    #[test]
    fn multiplication_4x4_scale()
    {
        let scale1 = Mat4::from_scale(vec3(2.0, 3.0, 4.0).into());
        let scale2 = Mat4::from_scale(vec3(0.5, 0.25, 0.125).into());
        let expected = Mat4::from_scale(vec3(1.0, 0.75, 0.5).into());

        assert_eq!(scale1 * scale2, expected);
    }

    #[test]
    fn multiplication_associativity_2x2()
    {
        let a = Mat2P::from_row_array([[1, 2], [3, 4]]);
        let b = Mat2P::from_row_array([[5, 6], [7, 8]]);
        let c = Mat2P::from_row_array([[9, 10], [11, 12]]);

        assert_eq!((a * b) * c, a * (b * c));
    }

    #[test]
    fn multiplication_associativity_3x3()
    {
        let a = Mat3P::from_row_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let b = Mat3P::from_row_array([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
        let c = Mat3P::from_row_array([[1, 0, 1], [0, 1, 0], [1, 0, 1]]);

        assert_eq!((a * b) * c, a * (b * c));
    }

    #[test]
    fn multiplication_associativity_4x4()
    {
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
        let b = Mat4P::from_row_array([[16, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]]);
        let c = Mat4P::from_row_array([[1, 0, 0, 1], [0, 1, 0, 0], [0, 0, 1, 0], [1, 0, 0, 1]]);

        assert_eq!((a * b) * c, a * (b * c));
    }

    #[test]
    fn multiplication_distributivity_2x2()
    {
        let a = Mat2P::from_row_array([[1, 2], [3, 4]]);
        let b = Mat2P::from_row_array([[5, 6], [7, 8]]);
        let c = Mat2P::from_row_array([[9, 10], [11, 12]]);

        assert_eq!(a * (b + c), a * b + a * c);
    }

    #[test]
    fn multiplication_distributivity_3x3()
    {
        let a = Mat3P::from_row_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let b = Mat3P::from_row_array([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
        let c = Mat3P::from_row_array([[1, 0, 1], [0, 1, 0], [1, 0, 1]]);

        assert_eq!(a * (b + c), a * b + a * c);
    }

    #[test]
    fn multiplication_distributivity_4x4()
    {
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
        let b = Mat4P::from_row_array([[16, 15, 14, 13], [12, 11, 10, 9], [8, 7, 6, 5], [4, 3, 2, 1]]);
        let c = Mat4P::from_row_array([[1, 0, 0, 1], [0, 1, 0, 0], [0, 0, 1, 0], [1, 0, 0, 1]]);

        assert_eq!(a * (b + c), a * b + a * c);
    }

    #[test]
    fn multiplication_large_numbers_2x2()
    {
        let a = Mat2P::from_row_array([[1000, 2000], [3000, 4000]]);
        let b = Mat2P::from_row_array([[5000, 6000], [7000, 8000]]);
        let expected = Mat2P::from_row_array([[19000000, 22000000], [43000000, 50000000]]);

        assert_eq!(a * b, expected);
    }


    #[test]
    fn multiplication_non_commutative_2x2()
    {
        let a = Mat2P::from_row_array([[1, 2], [3, 4]]);
        let b = Mat2P::from_row_array([[2, 1], [4, 3]]);

        assert_ne!(a * b, b * a);
    }

    #[test]
    fn multiplication_non_commutative_3x3()
    {
        let a = Mat3P::from_row_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let b = Mat3P::from_row_array([[2, 1, 0], [5, 4, 3], [8, 7, 6]]);

        assert_ne!(a * b, b * a);
    }

    #[test]
    fn multiplication_non_commutative_4x4()
    {
        // Test non-commutativity for 4x4 matrices
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
        let b = Mat4P::from_row_array([[2, 1, 0, 0], [6, 5, 4, 3], [10, 9, 8, 7], [14, 13, 12, 11]]);

        assert_ne!(a * b, b * a);
    }


    #[test]
    fn multiplication_symmetric_2x2()
    {
        // Test multiplication with symmetric matrices
        let a = Mat2P::from_row_array([[1, 2], [2, 3]]);
        let b = Mat2P::from_row_array([[4, 5], [5, 6]]);

        // (A * B)^T = B^T * A^T = B * A (since both are symmetric)
        assert_eq!((a * b).transpose(), b * a);
    }

    #[test]
    fn multiplication_symmetric_3x3()
    {
        // Test multiplication with 3x3 symmetric matrices
        let a = Mat3P::from_row_array([[1, 2, 3], [2, 4, 5], [3, 5, 6]]);
        let b = Mat3P::from_row_array([[2, 1, 0], [1, 3, 2], [0, 2, 4]]);

        // (A * B)^T = B^T * A^T = B * A (since both are symmetric)
        assert_eq!((a * b).transpose(), b * a);
    }

    #[test]
    fn multiplication_symmetric_4x4()
    {
        // Test multiplication with 4x4 symmetric matrices
        let a = Mat4P::from_row_array([[1, 2, 3, 4], [2, 5, 6, 7], [3, 6, 8, 9], [4, 7, 9, 10]]);
        let b = Mat4P::from_row_array([[2, 1, 0, 0], [1, 3, 2, 0], [0, 2, 4, 3], [0, 0, 3, 5]]);

        // (A * B)^T = B^T * A^T = B * A (since both are symmetric)
        assert_eq!((a * b).transpose(), b * a);
    }
}