use crate::prelude::*;

/// # Panics
///
/// Will panic if unable to open file

pub fn print_result(reader: BufReader<File>) {
    let mut lines = reader.lines().peekable();
    let mut stuff = Vec::new();

    loop {
        if lines.peek().is_some() {
            let unwrapped = lines.next().unwrap().unwrap();
            let mut n0 = unwrapped.chars();
            let n1 = lines
                .next()
                .unwrap()
                .unwrap()
                .chars()
                .collect::<HashSet<_>>();
            let n2 = lines
                .next()
                .unwrap()
                .unwrap()
                .chars()
                .collect::<HashSet<_>>();

            stuff.push(loop {
                if let Some(c) = n0.next() {
                    if n1.contains(&c) && n2.contains(&c) {
                        break c;
                    }
                } else {
                    unreachable!()
                }
            });
        } else {
            break;
        }
    }

    println!(
        "Badges: {}",
        stuff.into_iter().map(priority_of_char).sum::<u32>()
    );
}
