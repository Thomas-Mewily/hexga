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

    v.undo_action(undo_action.pop().unwrap());
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", undo_action);
}