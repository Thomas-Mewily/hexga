pub use crate::*;

/* 
pub trait Action
{
    fn execute<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo>;

}
*/
pub trait UndoAction : Sized // + Action
{
    /// The set of action that can be involved when undoing this action
    type Undo : UndoAction;
    type Context<'a>;
    type Output<'a>;

    fn execute_and_forget<'a,U>(self, context : &mut Self::Context<'a>, undo : &mut U) where U : ActionStack<Self::Undo> { self.execute(context, undo); }
    fn execute<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo>;
    
    fn execute_without_undo<'a>(self, context : &mut Self::Context<'a>) -> Self::Output<'a> { self.execute(context, &mut ()) }
    fn execute_without_undo_and_forget<'a>(self, context : &mut Self::Context<'a>) { self.execute_without_undo(context); }
}

pub trait UndoExtension
{
    fn undo_action<'a,A>(&'a mut self, action : A) -> A::Output<'a> where A : UndoAction<Context<'a> = Self> { action.execute_without_undo(self) }
    
    /// Undo the last command
    fn undo<'a,A,U>(&'a mut self, undo : &mut U) -> bool where U : CommandStack<A>, A : UndoAction<Context<'a> = Self> { undo.undo(self) }
    /// Redo the last command
    fn redo<'a,A,U>(&'a mut self, redo : &mut CommandsRedo<U,A>) -> bool where U : CommandStack<A>, A : UndoAction<Context<'a> = Self, Undo = A> { redo.redo(self) }
}
impl<T> UndoExtension for T {}

pub trait ActionStack<A> where A : UndoAction
{
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A;
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
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.undo.push_undo_action(|| (self.f)(f()));
    }
}

/// Ignore the action
impl<A> ActionStack<A> for () where A : UndoAction 
{
    fn push_undo_action<F>(&mut self, _ : F) where F : FnOnce() -> A {}
}

impl<A> ActionStack<A> for Vec<A> where A : UndoAction
{
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A {
        self.push(f());
    }
}
