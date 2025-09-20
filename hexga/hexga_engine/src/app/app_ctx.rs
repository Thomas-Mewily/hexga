use super::*;



#[derive(Default, Debug)]
pub struct AppParam
{
    title: String,
}


#[derive(Debug)]
pub struct AppContext
{
    pub(crate) windows: Option<WinitContext>,
    pub(crate) gpu: Option<GpuContext>,

    pub(crate) keyboard: Keyboard,
    pub(crate) param: AppParam,
}


impl AppContext 
{
    pub(crate) fn new(param: AppParam) -> Self { Self { param, windows: ___(), keyboard: ___(), gpu: ___() } }
}


impl ScopedMessage for AppContext
{
    fn begin_resumed(&mut self) {
        
    }
    fn end_resumed(&mut self) {
        
    }


    fn begin_paused(&mut self) {
        
    }
    fn end_paused(&mut self) {
        
    }

    fn begin_update(&mut self) {
        
    }
    fn end_update(&mut self) {
        
    }


    fn begin_draw(&mut self) {
        
    }
    fn end_draw(&mut self) {
        
    }


    fn begin_event(&mut self, ev: &AppEvent) {
        
    }
    fn end_event(&mut self) {
        
    }
}