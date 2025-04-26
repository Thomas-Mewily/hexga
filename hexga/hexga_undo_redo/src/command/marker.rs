pub use crate::*;


/// A marker used to indicate the beginning or ending of a command within a sequence of actions.
/// 
/// Multiple commands can be nested by using multiple begin/end markers or multiple nested scopes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandMarker<A> where A : UndoAction
{
    Begin(usize),
    End(usize),
    Nop(usize),
    Action(A),
}
impl<A> Debug for CommandMarker<A> where A : UndoAction + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self 
        {
            Self::Action(v) => write!(f, "{:?}", v),
            Self::Begin(nb) => match nb
            {
                1 => write!(f, "Begin"),
                n => write!(f, "Begin x{}", n),
            },
            Self::End(nb) => match nb
            {
                1 => write!(f, "End"),
                n => write!(f, "End x{}", n),
            },
            Self::Nop(nb) => match nb
            {
                1 => write!(f, "Nop"),
                n => write!(f, "Nop x{}", n),
            },
        }
    }
}

impl<A> CommandMarker<A> where A : UndoAction
{
    pub const fn is_begin (&self) -> bool { matches!(self, Self::Begin(_)) }
    pub const fn is_end   (&self) -> bool { matches!(self, Self::End(_)  ) }
    pub const fn is_nop   (&self) -> bool { matches!(self, Self::Nop(_)  ) }
    pub const fn is_action(&self) -> bool { matches!(self, Self::Action(_)) }

    pub const fn is_zero_action(&self) -> bool { matches!(self, Self::Nop(0) | Self::Begin(0) | Self::End(0)) }

    pub const fn is_begin_or_end (&self) -> bool { matches!(self, Self::Begin(_) | Self::End(_)) }
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
    fn begin(&mut self) { self.push(CommandMarker::Begin(1)); }
    fn end(&mut self) 
    { 
        match self.last_mut()
        {
            Some(CommandMarker::Begin(n)) if *n >= 1 => 
            {
                self.actions.pop();
                self.actions.push(CommandMarker::Nop(1));
            },
            Some(CommandMarker::Nop(v)) => 
            {
                v.increase_checked().expect("that a lot of Nop");
            },
            _ => self.actions.push(CommandMarker::End(1)),
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
    // Todo : use generic sequence ? vec, vecdequeu...
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
    pub fn len(&self) -> usize { self.actions.len() }

    pub fn push(&mut self, value : CommandMarker<A>)
    {
        if value.is_zero_action() { return; }
        
        if let Some(v) = self.last_mut()
        {
            use CommandMarker::*;
            match (v, value)
            {
                (Begin(a), Begin(b)) => 
                {
                    match a.checked_add(b)
                    {
                        Some(total) => *a = total,
                        None => 
                        {
                            let rest = usize::MAX - if *a > b
                            {
                                *a - b
                            }else
                            {
                                b - *a
                            };
                            *a = usize::MAX;
                            self.push(Begin(rest)); 
                        }
                    }
                }
                (End(a), End(b)) => 
                {
                    match a.checked_add(b)
                    {
                        Some(total) => *a = total,
                        None => 
                        {
                            let rest = usize::MAX - if *a > b
                            {
                                *a - b
                            }else
                            {
                                b - *a
                            };
                            *a = usize::MAX;
                            self.push(Begin(rest)); 
                        }
                    }
                },
                (Nop(a), Nop(b)) => 
                {
                    match a.checked_add(b)
                    {
                        Some(total) => *a = total,
                        None => 
                        {
                            let rest = usize::MAX - if *a > b
                            {
                                *a - b
                            }else
                            {
                                b - *a
                            };
                            *a = usize::MAX;
                            self.push(Begin(rest)); 
                        }
                    }
                },
                (_, b) => { self.actions.push(b); }
            }
        }else
        {
            self.actions.push(value);
        }
    }
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
