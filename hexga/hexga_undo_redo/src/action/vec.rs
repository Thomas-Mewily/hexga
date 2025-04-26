use crate::*;
pub use action::collection;
pub use action::mem;

pub type Clear  <T> = collection::Clear       <Vec<T>       >;
pub type Set    <T> = collection::SetIndex    <Vec<T>, usize>;
pub type Replace<T> = collection::ReplaceIndex<Vec<T>, usize>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pop<T>(PhantomData<T>);
impl<T> Default for Pop<T>{ fn default() -> Self { Self(PhantomData) } }
impl<T> Debug for Pop<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Pop") } }

impl<T> UndoAction for Pop<T> where for<'a> T: 'a + Clone
{
    type Undo = Push<T>;
    type Context<'a>= &'a mut Vec<T>;
    type Output<'a> = Option<T>;

    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        context.pop().map(|v| { undo.push(|| Push(v.clone())); v })
    }

    fn execute_and_forget<'a, U>(self, context : Self::Context<'a>, undo : &mut U) where U : UndoStack<Self::Undo> {
        context.pop().map(|v| { undo.push(|| Push(v)); });
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Push<T>(pub T);
impl<T> UndoAction for Push<T> where for<'a> T: 'a + Clone
{
    type Undo = Pop<T>;
    type Context<'a>= &'a mut Vec<T>;
    type Output<'a> = ();
    
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        context.push(self.0);
        undo.push(|| Pop::___());
    }
}

pub type Swap<T> = collection::SwapIndex<Vec<T>, usize>;

impl<T> UndoAction for Swap<T> where for<'a> T: 'a
{
    type Undo = Self;
    type Context<'a>= &'a mut Vec<T>;
    type Output<'a> = Result<(), std::slice::GetDisjointMutError>;
    
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        // Todo : add fn is_useful(&self) -> bool; and fn is_useful_on(&self, &ctx) -> bool; in this trait.
        //if self.i() != self.j() { return; }
        context.get_disjoint_mut([self.i(), self.j()]).map(|[a,b]|
            {
                std::mem::swap(a, b);
                undo.push(|| self);
            })
    }
}



#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Action<T> where for<'a> T : 'a + Clone
{
    Push(Push<T>),
    Pop(Pop<T>),

    Clear(Clear<T>),

    Set(Set<T>),
    Replace(Replace<T>), 
    Swap(Swap<T>),

    MemReplace(mem::Replace<Vec<T>>),
    MemTake(mem::Take<Vec<T>>),
}
impl<T> UndoAction for Action<T> where for<'a> T : 'a + Clone
{
    type Undo = Action<T>;
    type Context<'a>= &'a mut Vec<T>;
    type Output<'a> = ();
    
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        match self
        {
            Action::Push(v) => { v.execute_and_forget(context, &mut undo.handle(Action::Pop)); }
            Action::Pop(v) => { v.execute_and_forget(context, &mut undo.handle(Action::Push)); }
            Action::Clear(v) => { v.execute_and_forget(context, &mut undo.handle(Action::MemReplace)); }
            Action::Set(v) => { v.execute_and_forget(context, &mut undo.handle(Action::Replace)); }
            Action::Replace(v) => { v.execute_and_forget(context, &mut undo.handle(Action::Replace)); }
            Action::Swap(v) => { v.execute_and_forget(context, &mut undo.handle(Action::Swap)); }
            Action::MemReplace(v) => { v.execute_and_forget(context, &mut undo.handle(Action::MemReplace)); }
            Action::MemTake(v) => { v.execute_and_forget(context, &mut undo.handle(Action::MemReplace)); }
        };
    }
}

pub trait ActionExtension<T, U> where for<'a> T: 'a + Clone, U: UndoStack<Action<T>>
{
    fn push_action(&mut self, value: T, undo : &mut U);

    fn pop_action(&mut self, undo : &mut U) -> Option<T>;
    fn pop_action_and_forget(&mut self, undo : &mut U);

    fn clear_action(&mut self, undo : &mut U);

    /// Panics if `idx` is out of bounds.
    /// 
    /// Also known as `replace_action_and_forget`
    fn set_action(&mut self, idx : usize, value : T, undo : &mut U) { self.try_set_action(idx, value, undo).unwrap(); }
    /// Panics if `idx` is out of bounds.
    /// 
    /// to forget it, use `set_action()`
    fn replace_action(&mut self, idx : usize, value : T, undo : &mut U) -> T { self.try_replace_action(idx, value, undo).unwrap() }

    /// Panics if `idx` is out of bounds.
    fn swap_action(&mut self, i : usize, j : usize, undo : &mut U) { self.try_swap_action(i, j, undo).unwrap(); }

    /// Also known as `try_replace_action_and_forget`
    fn try_set_action(&mut self, idx : usize, value : T, undo : &mut U) -> Result<(), ()>;
    /// to forget it, use `try_set_action()`
    fn try_replace_action(&mut self, idx : usize, value : T, undo : &mut U) -> Result<T, ()>;

    fn try_swap_action(&mut self, i : usize, j : usize, undo : &mut U) -> Result<(), ()>;
}

/* 
pub trait SwapExtension<Idx, U> : GetIndexMut<Idx> + Sized 
    where U : UndoStack<Swap<Idx, Self>>, for<'a> Self::Output : 'a + Sized + Clone, for<'a> Idx : 'a + Clone
{
    fn swap_action(&mut self, idx : Idx, value : Self::Output, undo: &mut U) -> Result<(), ()>
    {
        Swap(idx, value).execute(self, undo)
    }
}
*/

impl<T, U> ActionExtension<T, U> for Vec<T> where for<'a> T: 'a + Clone, U: UndoStack<Action<T>>
{
    fn push_action(&mut self, value: T, undo: &mut U)
    { Push(value)   .execute(self, &mut undo.handle(Action::Pop )) }
    
    fn pop_action (&mut self,           undo: &mut U) -> Option<T> 
    { Pop::default().execute(self, &mut undo.handle(Action::Push)) }

    fn pop_action_and_forget(&mut self, undo : &mut U) 
    { Pop::default().execute_and_forget(self, &mut undo.handle(Action::Push)) }

    fn clear_action(&mut self, undo : &mut U)  
    { Clear::default().execute(self, &mut undo.handle(Action::MemReplace)) }
    
    /* 
    fn try_set_action(&mut self, idx : usize, value : T, undo : &mut U) -> Result<(), ()> 
    { Set() ::default().execute(self, &mut undo.handle(Action::MemReplace)) }
    
    fn try_replace_action(&mut self, idx : usize, value : T, undo : &mut U) -> Result<T, ()> {
        todo!()
    }
    
    fn try_swap_action(&mut self, i : usize, j : usize, undo : &mut U) -> Result<(), ()> {
        todo!()
    }
    */


}
