pub use crate::*;

/// A vector of command
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Commands<A,G=()> where A : UndoableAction
{
    // Todo : use generic sequence ? vec, vecdequeu...
    pub commands : Vec<(G,Command<A>)>,
}

impl<A,G> Debug for Commands<A,G> where A : UndoableAction + Debug, G : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.commands) } }
impl<A,G> Default for Commands<A,G> where A : UndoableAction { fn default() -> Self { Self::new() } }

impl<A,G> Deref for Commands<A,G> where A : UndoableAction { type Target=Vec<(G,Command<A>)>; fn deref(&self) -> &Self::Target { &self.commands } }
impl<A,G> DerefMut for Commands<A,G> where A : UndoableAction { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.commands } }

impl<A,G> From<Vec<(G,Command<A>)>> for Commands<A,G> where A : UndoableAction { fn from(actions: Vec<(G,Command<A>)>) -> Self { Self { commands: actions } } }
impl<A,G> From<Commands<A,G>> for Vec<(G,Command<A>)> where A : UndoableAction { fn from(value: Commands<A,G>) -> Self { value.commands } }



impl<A,G> From<CommandsFlow<A,G>> for Commands<A,G> where A : UndoableAction { fn from(value: CommandsFlow<A,G>) -> Self { value.to_commands() } }

impl<A,G> Commands<A,G> where A : UndoableAction
{
    pub const fn from_vec(actions : Vec<(G,Command<A>)>) -> Self { Self { commands: actions } }

    pub const fn new() -> Self { Self { commands: Vec::new() } }
    pub fn with_capacity(capacity: usize) -> Self { Self { commands: Vec::with_capacity(capacity) } }
    fn len(&self) -> usize { self.commands.len() }

    pub fn commands(&self) -> &[(G,Command<A>)] { &self.commands }
    pub fn commands_mut(&mut self) -> &mut [(G,Command<A>)] { &mut self.commands }
    pub fn into_commands(self) -> Vec<(G,Command<A>)> { self.commands }

    
    pub fn to_commands_flow(self) -> CommandsFlow<A,G> 
    {
        let mut flow = ___();
        self.extends_commands_flow(&mut flow);
        flow
    }
    pub fn extends_commands_flow(self, commands : &mut CommandsFlow<A,G>)
    {
        for (group_data, cmd) in self.commands
        {
            //commands.prepare_with_data(group_data);
            match cmd
            {
                Command::Action(a) => commands.push(CommandFlowMarker::Action(group_data, a)),
                Command::Sequence(mut seq) => 
                {
                    match seq.len()
                    {
                        0 => { commands.push(CommandFlowMarker::nop(group_data)) },   
                        1 => commands.push(CommandFlowMarker::Action(group_data, seq.pop().unwrap())),
                        n => 
                        {
                            for action in seq
                            {
                                commands.push(CommandFlowMarker::GroupAction(action));
                            }
                            commands.push(CommandFlowMarker::Group(group_data, n));
                        }
                    }
                },
                Command::Nop => commands.push(CommandFlowMarker::nop(group_data)),
            }
        }
    }
}

impl<A,G> Length for Commands<A,G> where A : UndoableAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A,G> Capacity for Commands<A,G> where A : UndoableAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.commands.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.commands.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.commands.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.commands.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.commands.try_reserve_exact(additional) }
}

impl<A,G> UndoStack<A,G> for Commands<A,G> where A : UndoableAction
{
    const LOG_UNDO : bool = true;

    #[track_caller]
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A 
    {
        let undo_action = f();
        use Command::*;

        let (group, combined) = self.commands.pop().expect("Forget to call CommandStackSequence::prepare()");

        let command = match combined
        {
            Action(a) => Sequence(vec![a,undo_action]),
            Sequence(mut seq) => { seq.push(undo_action); Sequence(seq) },
            Nop => Action(undo_action),
        };
        self.commands.push((group, command));
    }
    
    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<<A as UndoableAction>::Undo> {
        dest.prepare();
        self.take_last_command_actions().map(|(group, actions)| actions.for_each(|a| a.execute_and_forget_in(ctx, dest))).is_some()
    }

    fn prepare_with_data(&mut self, group_data : G) {
        self.commands.push((group_data, Command::Nop));
    } 
} 

impl<A,G> CommandStack<A,G> for Commands<A,G> where A : UndoableAction
{        
    fn pop_command(&mut self) -> Option<(G,Command<A>)> {
        self.commands.pop()
    }
    
    fn take_last_command_actions(&mut self) -> Option<(G, impl Iterator<Item = A>)> {
        let Some((group, cmd)) = self.commands.pop() else { return None; };

        let it = match cmd
        {
            Command::Action(a) => IterVecOrIterOnce::Once(std::iter::once(a)),
            Command::Sequence(seq) => IterVecOrIterOnce::Vec(seq.into_iter()),
            Command::Nop => IterVecOrIterOnce::Empty(std::iter::empty()),
        };
        Some((group, it))
    }
}

enum IterVecOrIterOnce<T>
{
    Empty(std::iter::Empty<T>),
    Once(std::iter::Once<T>),
    Vec(std::vec::IntoIter<T>),
}

impl<A> Iterator for IterVecOrIterOnce<A>
{
    type Item=A;
    fn next(&mut self) -> Option<Self::Item> 
    {
        match self
        {
            IterVecOrIterOnce::Empty(v) => v.next(),
            IterVecOrIterOnce::Once(v) => v.next(),
            IterVecOrIterOnce::Vec(v) => v.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self
        {
            IterVecOrIterOnce::Empty(v) => v.size_hint(),
            IterVecOrIterOnce::Once(v) => v.size_hint(),
            IterVecOrIterOnce::Vec(v) => v.size_hint(),
        } 
    }

}



/* 
impl<A,G> UndoCommandStack<A> for CommandStackSequence<A> where A : UndoAction
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