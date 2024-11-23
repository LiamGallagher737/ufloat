//! A `no_std` compatible library for formatting floating point numbers with [`ufmt`](https://crates.io/crates/ufmt).
//!
//! Formatting a float is now as easy as wrapping it in either the [`Uf32`] or [`Uf64`] struct with the number of decimal places to format to.
//!
//! ```rust
//! use ufloat::{Uf32, Uf64};
//! 
//! // Format to 3 decimal places.
//! let a = Uf32(123.456, 3);
//! // Format to 5 decimal places.
//! let b = Uf64(123.45678, 5);
//! ```
//! 
//! The [`libm`](https://crates.io/crates/libm) crate is used for math operations.

#![cfg_attr(not(test), no_std)]

pub use f32::Uf32;
pub use f64::Uf64;

mod f32;
mod f64;

#[cfg(test)]
mod test_utils {
    #[derive(Default)]
    pub struct Buffer {
        pub buffer: [u8; 32],
        pub position: usize,
    }
    impl ufmt::uWrite for Buffer {
        type Error = ();

        fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
            let bytes = s.as_bytes();
            let remaining_space = self.buffer.len().saturating_sub(self.position);

            if bytes.len() > remaining_space {
                panic!("Please increase test buffer size");
            }

            self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
            self.position += bytes.len();

            Ok(())
        }
    }
}
