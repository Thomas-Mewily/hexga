🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.  
It is subject to **breaking changes** in future releases.  
Use it at your own risk, and keep in mind that the API may change in future versions.

# BitFlags

A crate to manage bitflags with enums.

## Features

- Easily create and manage bitflags using enums.
- Provides intuitive methods for adding, removing, and toggling flags.
- Supports bitwise operations (`|`, `&`, `^`, `!`) for combining and manipulating flags.
- Implements iteration over active flags.


### Defining a BitFlags

```rust
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TeamFlag {
    Blue,
    Red,
    Yellow,
}

impl MaxValue for TeamFlag {
    const MAX: Self = Self::Yellow;
}

impl From<TeamFlag> for u8 {
    fn from(value: TeamFlag) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for TeamFlag {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TeamFlag::Blue),
            1 => Ok(TeamFlag::Red),
            2 => Ok(TeamFlag::Yellow),
            _ => Err(()),
        }
    }
}

type Team = BitFlags<TeamFlag, u8>;
```

### Basic Operations

```rust
fn main() 
{
    let mut team = Team::ZERO | TeamFlag::Blue | TeamFlag::Red;
    // same as
    // let mut team = Team::ZERO.added(TeamFlag::Blue).added(TeamFlag::Red);

    team ^= TeamFlag::Red;
    // same as
    // team.toggle(TeamFlag::Red);
    
    assert!(team.have(TeamFlag::Blue));
    assert!(!team.have(TeamFlag::Red));
    assert!(!team.have(TeamFlag::Yellow));

    team |= TeamFlag::Yellow;
    assert!(team.have(TeamFlag::Yellow));
}
```

### Iteration

```rust
fn main() 
{
    let mut teams = Team::ZERO | TeamFlag::Blue | TeamFlag::Red;

    // iteration :
    for team in teams
    {
        
    }
}
```
