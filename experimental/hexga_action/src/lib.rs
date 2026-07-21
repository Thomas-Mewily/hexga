use std::{collections::VecDeque, fmt::Debug};

pub trait Action<Ctx>
{
    type Ok;
    type Err: Debug + 'static;

    fn execute(&mut self, ctx: &mut Ctx) -> Result<Self::Ok, Self::Err>;
    fn undo(&mut self, ctx: &mut Ctx) -> Result<Self::Ok, Self::Err> { self.execute(ctx) }
}

pub trait ActionMarkerGroup
{
    /// Is the begining action marker.
    fn is_begin(&self) -> bool;
    /// Is the end action marker.
    fn is_end(&self) -> bool;
    /// A marker to note the begining of a group of 0..N actions : `(`.
    fn begin() -> Self;
    /// A marker to note the ending of a group of 0..N actions : `)`.
    fn end() -> Self;
}


pub trait ActionUndoRedo<A, Ctx>
    where A: Action<Ctx>
{
    fn can_redo(&mut self) -> bool;
    /// Redo the current action.
    fn redo(&mut self, ctx: &mut Ctx) -> Result<A::Ok, Option<A::Err>>;
    /// Redo the `nb` next actions in an atomic way.
    fn redo_by(&mut self, nb: usize, ctx: &mut Ctx) -> Result<(), Option<A::Err>> 
    {
        for ok_until in 0..nb 
        { 
            match self.redo(ctx) 
            {
                Ok(_) => continue,
                Err(e) => 
                {
                    for _ in 0..ok_until
                    {
                        self.undo(ctx).expect("logic error in the Action impl");
                    }
                    return Err(e);
                },
            }
        } 
        Ok(()) 
    }

    fn can_undo(&mut self) -> bool;

    /// Undo the current action.
    fn undo(&mut self, ctx: &mut Ctx) -> Result<A::Ok, Option<A::Err>>;
    /// Undo the `nb` next actions in an atomic way.
    fn undo_by(&mut self, nb: usize, ctx: &mut Ctx) -> Result<(), Option<A::Err>> 
    { 
        for ok_until in 0..nb 
        { 
            match self.undo(ctx) 
            {
                Ok(_) => continue,
                Err(e) => 
                {
                    for _ in 0..ok_until
                    {
                        self.redo(ctx).expect("logic error in the Action impl");
                    }
                    return Err(e);
                },
            }
        } 
        Ok(()) 
    }
}

pub struct UndoRedo<A>
{
    pub undo: Vec<A>,
    pub redo: Vec<A>,
}

pub struct UndoRedoIn<A, Ctx>
    where A: Action<Ctx>
{
    pub ctx : Ctx,
    pub undo_redo : UndoRedo<A>,
}

impl<A, Ctx> ActionUndoRedo<A, Ctx> for UndoRedo<A>
    where A: Action<Ctx>
{
    fn can_redo(&mut self) -> bool {
        !self.redo.is_empty()
    }

    fn redo(&mut self, ctx: &mut Ctx) -> Result<A::Ok, Option<A::Err>> {
        match self.redo.pop()
        {
            Some(mut action) => 
            {
                match action.execute(ctx)
                {
                    Ok(o) => { self.undo.push(action); Ok(o) },
                    Err(e) => Err(Some(e)),
                }
            },
            None => Err(None),
        }
    }

    fn can_undo(&mut self) -> bool {
        !self.undo.is_empty()
    }

    fn undo(&mut self, ctx: &mut Ctx) -> Result<A::Ok, Option<A::Err>> {
        match self.undo.pop()
        {
            Some(mut action) => 
            {
                match action.execute(ctx)
                {
                    Ok(o) => { self.redo.push(action); Ok(o) },
                    Err(e) => Err(Some(e)),
                }
            },
            None => Err(None),
        }
    }
}


/*

pub trait ActionManager<A, Ctx>
    where A: Action<Ctx>
{
    fn execute(&mut self, action : A, ctx: &mut Ctx) -> Result<A::Ok, A::Err>;
    fn undo(&mut self, action : A, ctx: &mut Ctx) -> Result<A::Ok, A::Err>;
}
*/
/*
pub trait ActionManager<A> : Sized
    where A: Action<Self>
{
    fn execute
}
*/


/*
Swap
*/


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PushPop<T>
{
    pub value: Option<T>
}
impl<T> Action<Vec<T>> for PushPop<T>
{
    type Ok=();
    type Err=();

    fn execute(&mut self, ctx: &mut Vec<T>) -> Result<Self::Ok, Self::Err> {
        match self.value.take()
        {
            Some(v) => { ctx.push(v); Ok(()) },
            None => { self.value = Some(ctx.pop().ok_or(())?); Ok(()) },
        }
    }
}
impl<T> Action<VecDeque<T>> for PushPop<T>
{
    type Ok=();
    type Err=();

    fn execute(&mut self, ctx: &mut VecDeque<T>) -> Result<Self::Ok, Self::Err> {
        match self.value.take()
        {
            Some(v) => { ctx.push_back(v); Ok(()) },
            None => { self.value = Some(ctx.pop_back().ok_or(())?); Ok(()) },
        }
    }
}