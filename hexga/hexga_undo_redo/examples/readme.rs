use hexga_undo_redo::prelude::*;

fn main()
{
    let mut values = vec![];
    let mut actions = vec![];

    values.push_action("foo", &mut actions);
    values.push_action("bar", &mut actions);

    let popped = values.pop_action(&mut actions);
    assert_eq!(popped, Some("bar"));
    assert_eq!(values, vec!["foo"]);

    values.undo(&mut actions);
    assert_eq!(values, vec!["foo", "bar"]);

    values.undo(&mut actions);
    assert_eq!(values, vec!["foo"]);

    values.undo(&mut actions);
    assert!(values.is_empty());
}