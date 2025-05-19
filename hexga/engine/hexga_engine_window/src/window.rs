//! mainly inspired by miniquad

pub trait LoopWindow
{
    fn clipboard_get(&mut self) -> Option<String>;
    fn clipboard_set(&mut self, text : &str);
    fn dpi_scale() -> f32;
    fn window_position() ->
}