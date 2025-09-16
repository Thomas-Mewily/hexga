use super::*;






pub trait App: 'static
{
    fn handle_event(&mut self, ev: AppEvent, ctx: &mut Ctx) { self.dispatch_event(ev, ctx); }
    fn dispatch_event(&mut self, ev: AppEvent, ctx: &mut Ctx)
    {
        match ev
        {
            AppEvent::Update => self.update(ctx),
            AppEvent::Draw => self.draw(ctx),
            AppEvent::Resumed => self.resumed(ctx),
            AppEvent::Paused => self.paused(ctx),
            AppEvent::Key(key_event) => todo!(),
        }
    }

    fn paused(&mut self, ctx: &mut Ctx) { let _ = ctx; }
    fn resumed(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn update(&mut self, ctx: &mut Ctx) { let _ = ctx; }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }
}


