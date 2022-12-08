use crate::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct FileItem(VecDeque<String>, u32);

pub fn print_result(reader: BufReader<File>) {
    let mut working_directory = Vec::new();
    let mut files = Vec::new();
    for unchecked_line in reader.lines() {
        let line = unchecked_line.unwrap();
        if line == "$ cd /" {
            continue;
        } else if line == "$ cd .." {
            working_directory.pop();
        } else if line.starts_with("$ cd") {
            let place = &line[5..];
            working_directory.push(String::from(place));
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            let mut file_data = line.split_whitespace();
            let type_or_size = file_data.next().unwrap();
            let name = file_data.next().unwrap();
            if type_or_size != "dir" {
                let size = type_or_size.parse::<u32>().unwrap();
                let mut full_path: VecDeque<String> = working_directory.clone().into();
                full_path.push_back(String::from(name));
                files.push(FileItem(full_path, size));
            }
        }
    }

    let mut dir_sizes = HashMap::<String, u32>::new();

    files.into_iter().for_each(|mut f| {
        let _file_name = f.0.pop_back().unwrap();
        let file_size = f.1;
        let mut working_path = String::new();
        loop {
            match f.0.pop_front() {
                Some(path) => {
                    working_path += "/";
                    working_path += &path;

                    dir_sizes.entry(working_path.clone()).or_insert(0);
                    dir_sizes.insert(
                        working_path.clone(),
                        file_size + dir_sizes[&working_path.clone()],
                    );
                }
                None => {
                    dir_sizes.entry(String::new()).or_insert(0);
                    dir_sizes.insert(String::new(), file_size + dir_sizes[""]);
                    break;
                }
            }
        }
    });

    println!(
        "{}",
        dir_sizes
            .into_iter()
            .filter(|(_k, v)| { *v <= 100000 })
            .fold(0, |acc, (_k, v)| acc + v)
    );
}
