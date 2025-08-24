use super::*;




pub struct AppParam
{
    pub window : Option<WindowParam>,
}
impl Default for AppParam
{
    fn default() -> Self {
        Self { window: Some(WindowParam::new()) }
    }
}

impl AppParam
{
    pub fn new() -> Self { ___() }

    fn with_window_mut_or_create(mut self, f: impl for<'a> FnOnce(WindowParam) -> WindowParam) -> Self
    {
        self.window.get_or_insert_with(WindowParam::new);
        self.window = Some(f(self.window.unwrap()));
        self
    }

    pub fn with_window(mut self, window : impl Into<Option<WindowParam>>) -> Self  { self.window = window.into(); self }
}

/* 
impl IWindowParam for AppParam
{
    fn title(&self) -> &str {
        self.window.as_ref().map(|w| w.title()).unwrap_or_default()
    }

    fn with_title(self, title: impl Into<String>) -> Self {
        self.with_window_mut_or_create(|w| w.with_title(title))
    }
}*/