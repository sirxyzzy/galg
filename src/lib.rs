use snafu::{prelude::*, Whatever};

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Whatever> {
    if a == 64 {
        whatever!("64 is not allowed")
    } else {
        Ok(a + b)
    }
}