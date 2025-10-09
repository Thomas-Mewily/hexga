use copypasta::ClipboardProvider;

use super::*;

singleton_access!(
    pub Clipboard,
    AppClipboard,
    { App::try_as_ref().map(|ctx| &ctx.clipboard) },
    { App::try_as_mut().map(|ctx| &mut ctx.clipboard) }
);

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

pub trait Clipboardable
{
    fn get(&mut self) -> Option<String>;
    fn set(&mut self, paste : String) -> Result<(), ()>;
}