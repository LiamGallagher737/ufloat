<div align="center">

# ufloat

A `no_std` compatible library for formatting floating point numbers with [`ufmt`](https://crates.io/crates/ufmt)

[![crates.io](https://img.shields.io/crates/v/ufloat?style=for-the-badge)](https://crates.io/crates/ufloat)
[![docs.rs](https://img.shields.io/docsrs/ufloat?style=for-the-badge)](https://docs.rs/ufloat/latest/ufloat/struct.Angle.html)

</div>

Formatting a float is now as easy as wrapping it in either the [`Uf32`](https://docs.rs/ufloat/latest/ufloat/struct.Uf32.html) or [`Uf64`](https://docs.rs/ufloat/latest/ufloat/struct.Uf64.html) struct with the number of decimal places to format to.

```rust
use ufloat::{Uf32, Uf64};

// Format to 3 decimal places.
let a = Uf32(123.456, 3);
// Format to 5 decimal places.
let b = Uf64(123.45678, 5);
```

The [`libm`](https://crates.io/crates/libm) crate is used for math operations.
