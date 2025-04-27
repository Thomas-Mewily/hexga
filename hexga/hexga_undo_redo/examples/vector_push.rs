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

    v.undo(&mut actions);
    println!("after:");
    println!("vector: {:?}", v);
    println!("undo_action: {:?}", actions);
}