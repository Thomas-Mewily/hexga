use super::*;


#[derive(Debug)]
pub struct Ctx
{
    pub(crate) proxy : EventLoopProxy
}

impl Ctx 
{
    pub(crate) fn new(proxy: EventLoopProxy) -> Self
    {
        Self { proxy }
    }
}


