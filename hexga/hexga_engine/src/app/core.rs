use super::*;

#[derive(Debug)]
pub struct AppCore
{
    /*
    pub(crate) gpu: Option<AppGpu>,
    */
    pub(crate) already_init: bool,

    pub(crate) clipboard: AppClipboard,
    pub(crate) pen:       Option<AppPen>,
    pub(crate) input:     AppInput,
    pub(crate) window:    AppWindow,
    pub(crate) perf:      AppPerf,
    pub(crate) param:     AppParam,
    pub(crate) proxy:     Option<EventLoopProxy>,
}
impl AppCore
{
    pub fn clipboard(&mut self) -> &mut AppClipboard { &mut self.clipboard }
    pub fn input(&mut self) -> &mut AppInput { &mut self.input }
    pub fn window(&mut self) -> &mut AppWindow { &mut self.window }
    pub fn perf(&mut self) -> &mut AppPerf { &mut self.perf }

    pub fn param(&self) -> &AppParam { &self.param }
    //pub fn param_mut(&mut self) -> &mut AppParam { &mut self.param }


    pub fn exit(&mut self)
    {
        //let _ = self.send_event(AppInternalEvent::Exit);
        self.window.destroy();
        self.proxy = None;
    }

    pub(crate) fn send_event(&mut self, event: AppInternalEvent)
    {
        let _ = self.try_send_event(event);
    }

    pub(crate) fn try_send_event(&mut self, event: AppInternalEvent) -> Result<(), ()>
    {
        match &self.proxy
        {
            Some(v) => v.send_event(event).map_err(|_|()),
            None => Err(()),
        }
    }
}

impl AppCore
{
    pub(crate) fn new() -> Self
    {
        Self
        {
            param: ___(),
            input: AppInput::new(),
            window: AppWindow::new(),
            clipboard: AppClipboard::new(),
            perf: AppPerf::new(),
            pen: None,
            proxy: None,
            already_init: false,
        }
    }

    pub(crate) fn init(&mut self, param: AppParam, proxy: EventLoopProxy)
    {
        assert_eq!(self.already_init, false);
        self.param = param;
        self.proxy = Some(proxy);
        self.already_init = true;
    }
}

impl ScopedFlow for AppCore
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.perf.begin_flow(flow);
        self.input.begin_flow(flow);
        self.pen.begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.input.end_flow(flow);
        self.perf.end_flow(flow);
        self.pen.end_flow(flow);
    }
}

singleton_single_thread!(pub App,AppCore,CONTEXT_APP,|| AppCore::new());

/*
pub struct App;
impl Deref for App
{
    type Target=AppCore;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        CONTEXT_APP.get().as_ref().expect("App was not init")
    }
}
impl DerefMut for App
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        CONTEXT_APP.instance_mut().as_mut().expect("App was not init")
    }
}
impl App
{
    pub fn is_init() -> bool { CONTEXT_APP.instance().is_some() }
    pub fn is_not_init() -> bool { !Self::is_init() }
    pub(crate) fn replace(new: Option<AppCore>) -> Option<AppCore>
    {
        std::mem::replace(CONTEXT_APP.instance_mut(), new)
    }
    pub(crate) fn destroy()
    {
        Self::replace(None);
    }
}
    */

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub struct AppParam
{
    pub title: String,
}

impl AppParam
{
    pub fn new() -> Self { ___() }
    pub fn with_title(self, title: impl Into<String>) -> Self { Self { title: title.into(), ..self } }
}