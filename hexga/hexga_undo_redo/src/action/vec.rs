use crate::*;
pub use action::collections;
pub use action::mem;

pub type Clear  <T> = collections::Clear       <Vec<T>       >;
pub type Set    <T> = collections::SetIndex    <Vec<T>, usize>;
pub type Replace<T> = collections::ReplaceIndex<Vec<T>, usize>;

pub struct Pop<T> { phantom : PhantomData<T> }
impl<T> Pop<T> { pub const fn new() -> Self { Self { phantom: PhantomData }}}

impl<T> Clone for Pop<T> { fn clone(&self) -> Self { Self::new() } }
impl<T> Copy for Pop<T>  {}
impl<T> PartialEq for Pop<T> { fn eq(&self, _: &Self) -> bool { true } }
impl<T> Eq for Pop<T> {}
impl<T> Hash for Pop<T> { fn hash<H: std::hash::Hasher>(&self, _: &mut H) { } }
impl<T> Default for Pop<T>{ fn default() -> Self { Self::new() } }
impl<T> Debug for Pop<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Pop") } }

impl<T> UndoableAction for Pop<T> where for<'a> T: 'a + Clone
{
    type Undo = Push<T>;
    type Context<'a>= Vec<T>;
    type Output<'a> = Option<T>;

    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        context.pop().map(|v| { stack.push_undo_action(|| Push::new(v.clone())); v })
    }

    fn execute_and_forget_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) where S : UndoStack<Self::Undo> {
        context.pop().map(|v| { stack.push_undo_action(|| Push::new(v)); });
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Push<T> { value : T }
impl<T> Debug for Push<T> where T : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Push({:?})", self.value) } }

impl<T> Push<T> 
{ 
    pub const fn new(value : T) -> Self { Self { value }}
    pub fn into_value(self) -> T { self.value }
    pub fn value(&self) -> &T { &self.value }
}

impl<T> UndoableAction for Push<T> where for<'a> T: 'a + Clone
{
    type Undo = Pop<T>;
    type Context<'a>= Vec<T>;
    type Output<'a> = ();
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        context.push(self.value);
        stack.push_undo_action(|| Pop::new());
    }
}

pub type Swap<T> = collections::SwapIndex<Vec<T>, usize>;

impl<T> UndoableAction for Swap<T> where for<'a> T: 'a
{
    type Undo = Self;
    type Context<'a>= Vec<T>;
    type Output<'a> = Result<(), std::slice::GetDisjointMutError>;
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        // Todo : add fn is_useful(&self) -> bool; and fn is_useful_on(&self, &ctx) -> bool; in this trait.
        //if self.i() != self.j() { return; }
        context.get_disjoint_mut([self.i, self.j]).map(|[a,b]|
            {
                std::mem::swap(a, b);
                stack.push_undo_action(|| self);
            })
    }
}



#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Action<T> where for<'a> T : 'a + Clone
{
    Push   (Push<T>),
    Pop    (Pop<T>),

    Clear  (Clear<T>),

    Set    (Set<T>),
    Replace(Replace<T>), 
    Swap   (Swap<T>),

    MemReplace(mem::Replace<Vec<T>>),
    MemTake   (mem::Take<Vec<T>>),
}
impl<T> Debug for Action<T> where for<'a> T : 'a + Clone + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Push(v) => write!(f, "{:?}", v),
            Self::Pop(v) => write!(f, "{:?}", v),
            Self::Clear(v) => write!(f, "{:?}", v),
            Self::Set(v) => write!(f, "{:?}", v),
            Self::Replace(v) => write!(f, "{:?}", v),
            Self::Swap(v) => write!(f, "{:?}", v),
            Self::MemReplace(v) => write!(f, "{:?}", v),
            Self::MemTake(v) => write!(f, "{:?}", v),
        }
    }
}

impl<T> UndoableAction for Action<T> where for<'a> T : 'a + Clone
{
    type Undo = Action<T>;
    type Context<'a>= Vec<T>;
    type Output<'a> = ();
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        match self
        {
            Action::Push(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::Pop)); }
            Action::Pop(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::Push)); }
            Action::Clear(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::MemReplace)); }
            Action::Set(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::Replace)); }
            Action::Replace(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::Replace)); }
            Action::Swap(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::Swap)); }
            Action::MemReplace(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::MemReplace)); }
            Action::MemTake(v) => { v.execute_and_forget_in(context, &mut stack.handle(Action::MemReplace)); }
        };
    }
}

pub trait ActionExtension<T, S> where for<'a> T: 'a + Clone, S: UndoStack<Action<T>>
{
    fn push_action(&mut self, value: T, stack : &mut S);

    fn pop_action(&mut self, stack : &mut S) -> Option<T>;
    fn pop_action_and_forget(&mut self, stack : &mut S);

    //fn clear_action(&mut self, stack : &mut S);

    /*
    /// Panics if `idx` is out of bounds.
    /// 
    /// Also known as `replace_action_and_forget`
    fn set_action(&mut self, idx : usize, value : T, stack : &mut S) { self.try_set_action(idx, value, stack).unwrap(); }
    /// Panics if `idx` is out of bounds.
    /// 
    /// to forget it, use `set_action()`
    fn replace_action(&mut self, idx : usize, value : T, stack : &mut S) -> T { self.try_replace_action(idx, value, stack).unwrap() }

    /// Panics if `idx` is out of bounds.
    fn swap_action(&mut self, i : usize, j : usize, stack : &mut S) { self.try_swap_action(i, j, stack).unwrap(); }

     
    /// Also known as `try_replace_action_and_forget`
    fn try_set_action(&mut self, idx : usize, value : T, stack : &mut S) -> Result<(), ()>;
    /// to forget it, use `try_set_action()`
    fn try_replace_action(&mut self, idx : usize, value : T, stack : &mut S) -> Result<T, ()>;
    */

    // Todo move it at the same place at set_action
    // fn try_swap_action(&mut self, i : usize, j : usize, stack : &mut S) -> Result<(), std::slice::GetDisjointMutError>;
}


impl<T, S> ActionExtension<T, S> for Vec<T> where for<'a> T: 'a + Clone, S: UndoStack<Action<T>>
{
    fn push_action(&mut self, value: T, stack: &mut S)
    { Push::new(value).execute_in(self, &mut stack.handle(Action::Pop)) }
    
    fn pop_action (&mut self, stack: &mut S) -> Option<T> 
    { Pop::new().execute_in(self, &mut stack.handle(Action::Push)) }

    fn pop_action_and_forget(&mut self, stack : &mut S) 
    { Pop::new().execute_and_forget_in(self, &mut stack.handle(Action::Push)) }

    /* 
    fn clear_action(&mut self, stack : &mut S)  
    { Clear::new().execute_in(self, &mut stack.handle(Action::MemReplace)) }
    
    /* 
    fn try_set_action(&mut self, idx : usize, value : T, stack : &mut S) -> Result<(), ()> 
    { Set::new(idx, value).execute_in(self, &mut stack.handle(Action::Replace)) }
    
    fn try_replace_action(&mut self, idx : usize, value : T, stack : &mut S) -> Result<T, ()> 
    { Replace::new(idx, value).execute_in(self, &mut stack.handle(Action::Replace)) }
    */

    fn try_swap_action(&mut self, i : usize, j : usize, stack : &mut S) -> Result<(), std::slice::GetDisjointMutError> 
    { Swap::new(i, j).execute_in(self, &mut stack.handle(Action::Swap)) }
     */
}
