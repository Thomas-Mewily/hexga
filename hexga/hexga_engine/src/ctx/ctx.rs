use super::*;


#[derive(Default)]
pub struct AppParam
{

}

pub trait AppCtx
{
    type Param;
    fn new(param: Self::Param) -> Self where Self: Sized;
}

#[derive(Debug)]
pub struct Ctx
{
    pub(crate) proxy : EventLoopProxy
}
impl AppCtx for Ctx
{
    type Param=EventLoopProxy;
    
    fn new(proxy: Self::Param) -> Self where Self: Sized {
        Self { proxy }
    }
}

impl Ctx 
{
    pub(crate) fn new(proxy: EventLoopProxy) -> Self
    {
        Self { proxy }
    }
}


