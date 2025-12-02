use super::*;

pub type AppResult<T=()> = Result<T,AppError>;

#[non_exhaustive]
#[derive(Debug)]
pub enum AppError
{
    AlreadyInit,
    EventLoop(winit::error::EventLoopError),
    Panics(Box<dyn Any + Send>),
}
impl From<winit::error::EventLoopError> for AppError { fn from(value: winit::error::EventLoopError) -> Self { AppError::EventLoop(value) } }
impl From<Box<dyn Any + Send>> for AppError { fn from(value: Box<dyn Any + Send>) -> Self { AppError::Panics(value) } }