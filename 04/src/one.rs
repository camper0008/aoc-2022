use crate::prelude::*;

/// # Panics
///
/// Will panic if unable to open file

pub fn print_result(reader: BufReader<File>) {
    println!(
        "{}",
        reader
            .lines()
            .map(|pair| {
                let mut ranges = pair
                    .unwrap()
                    .split(',')
                    .map(|elf| {
                        let mut range = elf
                            .split('-')
                            .map(|r| r.parse::<u32>().unwrap())
                            .collect::<Vec<_>>();
                        let end = range.pop().unwrap();
                        let start = range.pop().unwrap();
                        (start, end)
                    })
                    .collect::<Vec<(u32, u32)>>();
                let (start0, end0) = ranges.pop().unwrap();
                let (start1, end1) = ranges.pop().unwrap();
                if (start0..=end0).contains(&start1) && (start0..=end0).contains(&end1)
                    || (start1..=end1).contains(&start0) && (start1..=end1).contains(&end0)
                {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
}
