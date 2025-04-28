pub use crate::*;

/// Memorize where do the context/value come from using it's index
#[derive(Debug)]
pub struct ActionStackRelative<'a, S, A, T, Idx> where S : UndoStack<A>, A : UndoableAction, T : GetMut<Idx>, Idx : Clone
{
    stack : &'a mut S,
    index : Idx,
    phantom : PhantomData<(A,T)>,
}

impl<'a, S, A, T, Idx> ActionStackRelative<'a, S, A, T, Idx> where S : UndoStack<A>, A : UndoableAction, T : GetMut<Idx>, Idx : Clone
{
    pub fn new(undo : &'a mut S, source : Idx) -> Self { Self { stack: undo, index: source, phantom : PhantomData }}
}

/*
impl<'a, U, A, T, Idx> UndoStack<T> for ActionStackRelative<'a, U, A, T, Idx> where U : UndoStack<A>, A : UndoableAction, T : GetMut<Idx>, Idx : Clone
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
*/