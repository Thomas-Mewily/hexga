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

#[derive(Debug, Clone)]
pub enum DataAction
{
    Odd (vec::Action<i32>), // Action done on Odd
    Even(vec::Action<i32>), // Action done on Even
}

impl UndoableAction for DataAction
{
    type Undo = Self;
    type Context<'a> = Data;
    type Output<'a> = ();

    fn execute_in<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        match self
        {
            DataAction::Odd(vec_action) => vec_action.execute_in(&mut context.odd, &mut undo.handle(DataAction::Even)),
            DataAction::Even(vec_action) => vec_action.execute_in(&mut context.even, &mut undo.handle(DataAction::Odd)),
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
    let mut actions = vec![];

    d.push_action(42, &mut actions);
    d.push_action(43, &mut actions);
    println!("before");
    println!("data: {:?}", d);
    println!("undo_action: {:?}", actions);
    println!();

    d.undo(&mut actions);
    println!("after undoing 1 action");
    println!("data: {:?}", d);
    println!("undo_action: {:?}", actions);
    println!();
    

    d.undo(&mut actions);
    println!("after undoing 2 action");
    println!("data: {:?}", d);
    println!("undo_action: {:?}", actions);
}
