use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
let file = File::open("./resources/day04/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line.unwrap();
        println!("{}", line);
    }
}
