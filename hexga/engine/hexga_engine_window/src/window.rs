//! mainly inspired by miniquad

use hexga::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum CursorIcon 
{
    Default,
    Help,
    Pointer,
    Wait,
    Crosshair,
    Text,
    Move,
    NotAllowed,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
}

pub trait LoopWindow
{
    fn update(&mut self);
    fn draw  (&mut self);

    fn get_clipboard(&mut self) -> Option<String>;
    fn set_clipboard(&mut self, text : &str);

    fn dpi_scale(&mut self) -> f32;
    fn is_dpi_hight(&mut self) -> bool;

    /// Quit the window
    fn quit(&mut self);
    /// Ask the user for a quitting confirmation and quit
    fn requst_quit(&mut self);

    fn get_position(&mut self) -> Point2;
    fn set_position(&mut self, pos : Point2); 

    /// Current window size in pixel (taking dpi in account)
    fn get_size(&mut self) -> Point2;
    fn set_size(&mut self, size : Point2);


    fn set_fullscreen(&mut self, fullscreen: bool);


    fn show_keyboard(show: bool);

    fn show_mouse(shown: bool);
    fn grab_mouse(&mut self, grab: bool);
    fn set_mouse_cursor(cursor_icon: CursorIcon);
}

impl LoopWindow for ()
{
    fn draw  (&mut self) {}
    fn update(&mut self) {}

    fn get_clipboard(&mut self) -> Option<String> { None }
    fn set_clipboard(&mut self, _text : &str) {}

    fn dpi_scale(&mut self) -> f32 { 1.0 }
    fn is_dpi_hight(&mut self) -> bool { false }

    fn quit(&mut self) {}
    fn requst_quit(&mut self) {}

    fn get_position(&mut self) -> Point2 { Point2::ZERO }
    fn set_position(&mut self, _pos : Point2) {}

    fn get_size(&mut self) -> Point2 { Point2::ONE }
    fn set_size(&mut self, _size : Point2) {}

    fn set_fullscreen(&mut self, _fullscreen: bool) {}
    fn show_keyboard(_show: bool) {}

    fn show_mouse(_shown: bool) {}
    fn grab_mouse(&mut self, _grab: bool) {}
    fn set_mouse_cursor(_cursor_icon: CursorIcon) {}
}