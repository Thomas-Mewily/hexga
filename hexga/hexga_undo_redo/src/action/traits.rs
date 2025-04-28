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

/// What is the equivalent relative action when the action take place a collection.
/// ex : `x.replace_action(42, &mut stack)`
/// If `x` was a mutable reference to a collection, we need to track where do `x` come from :
/// 
/// ```rust
/// let mut stack = Vec::new();
/// let s = &mut stack;
/// 
/// let array = [1,2,3];
/// let x : &mut i32 = array.get_mut_or_panic(1);
/// 
/// x.replace_action(42, s); // <- We don't know where to x come from
/// ```
/// 
/// But the `replace_action` itself don't know the borrow value was relative to array
/// 
/// ```rust
/// let mut stack = Vec::new();
/// let s = &mut stack;
/// 
/// let array = [1,2,3];
/// array.get_mut_or_panic_action(1,s, |value, tmp_stack| value.replace_action(42, tmp_stack)); // Ok
/// ```
/// 
/// Here, the `actions::mem::Replace` will be related to the borrowed value, thus becoming :
/// `actions::mem::Replace` -> `actions::mem::ReplaceIndex`
pub trait Relative<Idx> : Action where for<'a> Self::Context<'a> : GetMut<Idx>
{
    type Relative<'a> : Action<Context<'a> = Self::Context<'a>>;
    fn relative<'a>(self, idx : Idx) -> Self::Relative<'a>; 
}