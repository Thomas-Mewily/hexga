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
pub enum CommandFlowMarker<A,G> where A : UndoableAction
{
    // Can probably reduce the size it by using u16 and doing some encoding to allow one command to rollback multiple group
    /// Annonce a group G of N GroupAction
    Group(G,usize),
    /// An action inside a group. G is defined in the Group.
    GroupAction(A),
    /// A group composed of a single action
    Action(G,A),
}
impl<A,G> Debug for CommandFlowMarker<A,G> where A : UndoableAction + Debug, G : Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            Self::Action(g, v) => if std::mem::size_of::<G>() == 0 
            {
                write!(f, "Action({:?})", v)
            } else {
                write!(f, "Action({:?} : {:?})", g, v)
            },
            Self::Group(g,n) => if std::mem::size_of::<G>() == 0 
            {
                write!(f, "Group({})", n)
            } else {
                write!(f, "Group({:?} : {})", g, n)
            },
            Self::GroupAction(a) => write!(f, "GroupAction({:?})", a)
        }
    }
}

impl<A,G> CommandFlowMarker<A,G> where A : UndoableAction
{
    pub const fn is_group (&self) -> bool { matches!(self, Self::Group (_, _)) }
    pub const fn is_action(&self) -> bool { matches!(self, Self::GroupAction(_)) }

    pub fn to_group(self) -> Option<(G,usize)> { if let CommandFlowMarker::Group(group,value) = self { Some((group, value)) } else { None }}
    pub fn to_action(self) -> Option<A> { if let CommandFlowMarker::GroupAction(value) = self { Some(value) } else { None }}

    pub const fn is_nop(&self) -> bool { matches!(self, Self::Group(_, 0)) }
    pub const fn nop(group_data : G) -> Self { CommandFlowMarker::Group(group_data, 0) }
}


/// A flow of commands. All action are stored inside the same sequence.
/// 
/// Each command end with [CommandFlowMarker::Group] with the given size,
/// except for commands composed of only one action, 
/// which can choose to skip it if they want in order to use less memory
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CommandsFlow<A,G> where A : UndoableAction
{
    // Todo : use generic sequence ? vec, vecdequeu...
    pub actions : Vec<CommandFlowMarker<A,G>>,
    // Todo : add a commands count here
}

impl<A,G> Debug for CommandsFlow<A,G> where A : UndoableAction + Debug, G : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.actions) } }
impl<A,G> Default for CommandsFlow<A,G> where A : UndoableAction { fn default() -> Self { Self::new() } }

impl<A,G> Deref for CommandsFlow<A,G> where A : UndoableAction { type Target=Vec<CommandFlowMarker<A,G>>; fn deref(&self) -> &Self::Target {&self.actions } }
impl<A,G> DerefMut for CommandsFlow<A,G> where A : UndoableAction { fn deref_mut(&mut self) -> &mut Self::Target {&mut self.actions } }

impl<A,G> From<Vec<CommandFlowMarker<A,G>>> for CommandsFlow<A,G> where A : UndoableAction { fn from(actions: Vec<CommandFlowMarker<A,G>>) -> Self { Self { actions } } }
impl<A,G> From<CommandsFlow<A,G>> for Vec<CommandFlowMarker<A,G>> where A : UndoableAction { fn from(value: CommandsFlow<A,G>) -> Self { value.actions } }

impl<A,G> From<Commands<A,G>> for CommandsFlow<A,G> where A : UndoableAction { fn from(value: Commands<A,G>) -> Self { value.to_commands_flow() } }

impl<A,G> CommandsFlow<A,G> where A : UndoableAction
{
    pub const fn from_vec(actions : Vec<CommandFlowMarker<A,G>>) -> Self { Self { actions } }

    pub const fn new() -> Self { Self { actions: Vec::new() } }
    pub fn with_capacity(capacity: usize) -> Self { Self { actions: Vec::with_capacity(capacity) } }
    fn len(&self) -> usize { self.actions.len() }

    pub fn actions(&self) -> &[CommandFlowMarker<A,G>] { &self.actions }
    pub fn actions_mut(&mut self) -> &mut [CommandFlowMarker<A,G>] { &mut self.actions }
    pub fn into_actions(self) -> Vec<CommandFlowMarker<A,G>> { self.actions }

    pub fn to_commands(self) -> Commands<A,G> 
    {
        let mut cmds = ___();
        self.extends_commands(&mut cmds);
        cmds
    }
    pub fn extends_commands(mut self, commands : &mut Commands<A,G>)
    {
        /*
        let mut idx = 0;

         
        for v in self.actions.into_iter()
        {
            match v
            {
                CommandFlowMarker::Group(group, size) => 
                {
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
                CommandFlowMarker::GroupAction(_) => todo!(),
                CommandFlowMarker::Action(_, _) => todo!(),
            }
            idx += 1;
        }
        */

        todo!();

        /* 
        loop
        {
            let Some(v) = self.get(idx) else 
            { 
                assert!(self.is_empty());
                return; 
            };

            match v
            {
                CommandFlowMarker::Group(group, size) => 
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
        */
    }
}

impl<A,G> Length for CommandsFlow<A,G> where A : UndoableAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A,G> Capacity for CommandsFlow<A,G> where A : UndoableAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.actions.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.actions.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.actions.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve_exact(additional) }
}


impl<A,G> UndoStack<A,G> for CommandsFlow<A,G> where A : UndoableAction 
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A 
    {
        debug_assert!(self.len().is_non_zero(), "Forget to call CommandsFlow::prepare()");

        match self.actions.pop().unwrap()
        {
            CommandFlowMarker::Group(g, nb) => 
            {
                self.actions.push(CommandFlowMarker::GroupAction(f()));
                self.actions.push(CommandFlowMarker::Group(g, nb+1));
            },
            _ => unreachable!(),
        };
    }
    
    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<<A as UndoableAction>::Undo> {
        dest.prepare();
        self.take_last_command_actions().map(|(_group, actions)| actions.for_each(|a| a.execute_and_forget_in(ctx, dest))).is_some()
    }

    fn prepare_with_data(&mut self, group_data : G) 
    {
        self.actions.push(CommandFlowMarker::nop(group_data));
    }
}

impl<A,G> CommandStack<A,G> for CommandsFlow<A,G> where A : UndoableAction 
{    
    fn pop_command(&mut self) -> Option<(G, Command<A>)> 
    {
        let Some(commands) = self.actions.pop() else { return None; };
        let (group_data, group_size) = commands.to_group().expect("Command flow always end by a group");
        
        let r = match group_size
        {
            0 => Command::Nop,
            1 =>        
            { 
                let last = self.pop().unwrap();
                Command::Action(last.to_action().unwrap())
            }
            _ =>
            {
                let idx_begin_drain = self.len() - group_size;
                Command::Sequence(self.drain(idx_begin_drain..).map(|v| v.to_action().unwrap()).collect())
            }
        };
        debug_assert!(self.is_empty() || self.last().as_ref().unwrap().is_group());
        Some((group_data, r))
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
    
    fn take_last_command_actions(&mut self) -> Option<(G, impl Iterator<Item = A>)> {
        let Some(commands) = self.actions.pop() else { return None; };
        let (group_data, group_size) = commands.to_group().expect("Command flow always end by a group");
        let idx_begin_drain = self.len() - group_size;
        Some((group_data, self.drain(idx_begin_drain..).rev().map(|v| v.to_action().expect("should be an action"))))
    }
}