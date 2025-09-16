use super::*;


#[derive(Debug)]
pub enum CtxEvent
{
    Gpu(Result<Gpu,String>),
}
