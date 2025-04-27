pub use crate::*;

pub trait Action : Sized
{
    type Context<'a>;
    type Output<'a>;
    fn execute<'a>(self, context : &mut Self::Context<'a>) -> Self::Output<'a>;
    fn execute_and_forget<'a>(self, context : &mut Self::Context<'a>) { self.execute(context); }
}

pub trait UndoableAction : Sized
{
    /// The set of action that can be involved when undoing this action
    type Undo : UndoableAction;
    type Context<'a>;
    type Output<'a>;

    // if forget is at true, return some, otherwise return none
    //fn execute_and_forget_cond_in<'a,U>(self, context : &mut Self::Context<'a>, undo : &mut U, forget : bool) -> Option<Self::Output<'a>> where U : ActionStack<Self::Undo> { self.execute_in(context, undo); }

    fn execute_in<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo>;
    fn execute_and_forget_in<'a,U>(self, context : &mut Self::Context<'a>, undo : &mut U) where U : ActionStack<Self::Undo> { self.execute_in(context, undo); }
}

impl<T> Action for T where T : UndoableAction + Sized
{
    type Context<'a>= T::Context<'a>;
    type Output<'a>= T::Output<'a>;

    fn execute<'a>(self, context : &mut Self::Context<'a>) -> Self::Output<'a> { self.execute_in(context, &mut ()) }
}

pub trait UndoExtension
{
    fn undo_action<'a,A>(&'a mut self, action : A) -> A::Output<'a> where A : UndoableAction<Context<'a> = Self> { action.execute(self) }
    
    /// Undo the last command
    fn undo<'a,A,U>(&'a mut self, undo : &mut U) -> bool where U : CommandStack<A>, A : UndoableAction<Context<'a> = Self> { undo.undo(self) }
    /// Redo the last command
    fn redo<'a,A,U>(&'a mut self, redo : &mut CommandsRedo<U,A>) -> bool where U : CommandStack<A>, A : UndoableAction<Context<'a> = Self, Undo = A> { redo.redo(self) }
}
impl<T> UndoExtension for T {}

pub trait ActionStack<A> where A : UndoableAction
{
    /// If true F will be called, otherwise F won't be called in `push_undo_action`
    const LOG_UNDO : bool;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn handle<'a, T>(&'a mut self, f : fn(T) -> A) -> ActionStackMap<'a,Self,A,T> where Self : Sized, T : UndoableAction { ActionStackMap::new(self, f) }

    fn undo(&mut self, ctx : &mut A::Context<'_>) -> bool;

}


pub struct ActionStackMap<'a, U, A, T> where U : ActionStack<A>, A : UndoableAction, T : UndoableAction
{
    undo : &'a mut U,
    f : fn(T) -> A,
}

impl<'a, U, A, T> ActionStackMap<'a, U, A, T> where U : ActionStack<A>, A : UndoableAction, T : UndoableAction
{
    pub fn new(undo : &'a mut U, f : fn(T) -> A) -> Self { Self { undo, f }}
}

impl<'a, U, A, T> ActionStack<T> for ActionStackMap<'a, U, A, T> where U : ActionStack<A>, A : UndoableAction, T : UndoableAction
{
    const LOG_UNDO : bool = U::LOG_UNDO;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.undo.push_undo_action(|| (self.f)(f()));
    }
    
    fn undo(&mut self, _ : &mut <T as UndoableAction>::Context<'_>) -> bool { false }
}

/// Ignore the action
impl<A> ActionStack<A> for () where A : UndoableAction 
{
    const LOG_UNDO : bool = false;
    fn push_undo_action<F>(&mut self, _ : F) where F : FnOnce() -> A {}
    
    fn undo(&mut self, _ : &mut <A as UndoableAction>::Context<'_>) -> bool { false }
}

// Todo impl it for sequence that support push ?
impl<A> ActionStack<A> for Vec<A> where A : UndoableAction
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A {
        self.push(f());
    }
    
    fn undo(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>) -> bool {
        self.pop().map(|v| v.execute_and_forget(ctx)).is_some()
    }
}
