use hexga_undo_redo::prelude::*;

fn main()
{
    let mut v = vec![];
    let mut actions = vec![];

    v.push_action("foo", &mut actions);
    v.push_action("bar", &mut actions);

    println!("before:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);
    println!();

    let maybe_popped = v.pop_action(&mut actions);
    if let Some(popped) = maybe_popped 
    {
        println!("{popped} was popped !");
    }
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);
    println!();

    v.undo(&mut actions);
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);

    v.undo(&mut actions);
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);
}