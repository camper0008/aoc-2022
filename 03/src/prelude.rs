#[must_use]
pub fn priority_of_char(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

pub use std::collections::HashSet;
pub use std::fs::File;
pub use std::io::{BufRead, BufReader};
