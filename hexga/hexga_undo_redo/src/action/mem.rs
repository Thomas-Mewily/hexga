use crate::*;


// Todo : ReplaceBox if the src is big

pub struct Swap<T> { phantom : PhantomData<T> }

impl<T> Swap<T> { pub const fn new() -> Self { Self { phantom: PhantomData }} }

impl<T> Clone   for Swap<T> { fn clone(&self) -> Self { Self::new() } }
impl<T> Copy    for Swap<T> {}

impl<T> PartialEq   for Swap<T> { fn eq(&self, _: &Self) -> bool { true } }
impl<T> Eq          for Swap<T> {}
impl<T> Hash        for Swap<T> { fn hash<H: std::hash::Hasher>(&self, _: &mut H) { } }
impl<T> Default     for Swap<T> { fn default() -> Self { Self::new() } }
impl<T> Debug       for Swap<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Swap") } }

impl<T> UndoableAction for Swap<T> where for<'a> T: 'a
{
    type Undo = Self;
    type Context<'a>= (&'a mut T, &'a mut T);
    type Output<'a> = ();
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        std::mem::swap(&mut context.0, &mut context.1);
        stack.push_undo_action(|| self);
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Replace<T> { pub src : T }
impl<T> Debug for Replace<T> where T : Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Replace({:?})", self.src) }
}

impl<T> Replace<T> 
{ 
    pub const fn new(src : T) -> Self { Self { src }} 
}

impl<T> Default for Replace<T> where T : Default { fn default() -> Self { Self::new(___()) } }

impl<T> UndoableAction for Replace<T> where for<'a> T: 'a + Clone
{
    type Undo = Replace<T>;
    type Context<'a> = T;
    type Output<'ctx> = T;
    
    fn execute_in<'a, S>(mut self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        std::mem::swap(&mut self.src, context);
        stack.push_undo_action(|| Replace::new(self.src.clone()));
        self.src
    }

    fn execute_and_forget_in<'a,S>(mut self, context : &mut Self::Context<'a>, stack : &mut S) where S : UndoStack<Self::Undo> {
        std::mem::swap(&mut self.src, context);
        stack.push_undo_action(|| Replace::new(self.src));
    }
}




pub struct Take<T> { phantom : PhantomData<T> }
impl<T> Take<T> { pub const fn new() -> Self { Self { phantom: PhantomData }} }

impl<T> Clone       for Take<T> { fn clone(&self) -> Self { Self::new() } }
impl<T> Copy        for Take<T> { }
impl<T> PartialEq   for Take<T> { fn eq(&self, _: &Self) -> bool { true } }
impl<T> Eq          for Take<T> { }
impl<T> Hash        for Take<T> { fn hash<H: std::hash::Hasher>(&self, _: &mut H) { } }
impl<T> Debug       for Take<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Take") } }
impl<T> Default     for Take<T> { fn default() -> Self { Self::new() } }

impl<T> UndoableAction for Take<T> where for<'a> T: 'a + Default + Clone
{
    type Undo = Replace<T>;
    type Context<'a>= T;
    type Output<'a> = T;
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    { Replace::new(___()).execute_in(context, stack) }

    fn execute_and_forget_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) where S : UndoStack<Self::Undo> 
    { Replace::new(___()).execute_and_forget_in(context, stack) }
}

pub fn take<T, S>(value : &mut T, stack : &mut S) -> T where for<'a> T: 'a + Default + Clone, S: UndoStack<Replace<T>>
{
    Take::new().execute_in(value, stack)
}

pub fn take_and_forget<T, S>(value : &mut T, stack : &mut S) where for<'a> T: 'a + Default + Clone, S: UndoStack<Replace<T>>
{
    Take::new().execute_and_forget_in(value, stack)
}

pub fn swap<T, S>(a : &mut T, b : &mut T, stack : &mut S) where for<'a> T: 'a, S: UndoStack<Swap<T>>
{
    Swap::new().execute_in(&mut (a,b), stack);
}

pub fn replace<T, S>(dest : &mut T, src : T, stack : &mut S) -> T where for<'a> T: 'a + Clone, S: UndoStack<Replace<T>>
{
    Replace::new(src).execute_in(dest, stack)
}

pub fn replace_and_forget<T, S>(dest : &mut T, src : T, stack : &mut S) where for<'a> T: 'a + Clone, S: UndoStack<Replace<T>>
{
    Replace::new(src).execute_and_forget_in(dest, stack)
}