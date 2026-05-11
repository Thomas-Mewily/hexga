use hexga_event_loop::window::{UserAttentionType, WindowButtonFlags, WindowLevel};
use super::*;

pub struct CurrentWindow;


impl Windowable for CurrentWindow
{
    fn request_draw(&mut self)
    {
        WINDOW.get_mut().request_draw();
    }

    fn request_user_attention(&mut self, request_type : impl Into<Option<UserAttentionType>>)
    {
        WINDOW.get_mut().request_user_attention(request_type);
    }
    
    fn winit_window(&self) -> hexga_event_loop::window::experimental::WinitWindowShared {
        WINDOW.get_mut().winit_window()
    }
    
    fn current_monitor(&self) -> Option<hexga_event_loop::monitor::Monitor> {
        WINDOW.get_mut().current_monitor()
    }
    
    fn primary_monitor(&self) -> Option<hexga_event_loop::monitor::Monitor> {
        WINDOW.get_mut().primary_monitor()
    }
    
    fn available_monitors(&self) -> impl Iterator<Item=hexga_event_loop::monitor::Monitor> {
        let v = WINDOW.get_mut().available_monitors().to_vec();
        v.into_iter()
    }
}


impl GetPosition<int,2> for CurrentWindow
{
    fn pos(&self) -> Vector<int, 2> 
    {
        WINDOW.get_mut().pos()
    }
}
impl SetPosition<int,2> for CurrentWindow
{
    fn set_pos(&mut self, pos: Vector<int, 2>) -> &mut Self {
        WINDOW.get_mut().set_pos(pos);
        self
    }
}
impl GetSize<int,2> for CurrentWindow
{
    fn size(&self) -> Vector<int, 2> {
        WINDOW.get_mut().size()
    }
}
impl SetSize<int,2> for CurrentWindow
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self {
        WINDOW.get_mut().set_size(size);
        self
    }
}

impl WindowAttribute for CurrentWindow
{
    fn title(&self) -> String
    {
        WINDOW.get_mut().title()
    }

    fn set_title(&mut self, title: String) -> &mut Self {
        WINDOW.get_mut().set_title(title);
        self
    }

    fn level(&self) -> WindowLevel {
        WINDOW.get_mut().level()
    }

    fn set_level(&mut self, level: WindowLevel) -> &mut Self {
        WINDOW.get_mut().set_level(level);
        self
    }

    fn is_resizable(&self) -> bool {
        WINDOW.get_mut().is_resizable()
    }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        WINDOW.get_mut().set_resizable(resizable);
        self
    }

    fn buttons(&mut self) -> WindowButtonFlags {
        WINDOW.get_mut().buttons()
    }

    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self {
        WINDOW.get_mut().set_buttons(buttons);
        self
    }

    fn maximised(&self) -> bool {
        WINDOW.get_mut().maximised()
    }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self {
        WINDOW.get_mut().set_maximized(maximized);
        self
    }

    fn is_visible(&self) -> bool {
        WINDOW.get_mut().is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> &mut Self {
        WINDOW.get_mut().set_visible(visible);
        self
    }

    fn is_transparent(&self) -> bool {
        WINDOW.get_mut().is_transparent()
    }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self {
        WINDOW.get_mut().set_transparent(transparent);
        self
    }

    fn have_blur(&self) -> bool {
        WINDOW.get_mut().have_blur()
    }

    fn set_blur(&mut self, blur: bool) -> &mut Self {
        WINDOW.get_mut().set_blur(blur);
        self
    }

    fn have_decoration(&self) -> bool {
        WINDOW.get_mut().have_decoration()
    }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self {
        WINDOW.get_mut().set_decoration(decorations);
        self
    }

    fn is_content_protected(&self) -> bool {
        WINDOW.get_mut().is_content_protected()
    }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self {
        WINDOW.get_mut().set_content_protected(protected);
        self
    }

    fn is_active(&self) -> bool {
        WINDOW.get_mut().is_active()
    }

    fn set_active(&mut self, active: bool) -> &mut Self {
        WINDOW.get_mut().set_active(active);
        self
    }
}