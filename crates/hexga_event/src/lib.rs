pub mod prelude
{
    pub use super::traits::*;
}

pub mod traits
{
    //pub use super::HandleEventWithContext;
    pub use super::HandleEvent;
}

/*
pub trait HandleEventWithContext<E, Ctx: ?Sized =()>
{
    fn handle_event_with(&mut self, event: E, ctx: &mut Ctx) -> Option<E>;
    //fn update(&mut self);
}
*/

/*
#[derive(Debug)]
pub struct WithContext<'a,V,Ctx>
{
    pub value: &'a mut V,
    pub context: &'a mut Ctx,
}
*/

pub trait HandleEvent<E>
{
    fn handle_event(&mut self, event: E) -> Option<E>;
}

/*
impl<E,Ctx,S> HandleEventWithContext<E, Ctx> for S where S: HandleEvent<E>, Ctx: ?Sized
{
    fn handle_event_with(&mut self, event: E, ctx: &mut Ctx) -> Option<E> {
        let _ = ctx;
        self.handle_event(event)
    }
}
*/
