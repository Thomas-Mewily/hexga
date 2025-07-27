use super::*;

pub struct AppParam
{
    // main window:
    pub window : Option<WindowParam>,
}
impl Default for AppParam
{
    fn default() -> Self {
        Self
        {
            window: Some(___()),
        }
    }
}