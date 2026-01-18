use super::*;


pub(crate) type GpuVertexFormat = wgpu::VertexFormat;
pub(crate) type GpuIndexFormat = wgpu::IndexFormat;


pub(crate) trait IGpuIndexFormat
{
    const GPU_INDEX_FORMAT : GpuIndexFormat;
}
impl IGpuIndexFormat for u16 { const GPU_INDEX_FORMAT : GpuIndexFormat = GpuIndexFormat::Uint16; }
impl IGpuIndexFormat for u32 { const GPU_INDEX_FORMAT : GpuIndexFormat = GpuIndexFormat::Uint32; }

pub(crate) trait IGpuVertexFormat
{
    const GPU_VERTEX_FORMAT : GpuVertexFormat;
}


impl IGpuVertexFormat for u8  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint8;  }
impl IGpuVertexFormat for u16 { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint16; }
impl IGpuVertexFormat for u32 { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint32; }

impl IGpuVertexFormat for i8  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint8;  }
impl IGpuVertexFormat for i16 { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint16; }
impl IGpuVertexFormat for i32 { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint32; }

impl IGpuVertexFormat for f32 { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float32; }
impl IGpuVertexFormat for f64 { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float64; }


impl IGpuVertexFormat for [u8;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint8;   }
impl IGpuVertexFormat for [u8;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint8x2; }
impl IGpuVertexFormat for [u8;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint8x4; }

impl IGpuVertexFormat for [u16;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint16;   }
impl IGpuVertexFormat for [u16;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint16x2; }
impl IGpuVertexFormat for [u16;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint16x4; }

impl IGpuVertexFormat for [u32;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint32;   }
impl IGpuVertexFormat for [u32;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint32x2; }
impl IGpuVertexFormat for [u32;3]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint32x3; }
impl IGpuVertexFormat for [u32;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Uint32x4; }



impl IGpuVertexFormat for [i8;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint8;   }
impl IGpuVertexFormat for [i8;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint8x2; }
impl IGpuVertexFormat for [i8;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint8x4; }

impl IGpuVertexFormat for [i16;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint16;   }
impl IGpuVertexFormat for [i16;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint16x2; }
impl IGpuVertexFormat for [i16;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint16x4; }

impl IGpuVertexFormat for [i32;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint32;   }
impl IGpuVertexFormat for [i32;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint32x2; }
impl IGpuVertexFormat for [i32;3]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint32x3; }
impl IGpuVertexFormat for [i32;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Sint32x4; }


impl IGpuVertexFormat for [f32;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float32;   }
impl IGpuVertexFormat for [f32;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float32x2; }
impl IGpuVertexFormat for [f32;3]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float32x3; }
impl IGpuVertexFormat for [f32;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float32x4; }

impl IGpuVertexFormat for [f64;1]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float64;   }
impl IGpuVertexFormat for [f64;2]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float64x2; }
impl IGpuVertexFormat for [f64;3]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float64x3; }
impl IGpuVertexFormat for [f64;4]  { const GPU_VERTEX_FORMAT : wgpu::VertexFormat = wgpu::VertexFormat::Float64x4; }

impl<T,const N:usize> IGpuVertexFormat for Vector<T,N> where [T;N]: IGpuVertexFormat
{
    const GPU_VERTEX_FORMAT : GpuVertexFormat = <[T;N]>::GPU_VERTEX_FORMAT;
}
impl<T> IGpuVertexFormat for RgbaOf<T> where [T;4]: IGpuVertexFormat
{
    const GPU_VERTEX_FORMAT : GpuVertexFormat = <[T;4]>::GPU_VERTEX_FORMAT;
}
impl<T> IGpuVertexFormat for HslaOf<T> where [T;4]: IGpuVertexFormat
{
    const GPU_VERTEX_FORMAT : GpuVertexFormat = <[T;4]>::GPU_VERTEX_FORMAT;
}

impl<T> IGpuVertexFormat for TimeOf<T> where T: IGpuVertexFormat
{
    const GPU_VERTEX_FORMAT : GpuVertexFormat = <T>::GPU_VERTEX_FORMAT;
}
impl<T> IGpuVertexFormat for AngleOf<T> where T: IGpuVertexFormat
{
    const GPU_VERTEX_FORMAT : GpuVertexFormat = <T>::GPU_VERTEX_FORMAT;
}
