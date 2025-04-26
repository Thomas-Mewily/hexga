pub use crate::*;

pub trait UndoAction : Sized
{
    /// The set of action that can be involved when undoing this action
    type Undo : UndoAction;
    type Context<'a>;
    type Output<'a>;

    fn execute_and_forget<'a,U>(self, context : Self::Context<'a>, undo : &mut U) where U : ActionStack<Self::Undo> { self.execute(context, undo); }
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo>;
    
    fn execute_without_undo<'a>(self, context : Self::Context<'a>) -> Self::Output<'a> { self.execute(context, &mut ()) }
    fn execute_without_undo_and_forget<'a>(self, context : Self::Context<'a>) { self.execute_without_undo(context); }
}

pub trait UndoExtension
{
    fn undo_action<'a,A>(&'a mut self, action : A) -> A::Output<'a> where A : UndoAction<Context<'a> = &'a mut Self> { action.execute_without_undo(self) }
    
    fn undo_and_dont_forget<'a,A,U>(&'a mut self, undo : &mut U) -> A::Output<'a> where U : UndoCommandStack<A>, A : UndoAction<Context<'a> = &'a mut Self>
    {
        undo.undo_and_dont_forget(self)
    }

    fn undo<'a,A,U>(&'a mut self, undo : &mut U) where U : UndoCommandStack<A>, A : UndoAction<Context<'a> = &'a mut Self>
    {
        undo.undo(self);
    }
}
impl<T> UndoExtension for T {}

pub trait ActionStack<A> where A : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn handle<'a, T>(&'a mut self, f : fn(T) -> A) -> UndoStackMap<'a,Self,A,T> where Self : Sized, T : UndoAction { UndoStackMap::new(self, f) }

    /* 
    fn undo_action<'a>(&mut self, action : A::Context<'a>) -> A::Output<'a> where A : UndoAction<Context<'a> = &'a mut Self> 
    { 
        context.execute_without_undo(context)
        action.execute_without_undo(self) 
    }*/
}


pub struct UndoStackMap<'a, U, A, T> where U : ActionStack<A>, A : UndoAction, T : UndoAction
{
    undo : &'a mut U,
    f : fn(T) -> A,
}

impl<'a, U, A, T> UndoStackMap<'a, U, A, T> where U : ActionStack<A>, A : UndoAction, T : UndoAction
{
    pub fn new(undo : &'a mut U, f : fn(T) -> A) -> Self { Self { undo, f }}
}

impl<'a, U, A, T> ActionStack<T> for UndoStackMap<'a, U, A, T> where U : ActionStack<A>, A : UndoAction, T : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.undo.push(|| (self.f)(f()));
    }
}

/// Ignore the action
impl<A> ActionStack<A> for () where A : UndoAction 
{
    fn push<F>(&mut self, _ : F) where F : FnOnce() -> A {}
}

impl<A> ActionStack<A> for Vec<A> where A : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A {
        self.push(f());
    }
}
