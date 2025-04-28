pub use crate::*;

/// Transform a action `T` into a more general action `A`
#[derive(Debug)]
pub struct ActionStackMap<'a, S, A, T> where S : UndoStack<A>, A : UndoableAction, T : UndoableAction
{
    stack : &'a mut S,
    f : fn(T) -> A,
}

impl<'a, S, A, T> ActionStackMap<'a, S, A, T> where S : UndoStack<A>, A : UndoableAction, T : UndoableAction
{
    pub fn new(stack : &'a mut S, f : fn(T) -> A) -> Self { Self { stack, f }}
}

impl<'a, S, A, T> UndoStack<T> for ActionStackMap<'a, S, A, T> where S : UndoStack<A>, A : UndoableAction, T : UndoableAction
{
    const LOG_UNDO : bool = S::LOG_UNDO;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.stack.push_undo_action(|| (self.f)(f()));
    }
    
    fn stack_undo_in<Dest>(&mut self, _ : &mut <T as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<T::Undo> { false }
    
    fn prepare(&mut self) { self.stack.prepare(); }
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
