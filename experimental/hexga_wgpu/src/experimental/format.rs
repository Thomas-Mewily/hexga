use super::*;

pub type WgpuVertexFormat = wgpu::VertexFormat;
pub type WgpuIndexFormat = wgpu::IndexFormat;

pub trait WgpuIndex
{
    const GPU_INDEX_FORMAT: WgpuIndexFormat;
}
impl WgpuIndex for u16
{
    const GPU_INDEX_FORMAT: WgpuIndexFormat = WgpuIndexFormat::Uint16;
}
impl WgpuIndex for u32
{
    const GPU_INDEX_FORMAT: WgpuIndexFormat = WgpuIndexFormat::Uint32;
}

pub trait WgpuVertex
{
    const GPU_VERTEX_FORMAT: WgpuVertexFormat;
}

impl WgpuVertex for u8
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint8;
}
impl WgpuVertex for u16
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint16;
}
impl WgpuVertex for u32
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint32;
}

impl WgpuVertex for i8
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint8;
}
impl WgpuVertex for i16
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint16;
}
impl WgpuVertex for i32
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint32;
}

impl WgpuVertex for f32
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32;
}
impl WgpuVertex for f64
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float64;
}

impl WgpuVertex for [u8; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint8;
}
impl WgpuVertex for [u8; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint8x2;
}
impl WgpuVertex for [u8; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint8x4;
}

impl WgpuVertex for [u16; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint16;
}
impl WgpuVertex for [u16; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint16x2;
}
impl WgpuVertex for [u16; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint16x4;
}

impl WgpuVertex for [u32; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint32;
}
impl WgpuVertex for [u32; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint32x2;
}
impl WgpuVertex for [u32; 3]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint32x3;
}
impl WgpuVertex for [u32; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Uint32x4;
}

impl WgpuVertex for [i8; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint8;
}
impl WgpuVertex for [i8; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint8x2;
}
impl WgpuVertex for [i8; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint8x4;
}

impl WgpuVertex for [i16; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint16;
}
impl WgpuVertex for [i16; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint16x2;
}
impl WgpuVertex for [i16; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint16x4;
}

impl WgpuVertex for [i32; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint32;
}
impl WgpuVertex for [i32; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint32x2;
}
impl WgpuVertex for [i32; 3]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint32x3;
}
impl WgpuVertex for [i32; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Sint32x4;
}

impl WgpuVertex for [f32; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32;
}
impl WgpuVertex for [f32; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x2;
}
impl WgpuVertex for [f32; 3]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
}
impl WgpuVertex for [f32; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;
}

impl WgpuVertex for [f64; 1]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float64;
}
impl WgpuVertex for [f64; 2]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float64x2;
}
impl WgpuVertex for [f64; 3]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float64x3;
}
impl WgpuVertex for [f64; 4]
{
    const GPU_VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float64x4;
}

impl<T, const N: usize> WgpuVertex for Vector<T, N>
where
    [T; N]: WgpuVertex,
{
    const GPU_VERTEX_FORMAT: WgpuVertexFormat = <[T; N]>::GPU_VERTEX_FORMAT;
}
impl<T> WgpuVertex for RgbaOf<T>
where
    [T; 4]: WgpuVertex,
{
    const GPU_VERTEX_FORMAT: WgpuVertexFormat = <[T; 4]>::GPU_VERTEX_FORMAT;
}
impl<T> WgpuVertex for HslaOf<T>
where
    [T; 4]: WgpuVertex,
{
    const GPU_VERTEX_FORMAT: WgpuVertexFormat = <[T; 4]>::GPU_VERTEX_FORMAT;
}

impl<T> WgpuVertex for TimeOf<T>
where
    T: WgpuVertex,
{
    const GPU_VERTEX_FORMAT: WgpuVertexFormat = <T>::GPU_VERTEX_FORMAT;
}
impl<T> WgpuVertex for AngleOf<T>
where
    T: WgpuVertex,
{
    const GPU_VERTEX_FORMAT: WgpuVertexFormat = <T>::GPU_VERTEX_FORMAT;
}
