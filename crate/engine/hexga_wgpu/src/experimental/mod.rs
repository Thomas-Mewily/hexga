use super::*;
pub use wgpu;

pub static GPU: SingletonOnce<GpuContext> = SingletonOnce::new();

pub struct GpuContext
{
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub fn instance() -> impl Deref<Target = wgpu::Instance> { &GPU.get().instance }
pub fn adapter() -> impl Deref<Target = wgpu::Adapter> { &GPU.get().adapter }
pub fn device() -> impl Deref<Target = wgpu::Device> { &GPU.get().device }
pub fn queue() -> impl Deref<Target = wgpu::Queue> { &GPU.get().queue }

pub mod prelude
{
    pub use super::{instance, adapter, device, queue};
}