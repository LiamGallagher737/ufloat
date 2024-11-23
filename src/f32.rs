use ufmt::{uDebug, uDisplay, uWrite};

/// A wrapper for [`f32`] implementing [`ufmt::uDisplay`] and [`ufmt::uDebug`].
///
/// ```rust
/// use ufloat::Uf32;
///
/// // Format to 3 decimal places.
/// let f32 = Uf32(123.456, 3);
/// ```
pub struct Uf32(pub f32, pub usize);

impl uDisplay for Uf32 {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        if self.0.is_sign_negative() {
            f.write_char('-')?;
        }

        let abs_val = libm::fabsf(self.0);

        if self.1 == 0 {
            ufmt::uwrite!(f, "{}", libm::roundf(abs_val) as usize)?;
            return Ok(());
        }

        let scale = libm::powf(10.0, self.1 as f32);
        let scaled = libm::roundf(abs_val * scale) as u64;

        let int_part = scaled / 10_u64.pow(self.1 as u32);
        let decimal_part = scaled % 10_u64.pow(self.1 as u32);

        ufmt::uwrite!(f, "{}", int_part)?;

        f.write_char('.')?;

        let mut decimal_digits = 0;
        let mut temp = decimal_part;
        while temp > 0 {
            temp /= 10;
            decimal_digits += 1;
        }

        for _ in 0..(self.1 - decimal_digits) {
            f.write_char('0')?;
        }

        if decimal_part > 0 {
            ufmt::uwrite!(f, "{}", decimal_part)?;
        }

        Ok(())
    }
}

// Use the display impl for debug
impl uDebug for Uf32 {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::Buffer;

    #[test]
    fn test_uf32_format() {
        check_format_f32(1.23456, "1", 0);
        check_format_f32(-1.23456, "-1", 0);

        check_format_f32(1.23456, "1.2", 1);
        check_format_f32(0.999, "1.0", 1);
        check_format_f32(-1.23456, "-1.2", 1);

        check_format_f32(1.23456, "1.23", 2);
        check_format_f32(1.0, "1.00", 2);
        check_format_f32(-1.23456, "-1.23", 2);

        check_format_f32(1.23456, "1.235", 3);
        check_format_f32(0.1, "0.100", 3);
        check_format_f32(-1.23456, "-1.235", 3);

        check_format_f32(0.00123, "0.0012", 4);
        check_format_f32(1.23456, "1.2346", 4);
        check_format_f32(-0.00123, "-0.0012", 4);

        check_format_f32(17.80416, "17.80416", 5);
    }

    fn check_format_f32(value: f32, expected: &str, dp: usize) {
        let mut buffer = Buffer::default();
        ufmt::uwrite!(&mut buffer, "{}", Uf32(value, dp)).unwrap();
        let result = core::str::from_utf8(&buffer.buffer[0..buffer.position]).unwrap();
        assert_eq!(expected, result);
    }
}
