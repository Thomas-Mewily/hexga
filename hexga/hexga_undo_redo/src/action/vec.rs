use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Pop<T>(pub PhantomData<T>);
impl<T> Default for Pop<T>{ fn default() -> Self { Self(PhantomData) } }

impl<T> UndoAction for Pop<T> where for<'a> T: 'a + Clone
{
    type ActionSet = Action<T>;
    type Context=Vec<T>;
    type Output<'a> = Option<T>;

    fn execute_without_undo<'a>(self, context : &'a mut Self::Context) -> Self::Output<'a> 
    {
        context.pop()
    }

    fn execute<'a, U>(self, context : &'a mut Self::Context, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::ActionSet> 
    {
        context.pop().map(|v| { undo.push(Action::Push(Push(v.clone()))); v })
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
        undo.push(Action::Pop(Pop(PhantomData)));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Clear<T>(pub PhantomData<T>);
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
            undo.push(Action::Replace(Replace(std::mem::take(context))));
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Replace<T>(pub Vec<T>);
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
    Replace(Replace<T>),
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
            Action::Push(v) => { v.execute(context, undo); },
            Action::Clear(v) => { v.execute(context, undo); }
            Action::Replace(v) => { v.execute(context, undo); }
        };
    }
}

pub trait IVecAction<T, U> where for<'a> T: 'a + Clone, U: UndoStack<Action<T>>
{
    fn push_action(&mut self, value: T, undo : &mut U);
    fn pop_action(&mut self, undo : &mut U) -> Option<T>;
    fn clear_action(&mut self, undo : &mut U);
    //fn swap_action(&mut self, undo : &mut U);
}

impl<T, U> IVecAction<T, U> for Vec<T> where for<'a> T: 'a + Clone, U: UndoStack<Action<T>>
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
}

