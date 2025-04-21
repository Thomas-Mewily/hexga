use hexga_undo_redo::prelude::*;
use hexga_undo_redo::action::vec;


#[derive(Default, PartialEq, Eq, Debug)]
pub struct Data
{
    // Note that both field have the **same** type, 
    // but we need to but there needs to be a way to differentiate
    // if a action is done on `odd` or `even`
    odd : Vec<i32>,
    even : Vec<i32>,
}

#[derive(Debug)]
pub enum DataAction
{
    Odd (vec::Action<i32>), // Action done on Odd
    Even(vec::Action<i32>), // Action done on Even
}

impl UndoAction for DataAction
{
    type ActionSet = Self;
    type Context = Data;
    type Output<'a> = ();

    fn execute<'a, U>(self, context : &'a mut Self::Context, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::ActionSet> 
    {
        match self
        {
            DataAction::Odd(vec_action) => vec_action.execute(&mut context.odd, &mut undo.handle(DataAction::Even)),
            DataAction::Even(vec_action) => vec_action.execute(&mut context.even, &mut undo.handle(DataAction::Odd)),
        }
    }
}

impl Data
{
    pub fn push_action(&mut self, value : i32, undo : &mut impl UndoStack<DataAction>)
    {
        if value % 2 == 0
        {
            self.even.push_action(value, &mut undo.handle(DataAction::Even));
        }else
        {
            self.odd.push_action(value, &mut undo.handle(DataAction::Odd));
        }
    }
}

fn main() 
{
    let mut d = Data::default();
    let mut undo_action = Vec::new();

    d.push_action(42, &mut undo_action);
    d.push_action(43, &mut undo_action);
    println!("before");
    println!("data: {:?}", d);
    println!("undo_action: {:?}", undo_action);
    println!();

    d.undo_action(undo_action.pop().unwrap());
    println!("after undoing 1 action");
    println!("data: {:?}", d);
    println!("undo_action: {:?}", undo_action);
    println!();
    

    d.undo_action(undo_action.pop().unwrap());
    println!("after undoing 2 action");
    println!("data: {:?}", d);
    println!("undo_action: {:?}", undo_action);
}
