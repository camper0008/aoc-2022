use crate::prelude::*;

pub fn print_result(reader: BufReader<File>) {
    let mut used_chars = VecDeque::new();
    let mut iter = reader.bytes().enumerate();
    println!(
        "{}",
        loop {
            if let Some((i, Ok(b))) = iter.next() {
                if used_chars.contains(&b) {
                    loop {
                        if let Some(el) = used_chars.pop_front() {
                            if el == b {
                                break;
                            }
                        }
                    }
                }

                used_chars.push_back(b);
                if used_chars.len() >= 14 {
                    break i + 1;
                }
            }
        }
    );
}
