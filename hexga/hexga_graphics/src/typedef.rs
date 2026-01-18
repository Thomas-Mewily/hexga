use super::*;

#[allow(non_camel_case_types)]
pub type Gpufloat = f32;

pub type GpuVector<const N:usize> = Vector<Gpufloat,N>;
pub type GpuVec1 = Vector1<Gpufloat>;
pub type GpuVec2 = Vector2<Gpufloat>;
pub type GpuVec3 = Vector3<Gpufloat>;
pub type GpuVec4 = Vector4<Gpufloat>;

pub const fn gpu_vec1(x: Gpufloat) -> GpuVec1 { GpuVec1::new(x) }
pub const fn gpu_vec2(x: Gpufloat, y: Gpufloat) -> GpuVec2 { GpuVec2::new(x, y) }
pub const fn gpu_vec3(x: Gpufloat, y: Gpufloat, z: Gpufloat) -> GpuVec3 { GpuVec3::new(x, y, z) }
pub const fn gpu_vec4(x: Gpufloat, y: Gpufloat, z: Gpufloat, w: Gpufloat) -> GpuVec4 { GpuVec4::new(x, y, z, w) }

pub type GpuRectangle<const N:usize> = Rectangle<Gpufloat,N>;
pub type GpuRect1 = GpuRectangle<1>;
pub type GpuRect2 = GpuRectangle<2>;
pub type GpuRect3 = GpuRectangle<3>;
pub type GpuRect4 = GpuRectangle<4>;

pub type GpuColor = GpuRgba;
pub type GpuRgba = RgbaOf<Gpufloat>;
pub type GpuHsla = HslaOf<Gpufloat>;

pub type GpuMatrix<const ROW : usize, const COL : usize> = Matrix<Gpufloat, ROW, COL>;
pub type GpuSquareMatrix<const N : usize> = GpuMatrix<N, N>;
pub type GpuMat1 = GpuSquareMatrix<1>;
pub type GpuMat2 = GpuSquareMatrix<2>;
pub type GpuMat3 = GpuSquareMatrix<3>;
pub type GpuMat4 = GpuSquareMatrix<4>;