use crate::*;
use super::convert::*;

#[derive(Clone)]
pub(crate) struct QuadState<S> where S : MainLoop
{
    pub(crate) state : S,
}

impl<S> miniquad::EventHandler for QuadState<S> where S : MainLoop
{
    fn update(&mut self) 
    {
        //Context.begin
        self.state.update();
    }

    fn draw(&mut self) {
        self.state.draw();
    }

    fn char_event(&mut self, character: char, keymods: miniquad::KeyMods, repeat: bool) 
    {
        self.state.handle_event(CharEvent {
            character,
            keymods: keymods.convert(),
            repeat,
        }.into());
    }

    fn files_dropped_event(&mut self) 
    {
        self.state.handle_event(WindowEvent::DropFile.into());
    }

    fn key_down_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, repeat: bool) {
        self.state.handle_event(KeyEvent
            {
                keycode: keycode.convert(),
                keymods: keymods.convert(),
                repeat: repeat,
                press: true,
            }.into());
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.state.handle_event(KeyEvent
            {
                keycode: keycode.convert(),
                keymods: keymods.convert(),
                repeat: false,
                press: false,
            }.into());
    }

    fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.state.handle_event(MouseButtonEvent
            {
                position: vec2(x as _, y as _),
                button: button.convert(),
                press: true,
            }.into());
    }

    fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.state.handle_event(MouseButtonEvent
            {
                position: vec2(x as _, y as _),
                button: button.convert(),
                press: false,
            }.into());
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) 
    {
        self.state.handle_event(MouseMove{ position : vec2(x as _, y as _) }.into());
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) 
    {
        self.state.handle_event(MouseEvent::Wheel(vec2(_x as _, _y as _)).into());
    }

    fn quit_requested_event(&mut self) 
    { 
        self.state.handle_event(WindowEvent::Quit.into()); 
    }

    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) 
    { 
        self.state.handle_event(MouseEvent::RawMove(vec2(dx as _, dy as _)).into()); 
    }

    fn resize_event(&mut self, _width: f32, _height: f32) {
        self.state.handle_event(WindowEvent::Resize(vec2(_width as _, _height as _)).into());
    }

    fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
        self.state.handle_event(TouchEvent
            {
                phase: phase.convert(),
                position: vec2(x as _, y as _),
                id,
            }.into());
    }

    fn window_minimized_event(&mut self) {
        self.state.handle_event(WindowEvent::Minimized.into());
    }

    fn window_restored_event(&mut self) {
        self.state.handle_event(WindowEvent::Restored.into());
    }
}