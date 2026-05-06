use super::*;


#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppParam
{
    pub window : WindowParam,
    pub gpu: GpuParam,
    pub update : TimeStrategy,
}

pub trait WithUpdateStrategy
{
    fn with_update(self, strat: TimeStrategy) -> Self;   
}

impl WithUpdateStrategy for AppParam
{
    fn with_update(mut self, strat: TimeStrategy) -> Self {
        self.update = strat; self
    }
}

impl GetPosition<int,2> for AppParam
{
    fn pos(&self) -> Vector<int, 2> {
        self.window.pos()
    }
}
impl SetPosition<int,2> for AppParam
{
    fn set_pos(&mut self, pos: Vector<int, 2>) -> &mut Self {
        self.window.set_pos(pos); self
    }
}
impl GetSize<int,2> for AppParam
{
    fn size(&self) -> Vector<int, 2> {
        self.window.size()
    }
}
impl SetSize<int,2> for AppParam
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self {
        self.window.set_size(size); self
    }
}

impl WindowAttribute for AppParam
{
    fn title(&self) -> &str {
        self.window.title()
    }

    fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.window.set_title(title);
        self
    }

    fn level(&self) -> WindowLevel {
        self.window.level()
    }

    fn set_level(&mut self, level: WindowLevel) -> &mut Self {
        self.window.set_level(level);
        self
    }

    fn is_resizable(&self) -> bool {
        self.window.is_resizable()
    }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.window.set_resizable(resizable);
        self
    }

    fn buttons(&mut self) -> window::WindowButtonFlags {
        self.window.buttons()
    }

    fn set_buttons(&mut self, buttons: window::WindowButtonFlags) -> &mut Self {
        self.window.set_buttons(buttons);
        self
    }

    fn maximised(&self) -> bool {
        self.window.maximised()
    }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self {
        self.window.set_maximized(maximized);
        self
    }

    fn is_visible(&self) -> bool {
        self.window.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.window.set_visible(visible);
        self
    }

    fn is_transparent(&self) -> bool {
        self.window.is_transparent()
    }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self {
        self.window.set_transparent(transparent);
        self
    }

    fn have_blur(&self) -> bool {
        self.window.have_blur()
    }

    fn set_blur(&mut self, blur: bool) -> &mut Self {
        self.window.set_blur(blur);
        self
    }

    fn have_decoration(&self) -> bool {
        self.window.have_decoration()
    }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self {
        self.window.set_decoration(decorations);
        self
    }

    fn is_content_protected(&self) -> bool {
        self.window.is_content_protected()
    }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self {
        self.window.set_content_protected(protected);
        self
    }

    fn is_active(&self) -> bool {
        self.window.is_active()
    }

    fn set_active(&mut self, active: bool) -> &mut Self {
        self.window.set_active(active);
        self
    }
}
