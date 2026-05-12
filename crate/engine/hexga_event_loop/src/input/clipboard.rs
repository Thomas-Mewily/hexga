use super::*;
use copypasta::ClipboardProvider;

pub mod prelude
{
    pub(crate) use super::*;
    pub use super::{Clipboard, Clipboardable};
}

pub trait Clipboardable
{
    fn get_clipboard(&mut self) -> Option<String>;
    fn set_clipboard(&mut self, paste: String) -> Result<(), ()>;
}

pub struct Clipboard
{
    ctx: Option<copypasta::ClipboardContext>,
}

impl Debug for Clipboard
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "Clipboard") }
}

impl Default for Clipboard
{
    fn default() -> Self { Self::new() }
}

impl Clipboard
{
    pub fn new() -> Self
    {
        Self {
            ctx: copypasta::ClipboardContext::new().ok(),
        }
    }
}

impl Clipboardable for Clipboard
{
    fn get_clipboard(&mut self) -> Option<String>
    {
        self.ctx.as_mut().map(|c| c.get_contents().ok()).flatten()
    }

    fn set_clipboard(&mut self, paste: String) -> Result<(), ()>
    {
        if let Some(c) = self.ctx.as_mut()
        {
            c.set_contents(paste).map_err(|_| ())
        }
        else
        {
            Err(())
        }
    }
}
