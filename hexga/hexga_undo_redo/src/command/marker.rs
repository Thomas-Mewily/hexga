pub use crate::*;


/// A marker used to indicate the beginning or ending of a command within a sequence of actions.
/// 
/// Multiple commands can be nested by using multiple begin/end markers or multiple nested scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandMarker<A> where A : UndoAction
{
    Begin,
    End,
    // A command that do nothings
    Nop,
    Action(A),
}
impl<A> CommandMarker<A> where A : UndoAction
{
    pub const fn is_begin (&self) -> bool { matches!(self, Self::Begin) }
    pub const fn is_end   (&self) -> bool { matches!(self, Self::End  ) }
    pub const fn is_nop   (&self) -> bool { matches!(self, Self::Nop  ) }
    pub const fn is_action(&self) -> bool { matches!(self, Self::Action(_)) }

    pub const fn is_begin_or_end (&self) -> bool { matches!(self, Self::Begin | Self::End) }
}

impl<A> ActionStack<A> for CommandStackMarker<A> where A : UndoAction 
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A 
    {
        debug_assert!(self.len().is_non_zero(), "Forget to call CommandStackMarker::begin()");
        self.actions.push(CommandMarker::Action(f()));
    }
}



impl<A> CommandStack<A> for CommandStackMarker<A> where A : UndoAction 
{
    fn begin(&mut self) { self.actions.push(CommandMarker::Begin); }
    fn end(&mut self) 
    { 
        if self.last().map(|v| v.is_begin()).unwrap_or(false) 
        {
            self.actions.pop();
            self.actions.push(CommandMarker::Nop);
        }else 
        {
            self.actions.push(CommandMarker::End);
        }
    }
}

/// Store command.
/// 
/// Each command starts with [CommandMarker::Begin] and ends with [CommandMarker::End],
/// except for commands composed of only one action, which can choose to skip it if they want.
///
/// command that don't do any action need to emit a [CommandMarker::Nop].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandStackMarker<A> where A: UndoAction
{ 
    pub actions : Vec<CommandMarker<A>>,
}
impl<A> Deref for CommandStackMarker<A> where A: UndoAction
{
    type Target=Vec<CommandMarker<A>>;
    fn deref(&self) -> &Self::Target { &self.actions }
}
impl<A> DerefMut for CommandStackMarker<A> where A: UndoAction
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.actions }
}

impl<A> CommandStackMarker<A> where A : UndoAction
{
    pub const fn new() -> Self { Self { actions: Vec::new() } }
    pub fn with_capacity(capacity: usize) -> Self { Self { actions: Vec::with_capacity(capacity) } }
    fn len(&self) -> usize { self.actions.len() }
}
impl<A> Length for CommandStackMarker<A> where A : UndoAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A> Capacity for CommandStackMarker<A> where A : UndoAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.actions.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.actions.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.actions.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve_exact(additional) }
}
