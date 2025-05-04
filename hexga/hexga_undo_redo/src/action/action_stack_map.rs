pub use crate::*;

/// Transform a action `T` into a more general action `A`
pub struct ActionStackMap<'a, S, A, G, T> where S : UndoStack<A, G>, A : UndoableAction, T : UndoableAction
{
    stack : &'a mut S,
    f : fn(T) -> A,
    phantom : PhantomData<G>,
}

impl<'a, S, A, G, T> Debug for ActionStackMap<'a, S, A, G, T> where S : UndoStack<A, G>, A : UndoableAction, T : UndoableAction, S : Debug, A : Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionStackMap").field("stack", &self.stack).field("f", &self.f).finish()
    }
}

impl<'a, S, A, G, T> ActionStackMap<'a, S, A, G, T> where S : UndoStack<A,G>, A : UndoableAction, T : UndoableAction
{
    pub fn new(stack : &'a mut S, f : fn(T) -> A) -> Self { Self { stack, f, phantom: PhantomData }}
}

impl<'a, S, A, G, T> UndoStack<T,G> for ActionStackMap<'a, S, A, G, T> where S : UndoStack<A,G>, A : UndoableAction, T : UndoableAction
{
    const LOG_UNDO : bool = S::LOG_UNDO;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.stack.push_undo_action(|| (self.f)(f()));
    }
    
    fn stack_undo_in<Dest>(&mut self, _ : &mut <T as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<T::Undo> { false }
    
    fn prepare_with_data(&mut self, group_data : G) { self.stack.prepare_with_data(group_data); }
}

/// Ignore the action
impl<A,G> UndoStack<A,G> for () where A : UndoableAction 
{
    const LOG_UNDO : bool = false;
    fn push_undo_action<F>(&mut self, _ : F) where F : FnOnce() -> A {}
    fn stack_undo_in<Dest>(&mut self, _ : &mut <A as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<A::Undo> { false }
    fn prepare_with_data(&mut self, _ : G) {}
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

    fn prepare_with_data(&mut self, _ : ()) {}
}
