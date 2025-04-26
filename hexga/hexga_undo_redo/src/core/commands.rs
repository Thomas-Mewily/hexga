pub use crate::*;

/// A vector of command
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Commands<A> where A : UndoAction
{
    // Todo : use generic sequence ? vec, vecdequeu...
    pub commands : Vec<Command<A>>,
}

impl<A> Default for Commands<A> where A : UndoAction { fn default() -> Self { Self::new() } }

impl<A> Deref for Commands<A> where A : UndoAction { type Target=Vec<Command<A>>; fn deref(&self) -> &Self::Target {&self.commands } }
impl<A> DerefMut for Commands<A> where A : UndoAction { fn deref_mut(&mut self) -> &mut Self::Target {&mut self.commands } }

impl<A> From<Vec<Command<A>>> for Commands<A> where A : UndoAction { fn from(actions: Vec<Command<A>>) -> Self { Self { commands: actions } } }
impl<A> From<Commands<A>> for Vec<Command<A>> where A : UndoAction { fn from(value: Commands<A>) -> Self { value.commands } }

impl<A> Commands<A> where A : UndoAction
{
    pub const fn from_vec(actions : Vec<Command<A>>) -> Self { Self { commands: actions } }

    pub const fn new() -> Self { Self { commands: Vec::new() } }
    pub fn with_capacity(capacity: usize) -> Self { Self { commands: Vec::with_capacity(capacity) } }
    fn len(&self) -> usize { self.commands.len() }

    pub fn commands(&self) -> &[Command<A>] { &self.commands }
    pub fn commands_mut(&mut self) -> &mut [Command<A>] { &mut self.commands }
    pub fn into_commands(self) -> Vec<Command<A>> { self.commands }
}

impl<A> Length for Commands<A> where A : UndoAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A> Capacity for Commands<A> where A : UndoAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.commands.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.commands.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.commands.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.commands.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.commands.try_reserve_exact(additional) }
}

impl<A> ActionStack<A> for Commands<A> where A : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A 
    {
        let b = f();
        use Command::*;

        let combined = match self.commands.pop().expect("Forget to call CommandStackSequence::prepare()")
        {
            Action(a) => Sequence(vec![a,b]),
            Sequence(mut seq) => { seq.push(b); Sequence(seq) },
            Nop => Action(b),
        };
        self.commands.push(combined);
    }
} 

impl<A> CommandStack<A> for Commands<A> where A : UndoAction
{
    fn prepare(&mut self) 
    {
        self.commands.push(Command::Nop);
    }
}



/* 
impl<A> UndoCommandStack<A> for CommandStackSequence<A> where A : UndoAction
{
    fn undo_and_dont_forget<'a>(&mut self, ctx : <A as UndoAction>::Context<'a>) -> Result<<A as UndoAction>::Output<'a>, ()> {
        todo!()
    }

    fn undo_n(&mut self, mut n : usize, ctx : &mut <A as UndoAction>::Context<'_>) -> Result<(), ()> 
    {
        // discutable. Maybe introduce a new type for an CommandStackSequence that is being used.
        // but it will complicate the end() fn
        if self.is_active() { return Err(()); } 
        
        while n != 0
        {
            let Some(a) = self.actions.pop() else { return Err(()); };

            match a
            {
                CommandSequence::Action(a) => a.execute_without_undo_and_forget(ctx),
                CommandSequence::Sequence(command_sequences) => todo!(),
                CommandSequence::Nop(_) => todo!(),
            }
        }
        Ok(())
    }
}*/

/* 
pub struct CommandStackSequenceNonFlatten<A> where A : UndoAction
{
    actions : Vec<CommandSequence<A>>,
    nb_action : Vec<usize>
}
*/