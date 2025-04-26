use crate::*;


pub struct Swap<T> { phantom : PhantomData<T> }

impl<T> Swap<T> { pub const fn new() -> Self { Self { phantom: PhantomData }} }

impl<T> Clone   for Swap<T> { fn clone(&self) -> Self { Self::new() } }
impl<T> Copy    for Swap<T> {}

impl<T> PartialEq   for Swap<T> { fn eq(&self, _: &Self) -> bool { true } }
impl<T> Eq          for Swap<T> {}
impl<T> Hash        for Swap<T> { fn hash<H: std::hash::Hasher>(&self, _: &mut H) { } }
impl<T> Default     for Swap<T> { fn default() -> Self { Self::new() } }
impl<T> Debug       for Swap<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Swap") } }

impl<T> UndoAction for Swap<T> where for<'a> T: 'a
{
    type Undo = Self;
    type Context<'a>= (&'a mut T, &'a mut T);
    type Output<'a> = ();
    
    fn execute<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo> 
    {
        std::mem::swap(&mut context.0, &mut context.1);
        undo.push_undo_action(|| self);
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

impl<T> UndoAction for Replace<T> where for<'a> T: 'a + Clone
{
    type Undo = Replace<T>;
    type Context<'a> = T;
    type Output<'ctx> = T;
    
    fn execute<'a, U>(mut self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo> 
    {
        std::mem::swap(&mut self.src, context);
        undo.push_undo_action(|| Replace::new(self.src.clone()));
        self.src
    }

    fn execute_and_forget<'a,U>(mut self, context : &mut Self::Context<'a>, undo : &mut U) where U : ActionStack<Self::Undo> {
        std::mem::swap(&mut self.src, context);
        undo.push_undo_action(|| Replace::new(self.src));
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

impl<T> UndoAction for Take<T> where for<'a> T: 'a + Default + Clone
{
    type Undo = Replace<T>;
    type Context<'a>= T;
    type Output<'a> = T;
    
    fn execute<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo> 
    { Replace::new(___()).execute(context, undo) }

    fn execute_and_forget<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) where U : ActionStack<Self::Undo> 
    { Replace::new(___()).execute_and_forget(context, undo) }
}

pub fn take<T, U>(value : &mut T, undo : &mut U) -> T where for<'a> T: 'a + Default + Clone, U: ActionStack<Replace<T>>
{
    Take::new().execute(value, undo)
}

pub fn take_and_forget<T, U>(value : &mut T, undo : &mut U) where for<'a> T: 'a + Default + Clone, U: ActionStack<Replace<T>>
{
    Take::new().execute_and_forget(value, undo)
}


pub fn swap<T, U>(a : &mut T, b : &mut T, undo : &mut U) where for<'a> T: 'a, U: ActionStack<Swap<T>>
{
    Swap::new().execute(&mut (a,b), undo);
}

pub fn replace<T, U>(dest : &mut T, src : T, undo : &mut U) -> T where for<'a> T: 'a + Clone, U: ActionStack<Replace<T>>
{
    Replace::new(src).execute(dest, undo)
}

pub fn replace_and_forget<T, U>(dest : &mut T, src : T, undo : &mut U) where for<'a> T: 'a + Clone, U: ActionStack<Replace<T>>
{
    Replace::new(src).execute_and_forget(dest, undo)
}