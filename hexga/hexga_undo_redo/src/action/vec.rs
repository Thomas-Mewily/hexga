use std::fmt::Debug;
use std::hash::Hash;

use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pop<T>(pub PhantomData<T>);
impl<T> Default for Pop<T>{ fn default() -> Self { Self(PhantomData) } }

impl<T> Debug for Pop<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pop<{}>", std::any::type_name::<T>())
    }
}

impl<T> UndoAction for Pop<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'a> = Option<T>;

    fn execute<'a, U>(self, context : &'a mut Self::Context, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::ActionSet> 
    {
        context.pop().map(|v| { undo.push(|| Action::Push(Push(v.clone()))); v })
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Push<T>(pub T);
impl<T> UndoAction for Push<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'ctx> = ();
    
    fn execute<'ctx, U>(self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::ActionSet> 
    {
        context.push(self.0);
        undo.push(|| Action::Pop(Pop(PhantomData)));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clear<T>(pub PhantomData<T>);

impl<T> Debug for Clear<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clear<{}>", std::any::type_name::<T>())
    }
}
impl<T> Default for Clear<T>{ fn default() -> Self { Self(PhantomData) } }

impl<T> UndoAction for Clear<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'ctx> = ();
    
    fn execute<'ctx, U>(self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::ActionSet> 
    {
        if !context.is_empty()
        {
            undo.push(|| Action::Set(Set(std::mem::take(context))));
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Set<T>(pub Vec<T>);
impl<T> UndoAction for Set<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'ctx> = ();
    
    fn execute<'ctx, U>(self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::ActionSet> 
    {
        *context = self.0;
        if !context.is_empty()
        {
            undo.push(|| Action::Clear(Clear(PhantomData)));
        }
    }
}




pub struct Swap<Idx,T>(Idx, T::Output) where T : GetIndexMut<Idx>;

impl<Idx,T> Copy for Swap<Idx,T> where T : GetIndexMut<Idx>, Idx : Copy, T::Output : Copy {}
impl<Idx,T> Clone for Swap<Idx,T> where T : GetIndexMut<Idx>, Idx : Clone, T::Output : Clone
{
    fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
}

impl<Idx,T> Eq for Swap<Idx,T> where T : GetIndexMut<Idx>, Idx : Eq, T::Output : Eq {}
impl<Idx,T> PartialEq for Swap<Idx,T> where T : GetIndexMut<Idx>, Idx : PartialEq, T::Output : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<Idx,T> Hash for Swap<Idx,T> where T : GetIndexMut<Idx>, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); self.1.hash(state); } }
impl<Idx,T> Debug for Swap<Idx,T> where T : GetIndexMut<Idx>, Idx : Debug, T::Output : Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Swap").field(&self.0).field(&&self.1).finish()
    }
}

impl<Idx,T> UndoAction for Swap<Idx,T> where T: GetIndexMut<Idx>, for<'a> Idx : 'a + Clone, for<'a> T::Output : 'a + Clone, T::Output : Sized 
{
    type ActionSet = Self;
    type Context=T;
    type Output<'ctx> = Result<(), ()>;
    
    fn execute<'ctx, U>(mut self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::ActionSet> 
    {
        let Swap(idx, value) = &mut self;
        match context.get_mut(idx.clone()) // I don't like this clone. Imagine it on a HashMap<String, ...>...
        {
            Some(v) => 
            {
                std::mem::swap(value, v);
                undo.push(|| self);
                Ok(())
            },
            None => Err(()),
        }
    }
}


/* 
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Swap<T>(pub usize, pub usize, PhantomData<T>);

impl<T> UndoAction for Replace<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'ctx> = ();
    
    fn execute<'ctx, U>(self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::ActionSet> 
    {
        *context = self.0;
        if !context.is_empty()
        {
            undo.push(Action::Clear(Clear(PhantomData)));
        }
    }
}*/

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Action<T>
{
    Pop(Pop<T>),
    Push(Push<T>),
    Clear(Clear<T>),
    Set(Set<T>),
    Swap(Swap<usize, Vec<T>>),
    //Swap()
}
impl<T> UndoAction for Action<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'ctx> = ();
    
    fn execute<'ctx, U>(self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::ActionSet> {
        match self
        {
            Action::Pop(v) => { v.execute(context, undo); }
            Action::Push(v) => { v.execute(context, undo); }
            Action::Clear(v) => { v.execute(context, undo); }
            Action::Set(v) => { v.execute(context, undo); }
            Action::Swap(v) => { let _ = v.execute(context, &mut undo.handle(Self::Swap)); }
        };
    }
}

pub trait ActionExtension<T, U> where for<'a> T: 'a + Clone, U: UndoStack<Action<T>>
{
    fn push_action(&mut self, value: T, undo : &mut U);
    fn pop_action(&mut self, undo : &mut U) -> Option<T>;
    fn clear_action(&mut self, undo : &mut U);
    fn swap_action(&mut self, idx : usize, value : T, undo : &mut U) -> Result<(), ()>;
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
    fn push_action(&mut self, value: T, undo: &mut U) {
        Push(value).execute(self, undo)
    }

    fn pop_action(&mut self, undo: &mut U) -> Option<T>
    {
        Pop::default().execute(self, undo)
    }

    fn clear_action(&mut self, undo : &mut U) 
    {
        Clear::default().execute(self, undo)
    }
    
    fn swap_action(&mut self, idx : usize, value : T, undo : &mut U) -> Result<(), ()> {
        Swap(idx, value).execute(self, &mut undo.handle(Action::Swap))
    }
}

