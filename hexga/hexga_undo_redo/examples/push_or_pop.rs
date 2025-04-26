use hexga_undo_redo::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum PushOrPop<T> where T : Clone
{
    Pop,
    Push(T),
}

impl<T> UndoAction for PushOrPop<T> where T : Clone
{
    type Undo = Self;
    type Context = Vec<T>;
    type Output<'a> = ();

    fn execute<'a, U>(self, context : &'a mut Self::Context, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> {
        match self
        {
            PushOrPop::Pop => match context.pop()
            {
                Some(v) => undo.push(||Self::Push(v)),
                None => (),
            },
            PushOrPop::Push(value) => { context.push(value); undo.push(||Self::Pop); }
        };
    }
}

fn main()
{
    let mut v = vec![];
    let mut undo_action = vec![];

    PushOrPop::Push("foo").execute(&mut v, &mut undo_action);
    PushOrPop::Push("bar").execute(&mut v, &mut undo_action);

    println!("before:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);
    println!();

        v.undo_action(undo_action.pop().unwrap());
        // same as :
        // undo_action.pop().unwrap().execute_without_undo(&mut v);

    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);
}