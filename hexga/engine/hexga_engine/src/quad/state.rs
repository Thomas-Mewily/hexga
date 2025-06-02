use hexga_engine_base::context::Context;

use crate::*;
use super::convert::*;

#[derive(Clone)]
pub(crate) struct QuadState<S> where S : MainLoopWithContext
{
    pub(crate) state : S,
}

impl<S> miniquad::EventHandler for QuadState<S> where S : MainLoopWithContext
{
    fn update(&mut self)
    {
        //Context.begin
        self.state.update_with(Context.as_mut());
    }

    fn draw(&mut self)
    {
        self.state.draw_with(Context.as_mut());
    }

    fn char_event(&mut self, character: char, keymods: miniquad::KeyMods, repeat: bool)
    {
        self.state.handle_event_with(CharEvent {
            character,
            keymods: keymods.convert(),
            repeat,
        }.into(), Context.as_mut());
    }

    fn files_dropped_event(&mut self)
    {
        self.state.handle_event_with(WindowEvent::DropFile.into(), Context.as_mut());
    }

    fn key_down_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, repeat: bool) {
        self.state.handle_event_with(KeyEvent
            {
                keycode: keycode.convert(),
                keymods: keymods.convert(),
                repeat: repeat,
                press: true,
            }.into(), Context.as_mut());
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.state.handle_event_with(KeyEvent
            {
                keycode: keycode.convert(),
                keymods: keymods.convert(),
                repeat: false,
                press: false,
            }.into(), Context.as_mut());
    }

    fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.state.handle_event_with(MouseButtonEvent
            {
                position: vec2(x as _, y as _),
                button: button.convert(),
                press: true,
            }.into(), Context.as_mut());
    }

    fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.state.handle_event_with(MouseButtonEvent
            {
                position: vec2(x as _, y as _),
                button: button.convert(),
                press: false,
            }.into(), Context.as_mut());
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32)
    {
        self.state.handle_event_with(MouseMove{ position : vec2(x as _, y as _) }.into(), Context.as_mut());
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32)
    {
        self.state.handle_event_with(MouseEvent::Wheel(vec2(_x as _, _y as _)).into(), Context.as_mut());
    }

    fn quit_requested_event(&mut self)
    {
        self.state.handle_event_with(WindowEvent::Quit.into(), Context.as_mut());
    }

    fn raw_mouse_motion(&mut self, dx: f32, dy: f32)
    {
        self.state.handle_event_with(MouseEvent::RawMove(vec2(dx as _, dy as _)).into(), Context.as_mut());
    }

    fn resize_event(&mut self, _width: f32, _height: f32) {
        self.state.handle_event_with(WindowEvent::Resize(vec2(_width as _, _height as _)).into(), Context.as_mut());
    }

    fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
        self.state.handle_event_with(TouchEvent
            {
                phase: phase.convert(),
                position: vec2(x as _, y as _),
                id : TouchID::new(id),
            }.into(), Context.as_mut());
    }

    fn window_minimized_event(&mut self) {
        self.state.handle_event_with(WindowEvent::Minimized.into(), Context.as_mut());
    }

    fn window_restored_event(&mut self) {
        self.state.handle_event_with(WindowEvent::Restored.into(), Context.as_mut());
    }
}