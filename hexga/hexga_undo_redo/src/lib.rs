/*
// See v14
pub trait Action : Sized
{
    type Context<'a>;
    type Output<'a>;
    fn execute<'a>(self, ctx: &mut Self::Context<'a>) -> Self::Output<'a>;
    fn execute_and_forget<'a>(self, context : &mut Self::Context<'a>) { self.execute(context); }
}

pub trait UndoableAction : Sized
{
    /// The set of action that can be involved when undoing this action
    type Undo : UndoableAction;
    type Context<'a>;
    type Output<'a>;

    fn execute_in<'a, Dest>(self, context : &mut Self::Context<'a>, dest : &mut Dest) -> Self::Output<'a> where Dest : UndoStack<Self::Undo>;
    fn execute_and_forget_in<'a,Dest>(self, context : &mut Self::Context<'a>, dest : &mut Dest) where Dest : UndoStack<Self::Undo> { self.execute_in(context, dest); }
}


pub trait UndoStack<A> where A : UndoableAction
{
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A;
}
     */