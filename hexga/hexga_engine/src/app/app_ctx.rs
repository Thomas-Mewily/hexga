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

impl AppContext
{
    fn scoped_flow<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }

    fn scoped_paused<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }

    fn scoped_update<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }

    fn scoped_draw<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }

    fn scoped_input<F,R>(&mut self, input: InputEvent, f: F) -> R where F: FnOnce(InputEvent) -> R { self.begin_scoped(); let r = f(input); self.end_scoped(); r }
}