pub mod one;
pub mod prelude;
pub mod two;

use prelude::{BufReader, File};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    one::print_result(reader);

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    two::print_result(reader);
}
