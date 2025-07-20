use copypasta::ClipboardProvider;

use super::*;

declare_context!(Clipboard, ClipboardManager, clipboard);


pub struct ClipboardManager
{
    ctx : Option<copypasta::ClipboardContext>,
}

impl Debug for ClipboardManager
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        f.debug_struct("ClipboardManager").finish()
    }
}

impl Default for ClipboardManager
{
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardManager
{
    pub fn new() -> Self { Self { ctx: copypasta::ClipboardContext::new().ok() } }
}

impl IClipboard for ClipboardManager
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

pub trait IClipboard
{
    fn get(&mut self) -> Option<String>;
    fn set(&mut self, paste : String) -> Result<(), ()>;
}