use crate::prelude::*;
use std::collections::VecDeque;

/// # Panics
///
/// Will panic if unable to open file

pub fn print_result(reader: BufReader<File>) {
    let mut answer_vec = VecDeque::from([0, 0, 0]);
    let mut answer = 0;

    for unchecked_line in reader.lines() {
        let line = unchecked_line.unwrap();
        if line.is_empty() {
            let one = answer_vec.pop_front().unwrap();
            let two = answer_vec.pop_front().unwrap();
            let three = answer_vec.pop_front().unwrap();

            if answer > one {
                answer_vec.push_back(answer);
                answer_vec.push_back(one);
                answer_vec.push_back(two);
            } else if answer > two {
                answer_vec.push_back(one);
                answer_vec.push_back(answer);
                answer_vec.push_back(two);
            } else if answer > three {
                answer_vec.push_back(one);
                answer_vec.push_back(two);
                answer_vec.push_back(answer);
            } else {
                answer_vec.push_back(one);
                answer_vec.push_back(two);
                answer_vec.push_back(three);
            }

            answer = 0;
        } else {
            answer += line.parse::<u32>().unwrap();
        }
    }

    let one = answer_vec.pop_front().unwrap();
    let two = answer_vec.pop_front().unwrap();
    let three = answer_vec.pop_front().unwrap();

    let sum = one + two + three;

    println!("Sum: {sum}.");
}
