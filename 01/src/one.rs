use crate::prelude::*;

/// # Panics
///
/// Will panic if unable to open file

pub fn print_result(reader: BufReader<File>) {
    let mut max_answer = 0;
    let mut answer = 0;

    for unchecked_line in reader.lines() {
        let line = unchecked_line.unwrap();
        if line.is_empty() {
            if answer > max_answer {
                max_answer = answer;
            }
            answer = 0;
        } else {
            answer += line.parse::<u32>().unwrap();
        }
    }

    println!("Top: {max_answer}");
}
