use super::*;
use hexga_event_loop::event_loop::EventLoopParam;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppParam
{
    pub window: WindowParam,
    pub gpu: GpuParam,
    pub event_loop: EventLoopParam, //pub update : TimeStrategy,
}

impl WithEventLoopParam for AppParam
{
    fn control_flow(&self) -> hexga_event_loop::event_loop::EventLoopControlFlow
    {
        self.event_loop.control_flow()
    }

    fn with_control_flow(
        mut self,
        control_flow: hexga_event_loop::event_loop::EventLoopControlFlow,
    ) -> Self
    {
        self.event_loop = self.event_loop.with_control_flow(control_flow);
        self
    }
}

impl WithEventLoopShortcut for AppParam
{
    fn exit_shortcut(&self) -> Option<KeyShortcut> { self.event_loop.exit_shortcut() }

    fn with_exit_shortcut(mut self, exit: Option<KeyShortcut>) -> Self
    {
        self.event_loop = self.event_loop.with_exit_shortcut(exit);
        self
    }

    fn copy_shortcut(&self) -> Option<KeyShortcut> { self.event_loop.copy_shortcut() }

    fn with_copy_shortcut(mut self, copy: Option<KeyShortcut>) -> Self
    {
        self.event_loop = self.event_loop.with_copy_shortcut(copy);
        self
    }

    fn paste_shortcut(&self) -> Option<KeyShortcut> { self.event_loop.paste_shortcut() }

    fn with_paste_shortcut(mut self, paste: Option<KeyShortcut>) -> Self
    {
        self.event_loop = self.event_loop.with_paste_shortcut(paste);
        self
    }

    fn cut_shortcut(&self) -> Option<KeyShortcut> { self.event_loop.cut_shortcut() }

    fn with_cut_shortcut(mut self, cut: Option<KeyShortcut>) -> Self
    {
        self.event_loop = self.event_loop.with_cut_shortcut(cut);
        self
    }
}


impl WindowAttribute for AppParam
{
    fn title(&self) -> String { self.window.title() }

    fn set_title(&mut self, title: String) -> &mut Self
    {
        self.window.set_title(title);
        self
    }

    fn level(&self) -> hexga_event_loop::window::WindowLevel { self.window.level() }

    fn set_level(&mut self, level: hexga_event_loop::window::WindowLevel) -> &mut Self
    {
        self.window.set_level(level);
        self
    }

    fn is_resizable(&self) -> bool { self.window.is_resizable() }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self
    {
        self.window.set_resizable(resizable);
        self
    }

    fn buttons(&mut self) -> hexga_event_loop::window::WindowButtonFlags { self.window.buttons() }

    fn set_buttons(&mut self, buttons: hexga_event_loop::window::WindowButtonFlags) -> &mut Self
    {
        self.window.set_buttons(buttons);
        self
    }

    fn is_maximised(&self) -> bool { self.window.is_maximised() }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self
    {
        self.window.set_maximized(maximized);
        self
    }

    fn is_visible(&self) -> bool { self.window.is_visible() }

    fn set_visible(&mut self, visible: bool) -> &mut Self
    {
        self.window.set_visible(visible);
        self
    }

    fn is_transparent(&self) -> bool { self.window.is_transparent() }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self
    {
        self.window.set_transparent(transparent);
        self
    }

    fn have_blur(&self) -> bool { self.window.have_blur() }

    fn set_blur(&mut self, blur: bool) -> &mut Self
    {
        self.window.set_blur(blur);
        self
    }

    fn have_decoration(&self) -> bool { self.window.have_decoration() }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self
    {
        self.window.set_decoration(decorations);
        self
    }

    fn is_content_protected(&self) -> bool { self.window.is_content_protected() }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self
    {
        self.window.set_content_protected(protected);
        self
    }

    fn is_active(&self) -> bool { self.window.is_active() }

    fn set_active(&mut self, active: bool) -> &mut Self
    {
        self.window.set_active(active);
        self
    }

    fn theme(&self) -> Option<hexga_event_loop::window::Theme> { self.window.theme() }

    fn set_theme(&mut self, theme: Option<hexga_event_loop::window::Theme>) -> &mut Self
    {
        self.window.set_theme(theme);
        self
    }

    fn icon(&self) -> Option<Image> { self.window.icon() }

    fn set_icon(&mut self, icon: impl Into<Option<Image>>) -> &mut Self
    {
        self.window.set_icon(icon);
        self
    }
}
