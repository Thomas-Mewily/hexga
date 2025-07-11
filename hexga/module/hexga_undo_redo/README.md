## HexGa Undo Redo

Fundamentally, this crate don't directly handle `command` but lower building block call `action`.
The implementation for doing and undoing an action is also the same, because it just a matter of executing an different action to undo.


A `command` is just a composition of `action`.

This crate then also provide abstraction to execute command. The main challenge of command is that a command can be TODO

An implementation of the [Command pattern](https://en.wikipedia.org/wiki/Command_pattern) that can be used to program undo / redo !


### Usage Example

Each action can also return a result, which allows mutating the receiver while getting the return value, like when popping a value from a vector !

```rust
use hexga_undo_redo::prelude::*;

fn main()
{
    let mut values = vec![];
    let mut undo_action = vec![];

    values.push_action("foo", &mut undo_action);
    values.push_action("bar", &mut undo_action);

    let popped = values.pop_action(&mut undo_action);
    // same as :
    // action::vec::Pop::default().execute(&mut value, &mut undo_action)
    assert_eq!(popped, Some("bar"));
    assert_eq!(values, vec!["foo"]);

    values.undo_action(undo_action.pop().unwrap());
    assert_eq!(values, vec!["foo", "bar"]);

    values.undo_action(undo_action.pop().unwrap());
    assert_eq!(values, vec!["foo"]);

    values.undo_action(undo_action.pop().unwrap());
    assert!(values.is_empty());
}
```

### Implementation Example

Unlike other crate like [undo](https://crates.io/crates/undo), when doing a action you can declare zero, one or multiple actions about how to undo your action !

- Pros :
    - Allow some small optimisations, like avoiding emitting an undo action if there is no need.

    - You can define how to undo an action by calling multiple other undo action. Composition become easier, so some complexe action like `replacing all even value by 0 in a vector` become trivial to implement (you can emit multiples `action::vec::Replace` without allocating a new vector for the action)

    - Very conveniant for video game/simulation application.


- Cons :
    - 1 action executed = N actions to undo it (generally 0 or 1), so to undo 1 action you need to use some sentinel value.

```rust
use hexga_undo_redo::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum PushOrPop<T> where T: Clone
{
    Pop,
    Push(T),
}

impl<T> UndoAction for PushOrPop<T> where for<'a> T : 'a + Clone
{
    type Undo = Self;
    type Context<'a> = &'a mut Vec<T>;
    type Output<'a> = ();

    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> {
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
```

### More complicated data ?

This crate also handle the composition of multiple action, even if they have the same type !
(Check the `composite.rs` examples)

```rust
#[derive(Default, PartialEq, Eq, Debug)]
pub struct Data
{
    // Note that both field have the **same** type,
    // but we need to but there needs to be a way to differentiate
    // if a action is done on `odd` or `even`
    odd  : Vec<i32>,
    even : Vec<i32>,
}

pub enum DataAction
{
    Odd (vec::Action<i32>), // Action done on Odd
    Even(vec::Action<i32>), // Action done on Even
}

impl UndoAction for DataAction
{
    type Undo = Self;
    type Context<'a> = &'a mut Data;
    type Output<'a> = ();

    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo>
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
            self.even.push_action(value, &mut undo.handle(DataAction::Even)); // Just like that !
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
    d.undo_action(undo_action.pop().unwrap());
}
```

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.