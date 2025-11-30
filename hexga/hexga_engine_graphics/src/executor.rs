use super::*;

pub trait HasGpuExecutor<'a> : Has<'a, ExecutorRef<'a>>
{
    fn executor(&'a self) -> ExecutorRef<'a> { self.retrieve() }
}
impl<'a,T> HasGpuExecutor<'a> for T where T: Has<'a, ExecutorRef<'a>> {}

impl<'a> Has<'a,ExecutorRef<'a>> for Executor
{
    fn retrieve(&'a self) -> ExecutorRef<'a> {
        self.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Executor
{
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
impl Executor
{
    pub fn as_ref(&self) -> ExecutorRef<'_>
    {
        ExecutorRef{ device: &self.device, queue: &self.queue }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutorRef<'a>
{
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}
impl<'a> From<&'a Executor> for ExecutorRef<'a>
{
    fn from(value: &'a Executor) -> Self
    {
        let Executor { device, queue } = &value;
        Self { device, queue }
    }
}
impl<'a> From<ExecutorRef<'a>> for Executor
{
    fn from(value: ExecutorRef<'a>) -> Self
    {
        let ExecutorRef { device, queue } = value;
        Self { device: device.clone(), queue: queue.clone() }
    }
}


