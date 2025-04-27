use hexga_undo_redo::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum PushOrPop<T> where T : Clone
{
    Pop,
    Push(T),
}

impl<T> UndoableAction for PushOrPop<T> where for<'a> T : 'a + Clone
{
    type Undo = Self;
    type Context<'a> = Vec<T>;
    type Output<'a> = ();

    fn execute_in<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> {
        match self
        {
            PushOrPop::Pop => match context.pop()
            {
                Some(v) => undo.push_undo_action(||Self::Push(v)),
                None => (),
            },
            PushOrPop::Push(value) => { context.push(value); undo.push_undo_action(||Self::Pop); }
        };
    }
}

fn main()
{
    let mut v = vec![];
    let mut actions = vec![];

    PushOrPop::Push("foo").execute_in(&mut v, &mut actions);
    PushOrPop::Push("bar").execute_in(&mut v, &mut actions);

    println!("before:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);
    println!();

    v.undo(&mut actions);

    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);
}