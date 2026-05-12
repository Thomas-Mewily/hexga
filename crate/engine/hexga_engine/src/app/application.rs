use super::*;

//pub(crate) type AppInternalEvent = ();

pub trait App<Event = AppEvent, Ctx = ()>
{
    fn event(&mut self, ev: Event, ctx: &mut Ctx) -> Option<Event>;

    /// Update is constantly called over time
    /// Don't draw during update. The [CurrentWindow] surface may not be initialized.
    fn update(&mut self, dt: Duration, ctx: &mut Ctx);
    fn draw(&mut self, coef: coef, ctx: &mut Ctx);
}
