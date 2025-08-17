use crate::*;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct WindowParam
{
    /// Title of the window, defaults to an empty string.
    title: String,
    size : Option<Point2>,
    position : Option<Point2>,
}

impl WindowParam
{
    pub fn new() -> Self { ___() }

    pub fn with_title(mut self, title: impl Into<String>) -> Self { self.title = title.into(); self }
    pub fn with_size(mut self, size: impl Into<Option<Point2>>) -> Self { self.size = size.into(); self }
    pub fn with_position(mut self, position: impl Into<Option<Point2>>) -> Self { self.position = position.into(); self }
}
