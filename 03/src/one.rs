use crate::prelude::*;

/// # Panics
///
/// Will panic if unable to open file

pub fn print_result(reader: BufReader<File>) {
    type PartitionedContents = (Vec<(usize, char)>, Vec<(usize, char)>);

    println!(
        "Priority: {}",
        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let line_len = line.len();
                (line, line_len)
            })
            .map(|(line, line_len)| -> PartitionedContents {
                line.chars()
                    .enumerate()
                    .partition(|(i, _)| *i < line_len / 2)
            })
            .map(|(a, b)| (
                a.into_iter().map(|(_, ac)| ac),
                b.into_iter().map(|(_, bc)| bc)
            ))
            .map(|(a, b)| (a.collect::<HashSet<_>>(), b.collect::<HashSet<_>>()))
            .map(|(left, right)| {
                left.into_iter()
                    .map(|c| {
                        if right.contains(&c) {
                            priority_of_char(c)
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
    );
}
