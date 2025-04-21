use hexga_undo_redo::prelude::*;

fn main()
{
    let mut values = vec![];
    let mut undo_action = vec![];

    values.push_action("foo", &mut undo_action);
    values.push_action("bar", &mut undo_action);

    let popped = values.pop_action(&mut undo_action);
    assert_eq!(popped, Some("bar"));
    assert_eq!(values, vec!["foo"]);

    values.undo_action(undo_action.pop().unwrap());
    assert_eq!(values, vec!["foo", "bar"]);

    values.undo_action(undo_action.pop().unwrap());
    assert_eq!(values, vec!["foo"]);

    values.undo_action(undo_action.pop().unwrap());
    assert!(values.is_empty());
}