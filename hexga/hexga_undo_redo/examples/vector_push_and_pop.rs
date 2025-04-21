use hexga_undo_redo::prelude::*;

fn main()
{
    let mut v = vec![];
    let mut undo_action = vec![];

    v.push_action("foo", &mut undo_action);
    v.push_action("bar", &mut undo_action);

    println!("before:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);
    println!();

    let maybe_popped = v.pop_action(&mut undo_action);
    if let Some(popped) = maybe_popped 
    {
        println!("{popped} was popped !");
    }
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);
    println!();

    v.undo_action(undo_action.pop().unwrap());
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);

    v.undo_action(undo_action.pop().unwrap());
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);
}