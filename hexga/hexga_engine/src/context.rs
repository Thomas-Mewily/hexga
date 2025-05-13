use crate::*;

static mut CONTEXT : Option<Context> = None;

#[allow(static_mut_refs)]
pub(crate) fn ctx_ref() -> &'static Context  { unsafe { CONTEXT.as_ref().unwrap() } }
#[allow(static_mut_refs)]
pub(crate) fn ctx_mut() -> &'static mut Context  { unsafe { CONTEXT.as_mut().unwrap() } }


pub(crate) struct Ctx;
impl Deref for Ctx
{
    type Target=Context;
    #[allow(static_mut_refs)]
    fn deref(&self) -> &Self::Target { ctx_ref() }
}
impl DerefMut for Ctx
{
    #[allow(static_mut_refs)]
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut() }
}


pub(crate) struct Context
{
    pub(crate) state  : Box<dyn EventLoop>,
    pub(crate) events : Vec<Event>,
    pub(crate) render : Box<dyn miniquad::RenderingBackend>,
    pub(crate) pen    : ContextPen,
}

impl Drop for Context
{
    fn drop(&mut self) 
    {
        #[allow(static_mut_refs)]
        unsafe 
        {
            assert!(CONTEXT.is_some());
            CONTEXT = None;
        }
    }
}


impl Context
{
    pub(crate) fn new(state : Box<dyn EventLoop>) -> bool 
    { 
        #[allow(static_mut_refs)]
        unsafe 
        {
            assert!(CONTEXT.is_none(), "Can't init twice a singleton");
            if  CONTEXT.is_some() { return false; }
            CONTEXT = Some(Self { state, events: ___(), render: miniquad::window::new_rendering_backend(), pen : ContextPen::new() });
            true
        }
    }
    pub(crate) fn push_event(&mut self, event : impl Into<Event>) { self.events.push(event.into()); }
}

impl EventHandler for Ctx
{
    fn update(&mut self) 
    {
        let Context { state, events, .. } = &mut *Ctx;
        
        for e in events.drain(..)
        {
            if !state.handle_event(&e)
            {
            }
        }
        state.update();
    }

    fn draw(&mut self) 
    {
        self.state.draw();
    }

    fn char_event(&mut self, character: char, keymods: miniquad::KeyMods, repeat: bool) {
        self.push_event(CharEvent { character, keymods : keymods.into(), repeat });
    }

    fn key_down_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, repeat: bool) {
        self.push_event(KeyEvent { keycode: keycode.into(), keymods: keymods.into(), repeat, press : true });
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.push_event(KeyEvent { keycode: keycode.into(), keymods: keymods.into(), repeat : false, press : false });
    }

    fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.push_event(MouseButtonEvent{ position: vec2(x as _, y as _), button: button.into(), press: true });
    }

    fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.push_event(MouseButtonEvent{ position: vec2(x as _, y as _), button: button.into(), press: true });
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.push_event(MouseEvent::Move(vec2(x as _, y as _)));
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.push_event(MouseEvent::Wheel(vec2(x as _, y as _)));
    }

    fn quit_requested_event(&mut self) {
        self.push_event(WindowEvent::Quit)
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.push_event(WindowEvent::Resize(vec2(width as _, height as _)));
    }

    fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
        self.push_event(TouchEvent{ phase: phase.into(), id: id as _, position: vec2(x as _, y as _) });
    }

    fn files_dropped_event(&mut self) 
    {
        let idx = miniquad::window::dropped_file_count();
        let path = miniquad::window::dropped_file_path(idx).unwrap_or_default();
        let bytes = miniquad::window::dropped_file_bytes(idx).unwrap_or_default();
        self.push_event(DropFileEvent{ path, bytes });
    }

    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        self.push_event(MouseEvent::RawMove(vec2(dx as _, dy as _)));
    }

    fn window_minimized_event(&mut self) {
        self.push_event(WindowEvent::Minimized);
    }

    fn window_restored_event(&mut self) {
        self.push_event(WindowEvent::Restored);
    }
}

//pub type EventResult<T=Event> = Result<T,()>;


