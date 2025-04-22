#[allow(unused_imports)]
use std::marker::PhantomData;

pub trait UndoStack<A : UndoAction>
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A;
    fn handle<'a, T>(&'a mut self, f : fn(T) -> A) -> UndoStackMap<'a,Self,A,T> where Self : Sized, T : UndoAction { UndoStackMap::new(self, f) }
}

pub struct UndoStackMap<'a, U, A, T> where U : UndoStack<A>, A : UndoAction, T : UndoAction
{
    undo : &'a mut U,
    f : fn(T) -> A,
}

impl<'a, U, A, T> UndoStackMap<'a, U, A, T> where U : UndoStack<A>, A : UndoAction, T : UndoAction
{
    pub fn new(undo : &'a mut U, f : fn(T) -> A) -> Self { Self { undo, f }}
}

impl<'a, U, A, T> UndoStack<T> for UndoStackMap<'a, U, A, T> where U : UndoStack<A>, A : UndoAction, T : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.undo.push(|| (self.f)(f()));
    }
}

/// Ignore the action
impl<A> UndoStack<A> for () where A : UndoAction 
{
    fn push<F>(&mut self, _ : F) where F : FnOnce() -> A {}
}

impl<A> UndoStack<A> for Vec<A> where A : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A {
        self.push(f());
    }
}

pub trait UndoAction : Sized + Clone
{
    /// The set of action that can be involved when undoing your action
    type ActionSet : UndoAction;
    type Context;
    type Output<'a>;
    
    fn execute<'a, U>(self, context : &'a mut Self::Context, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::ActionSet>;
    fn execute_without_undo<'a>(self, context : &'a mut Self::Context) -> Self::Output<'a> { self.execute(context, &mut ()) }
}

pub trait UndoExtension
{
    fn undo_action<'a,A>(&'a mut self, action : A) -> A::Output<'a> where A : UndoAction<Context = Self> { action.execute_without_undo(self) }
}
impl<T> UndoExtension for T {}