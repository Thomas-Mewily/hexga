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

    fn execute_in<'a, Dest>(self, context : &mut Self::Context<'a>, dest : &mut Dest) -> Self::Output<'a> where Dest : UndoStack<Self::Undo>;
    fn execute_and_forget_in<'a,Dest>(self, context : &mut Self::Context<'a>, dest : &mut Dest) where Dest : UndoStack<Self::Undo> { self.execute_in(context, dest); }
}

impl<T> Action for T where T : UndoableAction + Sized
{
    type Context<'a>= T::Context<'a>;
    type Output<'a>= T::Output<'a>;

    fn execute<'a>(self, context : &mut Self::Context<'a>) -> Self::Output<'a> { self.execute_in(context, &mut ()) }
}

pub trait UndoExtension
{
    /// Undo the last action/command
    fn undo<'a,A,S>(&'a mut self, src : &mut S) -> bool where S : UndoStack<A>, A : UndoableAction<Context<'a> = Self> { src.stack_undo(self) }
    // Redo the last action/command
    fn redo<'a,A,S>(&'a mut self, src : &mut S) -> bool where S : RedoStack<A>, A : UndoableAction<Context<'a> = Self> { src.stack_redo(self) }
}
impl<T> UndoExtension for T {}

pub trait UndoStack<A> where A : UndoableAction
{
    /// Prepare the next action/command. Can be ommited on action
    fn prepare(&mut self);

    /// If true F will be called, otherwise F won't be called in `push_undo_action`
    const LOG_UNDO : bool;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn handle<'a, T>(&'a mut self, f : fn(T) -> A) -> ActionStackMap<'a,Self,A,T> where Self : Sized, T : UndoableAction { ActionStackMap::new(self, f) }

    fn stack_undo(&mut self, ctx : &mut A::Context<'_>) -> bool { self.stack_undo_in(ctx, &mut ()) }
    fn stack_undo_in<Dest>(&mut self, ctx : &mut A::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<A::Undo>;
}

pub trait RedoStack<A> : UndoStack<A> where A : UndoableAction
{
    fn stack_redo(&mut self, ctx : &mut A::Context<'_>) -> bool { self.stack_redo_in(ctx, &mut ()) }
    fn stack_redo_in<Dest>(&mut self, ctx : &mut A::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<A::Undo>;
}

pub struct ActionStackMap<'a, U, A, T> where U : UndoStack<A>, A : UndoableAction, T : UndoableAction
{
    undo : &'a mut U,
    f : fn(T) -> A,
}

impl<'a, U, A, T> ActionStackMap<'a, U, A, T> where U : UndoStack<A>, A : UndoableAction, T : UndoableAction
{
    pub fn new(undo : &'a mut U, f : fn(T) -> A) -> Self { Self { undo, f }}
}

impl<'a, U, A, T> UndoStack<T> for ActionStackMap<'a, U, A, T> where U : UndoStack<A>, A : UndoableAction, T : UndoableAction
{
    const LOG_UNDO : bool = U::LOG_UNDO;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.undo.push_undo_action(|| (self.f)(f()));
    }
    
    fn stack_undo_in<Dest>(&mut self, _ : &mut <T as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<T::Undo> { false }
    
    fn prepare(&mut self) { self.undo.prepare(); }
}

/// Ignore the action
impl<A> UndoStack<A> for () where A : UndoableAction 
{
    const LOG_UNDO : bool = false;
    fn push_undo_action<F>(&mut self, _ : F) where F : FnOnce() -> A {}
    fn stack_undo_in<Dest>(&mut self, _ : &mut <A as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<A::Undo> { false }
    fn prepare(&mut self) {}
}

// Todo impl it for sequence that support push ?
impl<A> UndoStack<A> for Vec<A> where A : UndoableAction
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A {
        self.push(f());
    }
    
    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<A::Undo> {
        dest.prepare();
        self.pop().map(|v| v.execute_and_forget_in(ctx, dest)).is_some()
    }

    fn prepare(&mut self) {}
}
