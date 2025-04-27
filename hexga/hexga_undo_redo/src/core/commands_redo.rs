pub use crate::*;

/// Reverse the action, but still suffer the Great Undo-Redo Quandary like most software
/// from the https://github.com/zaboople/klonk/blob/master/TheGURQ.md
pub struct RedoReverse<S,A> where S : UndoStack<A>, A : UndoableAction<Undo = A>
{
    undo : S,
    redo : S,
    phantom : PhantomData<A>,
}
impl<S, A> Clone for RedoReverse<S, A> where S: UndoStack<A> + Clone, A : UndoableAction<Undo = A>
{
    fn clone(&self) -> Self 
    {
        Self {
            undo: self.undo.clone(),
            redo: self.redo.clone(),
            phantom: PhantomData,
        }
    }
}

impl<S, A> Debug for RedoReverse<S, A> where S: UndoStack<A> + Debug, A : UndoableAction<Undo = A>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedoReverse")
            .field("undo", &self.undo)
            .field("redo", &self.redo)
            .finish()
    }
}

impl<S, A> RedoReverse<S, A> where S: UndoStack<A>, A : UndoableAction<Undo = A>
{
    pub fn new() -> Self where S : Default { Self::new_with_undo_redo(___(), ___()) }
    pub fn new_with_undo_redo(undo: S, redo: S) -> Self 
    {
        Self 
        {
            undo,
            redo,
            phantom: PhantomData,
        }
    }

    pub fn undo_stack(&self) -> &S { &self.undo }
    pub fn undo_stack_mut(&mut self) -> &mut S { &mut self.undo }

    pub fn redo_stack(&self) -> &S { &self.redo }
    pub fn redo_stack_mut(&mut self) -> &mut S { &mut self.redo }

    pub fn into_undo_redo_stack(self) -> (S,S) { let Self { undo, redo, phantom: _ } = self; (undo, redo) }
}

impl<U, A> UndoStack<A> for RedoReverse<U,A> where U: UndoStack<A>, A : UndoableAction<Undo = A>
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A { self.undo.push_undo_action(f); }
    
    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<<A as UndoableAction>::Undo> {
        self.undo.stack_undo_in(ctx, dest)
    }  
}

impl<U, A> RedoStack<A> for RedoReverse<U,A> where U: UndoStack<A>, A : UndoableAction<Undo = A>
{
    fn stack_redo(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>) -> bool {
        self.redo.stack_undo_in(ctx, &mut self.undo)
    }
}

impl<U, A> CommandStack<A> for RedoReverse<U,A> where U: CommandStack<A>, A : UndoableAction<Undo = A>
{
    fn prepare(&mut self) { self.undo.prepare(); }
    fn pop_command(&mut self) -> Option<Command<A>> { self.undo.pop_command() }
    
    fn take_last_command_actions(&mut self) -> Option<impl Iterator<Item = A>> { self.undo.take_last_command_actions() }
}