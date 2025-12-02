pub type GpuResult<T=()> = Result<T,GpuError>;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum GpuError
{
    GpuAlreadyInit,
    Adapter(wgpu::RequestAdapterError),
    Device(wgpu::RequestDeviceError),
    BufferRead(wgpu::BufferAsyncError),
    Unknow,
}
impl From<wgpu::RequestAdapterError> for GpuError
{
    fn from(value: wgpu::RequestAdapterError) -> Self { GpuError::Adapter(value) }
}
impl From<wgpu::RequestDeviceError> for GpuError
{
    fn from(value: wgpu::RequestDeviceError) -> Self { GpuError::Device(value) }
}
impl From<wgpu::BufferAsyncError> for GpuError
{
    fn from(value: wgpu::BufferAsyncError) -> Self { GpuError::BufferRead(value) }
}
impl From<()> for GpuError
{
    fn from(value: ()) -> Self { GpuError::Unknow }
}