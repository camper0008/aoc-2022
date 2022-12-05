use crate::prelude::*;

pub fn print_result(reader: BufReader<File>) {
    println!(
        "Two: {}",
        reader
            .lines()
            .map(std::result::Result::unwrap)
            .map(|l| l.split_whitespace().flat_map(str::chars).collect())
            .map(|c: Vec<char>| match (c[0], c[1]) {
                ('A', 'X') => 3, // 0 + 3
                ('A', 'Y') => 4, // 3 + 1
                ('A', 'Z') => 8, // 6 + 2

                ('B', 'X') => 1, // 0 + 1
                ('B', 'Y') => 5, // 3 + 2
                ('B', 'Z') => 9, // 6 + 3

                ('C', 'X') => 2, // 0 + 2
                ('C', 'Y') => 6, // 3 + 3
                ('C', 'Z') => 7, // 6 + 1

                _ => unreachable!(),
            })
            .sum::<u32>()
    );
}
