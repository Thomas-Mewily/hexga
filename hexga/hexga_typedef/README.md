

## HexGa Typedef

Provide a common typedef for `float`, `int` and `uint`. The precision can be changed with feature flags and can be easily shared across multiple crates.

You must enable one flag for int and for float precision.
See the toml to change the precision :

```toml
[features]
default = ["int_are_32_bits", "float_are_32_bits"]

int_are_8_bits    = []
int_are_16_bits   = []
int_are_32_bits   = []
int_are_64_bits   = []
int_are_size_bits = []

float_are_32_bits   = []
float_are_64_bits   = []
float_are_size_bits = []
```

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.