pub use crate::*;


/// Once executing an action, clear the redo timeline.
/// 
/// So this still suffer the [Great Undo-Redo Quandary like most software](https://github.com/zaboople/klonk/blob/master/TheGURQ.md). 
pub struct Redo<S,A,G> where S : UndoStack<A,G>, A : UndoableAction<Undo = A>
{
    undo : S,
    redo : S,
    phantom : PhantomData<(A,G)>,
}
impl<S, A, G> Clone for Redo<S, A, G> where S: UndoStack<A,G> + Clone, A : UndoableAction<Undo = A>
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

impl<S, A, G> Debug for Redo<S, A, G> where S: UndoStack<A,G> + Debug, A : UndoableAction<Undo = A>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Redo")
            .field("undo", &self.undo)
            .field("redo", &self.redo)
            .finish()
    }
}

impl<S, A, G> Redo<S, A, G> where S: UndoStack<A,G>, A : UndoableAction<Undo = A>
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

impl<S, A, G> UndoStack<A, G> for Redo<S,A,G> where S: UndoStack<A,G>, A : UndoableAction<Undo = A>
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A { self.undo.push_undo_action(f); }

    fn stack_undo(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>) -> bool 
    {
        self.undo.stack_undo_in(ctx, &mut self.redo)
    }  

    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<<A as UndoableAction>::Undo> {
        self.undo.stack_undo_in(ctx, dest)
    }

    fn prepare_with_data(&mut self, group_data : G) { self.undo.prepare_with_data(group_data); }
}

impl<S, A, G> RedoStack<A,G> for Redo<S,A,G> where S: UndoStack<A,G>, A : UndoableAction<Undo = A>
{
    fn stack_redo(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>) -> bool 
    {
        self.redo.stack_undo_in(ctx, &mut self.undo)
    }
    
    fn stack_redo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<<A as UndoableAction>::Undo> {
        self.redo.stack_undo_in(ctx, dest)
    }
}

impl<S, A, G> CommandStack<A,G> for Redo<S,A,G> where S: CommandStack<A,G>, A : UndoableAction<Undo = A>
{
    fn pop_command(&mut self) -> Option<(G,Command<A>)> { self.undo.pop_command() }
    
    fn take_last_command_actions(&mut self) -> Option<(G, impl Iterator<Item = A>)> { self.undo.take_last_command_actions() }
}