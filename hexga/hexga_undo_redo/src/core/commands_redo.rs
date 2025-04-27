pub use crate::*;

pub struct CommandsRedo<U,A> where U : CommandStack<A>, A : ActionUndo<Undo = A>
{
    undo : U,
    redo : U,
    phantom : PhantomData<A>,
}
impl<U, A> Clone for CommandsRedo<U, A> where U: CommandStack<A> + Clone, A : ActionUndo<Undo = A>
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

impl<U, A> Debug for CommandsRedo<U, A> where U: CommandStack<A> + Debug, A : ActionUndo<Undo = A>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandsRedo")
            .field("undo", &self.undo)
            .field("redo", &self.redo)
            .finish()
    }
}

impl<U, A> CommandsRedo<U, A> where U: CommandStack<A>, A : ActionUndo<Undo = A>
{
    pub fn new() -> Self where U : Default { Self::new_with_undo_redo(___(), ___()) }
    pub fn new_with_undo_redo(undo: U, redo: U) -> Self 
    {
        Self {
            undo,
            redo,
            phantom: PhantomData,
        }
    }

    pub fn undo_stack(&self) -> &U { &self.undo }
    pub fn undo_stack_mut(&mut self) -> &mut U { &mut self.undo }

    pub fn redo_stack(&self) -> &U { &self.redo }
    pub fn redo_stack_mut(&mut self) -> &mut U { &mut self.redo }

    pub fn into_undo_redo_stack(self) -> (U,U) { let Self { undo, redo, phantom: _ } = self; (undo, redo) }

    pub fn redo(&mut self, ctx : &mut A::Context<'_>) -> bool 
    {
        self.undo.prepare();
        self.redo.iter_last_action_actions().map(|actions| actions.for_each(|a| a.execute_and_forget_in(ctx, &mut self.undo))).is_some()
    }
}

impl<U, A> ActionStack<A> for CommandsRedo<U,A> where U: CommandStack<A>, A : ActionUndo<Undo = A>
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A { self.undo.push_undo_action(f); }    
}

impl<U, A> CommandStack<A> for CommandsRedo<U,A> where U: CommandStack<A>, A : ActionUndo<Undo = A>
{
    fn prepare(&mut self) { self.undo.prepare(); }
    fn pop_command(&mut self) -> Option<Command<A>> { self.undo.pop_command() }

    fn undo(&mut self, ctx : &mut <A as ActionUndo>::Context<'_>) -> bool 
    {
        self.redo.prepare();
        self.undo.iter_last_action_actions().map(|actions| actions.for_each(|a| a.execute_and_forget_in(ctx, &mut self.redo))).is_some()
    }
    
    fn iter_last_action_actions(&mut self) -> Option<impl Iterator<Item = A>> { self.undo.iter_last_action_actions() }
}