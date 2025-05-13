use hexga_core::prelude::*;
use hexga_math::prelude::*;
use miniquad::EventHandler;

use std::fmt::Debug;

mod event;
pub use event::*;

mod conf;
pub use conf::*;

#[derive(Debug)]
struct PumpEvent<T> where T : EventLoop
{
    state  : T,
    events : Vec<Event>
}

impl<T> PumpEvent<T> where T : EventLoop
{
    pub fn new(state  : T) -> Self { Self { state, events: ___() }}
    pub fn push_event(&mut self, event : impl Into<Event>) { self.events.push(event.into()); }
}

impl<T> EventHandler for PumpEvent<T> where T : EventLoop
{
    fn update(&mut self) 
    {
        for e in self.events.drain(..)
        {
            if !self.state.handle_event(&e)
            {
            }
        }
        self.state.update();
    }

    fn draw(&mut self) {
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

    fn files_dropped_event(&mut self) {
        todo!()
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

pub trait EventLoop
{
    fn update(&mut self);
    fn draw(&mut self);

    fn handle_event(&mut self, event : &Event) -> bool;
}

