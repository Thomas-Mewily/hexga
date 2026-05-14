use super::*;

pub struct GpuContext
{
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuContext
{
    pub fn instance(&self) -> impl Deref<Target = wgpu::Instance> { &self.instance }
    pub fn adapter(&self) -> impl Deref<Target = wgpu::Adapter> { &self.adapter }
    pub fn device(&self) -> impl Deref<Target = wgpu::Device> { &self.device }
    pub fn queue(&self) -> impl Deref<Target = wgpu::Queue> { &self.queue }

    /// Waits for the most recent submission.
    pub fn wait(&self) 
    {
        self.device().poll(wgpu::PollType::Wait);
    }
}
