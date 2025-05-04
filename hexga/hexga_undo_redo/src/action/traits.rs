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

pub trait UndoStack<A,G=()> where A : UndoableAction
{
    /// Prepare the next group of action/command. Can be ommited on action
    fn prepare_with_data(&mut self, group_data : G);

    /// Prepare the next group of action/command. Can be ommited on action
    fn prepare(&mut self) where G : Default { self.prepare_with_data(___());}

    /// If true F will be called, otherwise F won't be called in `push_undo_action`
    const LOG_UNDO : bool;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn handle<'a, T>(&'a mut self, f : fn(T) -> A) -> ActionStackMap<'a,Self,A,G,T> where Self : Sized, T : UndoableAction { ActionStackMap::new(self, f) }

    fn stack_undo(&mut self, ctx : &mut A::Context<'_>) -> bool { self.stack_undo_in(ctx, &mut ()) }
    fn stack_undo_in<Dest>(&mut self, ctx : &mut A::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<A::Undo>;

    //fn relative_to<'a, Idx, C, P>(&'a mut self, index : Idx, collection : &'a mut C) -> ActionStackRelative<'a,Self,A,G,C,Idx,P> where Self : Sized, C : GetMut<Idx>, Idx : Clone, P: Policy { ActionStackRelative::new(self, collection, index) }
}

pub trait RedoStack<A,G=()> : UndoStack<A,G> where A : UndoableAction
{
    fn stack_redo(&mut self, ctx : &mut A::Context<'_>) -> bool { self.stack_redo_in(ctx, &mut ()) }
    fn stack_redo_in<Dest>(&mut self, ctx : &mut A::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<A::Undo>;
}
