use copypasta::ClipboardProvider;
use super::*;

pub mod prelude
{
    pub use super::{Clipboard,Clipboardable};
    pub(crate) use super::*;
}

pub trait Clipboardable
{
    fn get(&mut self) -> Option<String>;
    fn set(&mut self, paste : String) -> Result<(), ()>;
}

pub struct Clipboard;
impl Clipboardable for Clipboard
{
    fn get(&mut self) -> Option<String> {
        app().clipboard.get()
    }

    fn set(&mut self, paste : String) -> Result<(), ()> {
        app().clipboard.set(paste)
    }
}


pub struct AppClipboard
{
    ctx : Option<copypasta::ClipboardContext>,
}

impl Debug for AppClipboard
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        f.debug_struct("ClipboardManager").finish()
    }
}

impl Default for AppClipboard
{
    fn default() -> Self {
        Self::new()
    }
}

impl AppClipboard
{
    pub fn new() -> Self { Self { ctx: copypasta::ClipboardContext::new().ok() } }
}

impl Clipboardable for AppClipboard
{
    fn get(&mut self) -> Option<String> {
        self.ctx.as_mut().map(|c| c.get_contents().ok()).flatten()
    }

    fn set(&mut self, paste : String) -> Result<(), ()>
    {
        if let Some(c) = self.ctx.as_mut()
        {
            c.set_contents(paste).map_err(|_| ())
        }else
        {
            Err(())
        }
    }
}

