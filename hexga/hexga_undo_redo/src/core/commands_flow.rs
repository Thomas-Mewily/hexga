pub use crate::*;

/* 

/// A marker used to indicate the beginning or ending of a command within a sequence of actions.
/// 
/// Every command start by Begin and finish by End, except the command composed of only 1 action.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandMarker<A> where A : UndoAction
{
    Begin,
    End,
    Nop,
    Action(A),
}
*/


#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandFlowMarker<A> where A : UndoableAction
{
    // Can probably reduce the size it by using u16 and doing some encoding to allow one command to rollback multiple group
    Group(usize),
    Action(A),
}
impl<A> Debug for CommandFlowMarker<A> where A : UndoableAction + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            Self::Action(v) => write!(f, "{:?}", v),
            Self::Group(n) => write!(f, "Group({})", n),
        }
    }
}

impl<A> CommandFlowMarker<A> where A : UndoableAction
{
    pub const fn is_group (&self) -> bool { matches!(self, Self::Group (_)) }
    pub const fn is_action(&self) -> bool { matches!(self, Self::Action(_)) }

    pub fn to_group(self) -> Option<usize> { if let CommandFlowMarker::Group(value) = self { Some(value) } else { None }}
    pub fn to_action(self) -> Option<A> { if let CommandFlowMarker::Action(value) = self { Some(value) } else { None }}

    pub const fn is_nop(&self) -> bool { matches!(self, Self::Group(0)) }
    pub const NOP : Self = CommandFlowMarker::Group(0);
}


/// A flow of commands. All action are stored inside the same sequence.
/// 
/// Each command end with [CommandFlowMarker::Group] with the given size,
/// except for commands composed of only one action, 
/// which can choose to skip it if they want in order to use less memory
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CommandsFlow<A> where A : UndoableAction
{
    // Todo : use generic sequence ? vec, vecdequeu...
    pub actions : Vec<CommandFlowMarker<A>>,
    // Todo : add a commands count here
}

impl<A> Debug for CommandsFlow<A> where A : UndoableAction + Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.actions) } }
impl<A> Default for CommandsFlow<A> where A : UndoableAction { fn default() -> Self { Self::new() } }

impl<A> Deref for CommandsFlow<A> where A : UndoableAction { type Target=Vec<CommandFlowMarker<A>>; fn deref(&self) -> &Self::Target {&self.actions } }
impl<A> DerefMut for CommandsFlow<A> where A : UndoableAction { fn deref_mut(&mut self) -> &mut Self::Target {&mut self.actions } }

impl<A> From<Vec<CommandFlowMarker<A>>> for CommandsFlow<A> where A : UndoableAction { fn from(actions: Vec<CommandFlowMarker<A>>) -> Self { Self { actions } } }
impl<A> From<CommandsFlow<A>> for Vec<CommandFlowMarker<A>> where A : UndoableAction { fn from(value: CommandsFlow<A>) -> Self { value.actions } }

impl<A> From<Commands<A>> for CommandsFlow<A> where A : UndoableAction { fn from(value: Commands<A>) -> Self { value.to_commands_flow() } }

impl<A> CommandsFlow<A> where A : UndoableAction
{
    pub const fn from_vec(actions : Vec<CommandFlowMarker<A>>) -> Self { Self { actions } }

    pub const fn new() -> Self { Self { actions: Vec::new() } }
    pub fn with_capacity(capacity: usize) -> Self { Self { actions: Vec::with_capacity(capacity) } }
    fn len(&self) -> usize { self.actions.len() }

    pub fn actions(&self) -> &[CommandFlowMarker<A>] { &self.actions }
    pub fn actions_mut(&mut self) -> &mut [CommandFlowMarker<A>] { &mut self.actions }
    pub fn into_actions(self) -> Vec<CommandFlowMarker<A>> { self.actions }

    pub fn to_commands(self) -> Commands<A> 
    {
        let mut cmds = ___();
        self.extends_commands(&mut cmds);
        cmds
    }
    pub fn extends_commands(mut self, commands : &mut Commands<A>)
    {
        let mut idx = 0;
        loop
        {
            let Some(v) = self.get(idx) else 
            { 
                assert!(self.is_empty());
                return; 
            };

            match v
            {
                CommandFlowMarker::Group(size) => 
                {
                    let size = *size;
                    let individual_len = idx - size;

                    for unit_action in self.drain(0..individual_len).map(|v| v.to_action().unwrap())
                    {
                        commands.push(Command::Action(unit_action));
                    }

                    if size.is_zero() { commands.push(Command::Nop); }
                    else 
                    {
                        commands.push(Command::Sequence(self.drain(0..idx).map(|v| v.to_action().unwrap()).collect()));
                    }
                },
                CommandFlowMarker::Action(_) => {}, // Probably in a group
            }
            idx += 1;

        }
    }
}

impl<A> Length for CommandsFlow<A> where A : UndoableAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A> Capacity for CommandsFlow<A> where A : UndoableAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.actions.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.actions.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.actions.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve_exact(additional) }
}


impl<A> UndoStack<A> for CommandsFlow<A> where A : UndoableAction 
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A 
    {
        debug_assert!(self.len().is_non_zero(), "Forget to call CommandStackMarker::begin()");

        let group_size = match self.actions.pop()
        {
            Some(v) => match v
            {
                CommandFlowMarker::Group(nb) => nb + 1,
                _ => unreachable!("actions should always finish by a group"),
            }
            None => 1,
        };
        self.actions.push(CommandFlowMarker::Action(f()));
        self.actions.push(CommandFlowMarker::Group(group_size));
    }
    
    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<<A as UndoableAction>::Undo> {
        dest.prepare();
        self.take_last_command_actions().map(|actions| actions.for_each(|a| a.execute_and_forget_in(ctx, dest))).is_some()
    }

    fn prepare(&mut self) 
    {
        if let Some(CommandFlowMarker::Group(nb)) = self.actions.last()
        {
            if *nb != 1
            {
                self.actions.push(CommandFlowMarker::NOP);
            }
        }else
        {
            self.actions.push(CommandFlowMarker::NOP);
        }
    }
}

impl<A> CommandStack<A> for CommandsFlow<A> where A : UndoableAction 
{    
    fn pop_command(&mut self) -> Option<Command<A>> 
    {
        let Some(commands) = self.actions.pop() else { return None; };
        let group_size = commands.to_group().expect("Command flow always end by a group");
        
        let r = match group_size
        {
            0 => Some(Command::Nop),
            1 =>        
            { 
                let last = self.pop().unwrap();
                Some(Command::Action(last.to_action().unwrap()))
            }
            _ =>
            {
                let idx_begin_drain = self.len() - group_size;
                Some(Command::Sequence(self.drain(idx_begin_drain..).map(|v| v.to_action().unwrap()).collect()))
            }
        };
        debug_assert!(self.is_empty() || self.last().as_ref().unwrap().is_group());
        r
    }
    
    /* 
    fn undo(&mut self, ctx : &mut <A as UndoAction>::Context<'_>) -> Result<(), ()> {
        let Some(commands) = self.actions.pop() else { return Err(()); };
        let group_size = commands.to_group().expect("Command flow always end by a group");

        for _ in 0..group_size
        {
            self.pop().expect("invalid group size").to_action().expect("should be an action").execute_without_undo_and_forget(ctx);
        }
        debug_assert!(self.is_empty() || self.last().as_ref().unwrap().is_group());
        Ok(())
    }
    */
    
    fn take_last_command_actions(&mut self) -> Option<impl Iterator<Item = A>> {
        let Some(commands) = self.actions.pop() else { return None; };
        let group_size = commands.to_group().expect("Command flow always end by a group");
        let idx_begin_drain = self.len() - group_size;
        Some(self.drain(idx_begin_drain..).rev().map(|v| v.to_action().expect("should be an action")))
    }
}