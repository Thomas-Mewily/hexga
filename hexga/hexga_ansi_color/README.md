## HexGa Ansi Color

A minimal package for using the Ansi Color :

Provides optional support for [Serde](https://docs.rs/serde/latest/serde/) (serialization / deserialization) when the "serde" feature is enabled.

```rust
use hexga_ansi_color::*;

println!("{}I'm green{}", AnsiColor::GREEN, AnsiColor::RESET);
println!("{}I'm red{}", AnsiColor::new_foreground(AnsiColorKind::Red), AnsiColor::RESET);
println!("{}White on magenta background{}", AnsiColor::new(AnsiColorKind::Magenta, AnsiColorLayer::Background), AnsiColor::RESET);
```
![image](https://github.com/user-attachments/assets/c1bd8fd3-f10f-4c92-b2cd-e012a4c61b33)

```rust
pub enum AnsiColorKind
{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
}

pub enum AnsiColorLayer 
{
    Foreground,
    Background,
}

pub struct AnsiColor
{
    pub color : AnsiColorKind,
    pub layer : AnsiColorLayer,
}
```

Based on [minimal_ansi_color](https://crates.io/crates/minimal_ansi_color), but integrated to hexga.

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.