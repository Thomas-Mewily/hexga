use super::*;



#[derive(Default)]
pub struct AppParam
{
    title: String,
}



#[derive(Debug)]
pub struct Ctx
{
    pub(crate) proxy : EventLoopProxy,
    pub(crate) keyboard: Keyboard,
    pub(crate) gpu: Option<Gpu>,
}

impl HasRef<Keyboard> for Ctx  { fn retrive(&self) -> &Keyboard { &self.keyboard } }
impl HasMut<Keyboard> for Ctx  { fn retrive_mut(&mut self) -> &mut Keyboard { &mut self.keyboard } }

impl Ctx 
{
    pub(crate) fn new(proxy: EventLoopProxy) -> Self
    {
        Self { proxy, keyboard: ___(), gpu: ___() }
    }
}
