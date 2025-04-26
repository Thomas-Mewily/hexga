pub use crate::*;


pub trait CommandStack<A> : ActionStack<A> where A : UndoAction
{
    /// Marks the beginning a command
    fn begin(&mut self);
    /// Marks the end of a command. Panics if `begin()` was not called before.
    fn end(&mut self);

    /// Execute the whole command inside the f scope
    fn execute<R,F>(&mut self, f : F) -> R where F : FnOnce(&mut Self) -> R
    {
        self.begin();
        let r = f(self);
        self.end();
        r
    }
}

pub trait UndoCommandStack<A> : CommandStack<A> where A : UndoAction
{
    // Todo : use generic sequence ? vec, vecdequeu...
    fn pop_to_marker(&mut self) -> CommandStackMarker<A>;
    fn pop_to_sequence(&mut self) -> CommandStackSequence<A>;

    fn undo_and_dont_forget<'a>(&mut self, ctx : A::Context<'a>) -> A::Output<'a>;
    fn undo(&mut self, ctx : A::Context<'_>);
}


/* 
pub trait CommandStack<A : UndoAction>
{
    fn execute<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn undo<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn redo<F>(&mut self, f : F) where F : FnOnce() -> A;
}
pub struct Undo
*/

