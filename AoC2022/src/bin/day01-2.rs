use std::fs::File;
use std::io::{BufRead, BufReader};

// Number of elves for the top
// (Here we want the calories of the TOP 3 elves)
const TOP_ELVES: usize = 3;

fn main() {
    let file = File::open("./resources/day01/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut elf = 0;
    // max is a self sorting array [u32, 3]
    let mut max = Array::new();

    for line in reader {
        let line: Result<u32, _> = line.unwrap().parse();
        // Line will either be the number, or Err if blank
        match line {
            Ok(num) => elf += num,
            Err(_) => {
                max.push(elf);
                elf = 0;
            }
        }
    }
    // Check the last elf (if it doesn't end in a blank line)
    max.push(elf);

    println!("{:?}", max.sum());
}

type Array = [u32; TOP_ELVES];

trait SelfSorting {
    fn new() -> Self;
    fn push(&mut self, value: u32);
    fn sum(self) -> u32;
}
impl SelfSorting for Array {
    fn new() -> Self {
        [0; TOP_ELVES]
    }

    fn push(&mut self, value: u32) {
        let max = self;
        // Insert the value if it's greater than the smallest value 
        // while maintaning the array sorted
        for i in 0..TOP_ELVES {
            if value > max[i] {
                if i > 0 {
                    max[i-1] = max[i]
                }
                max[i] = value
            }
        }
    }

    fn sum(self) -> u32 {
        let mut sum = 0;
        let max = self;
        for i in 0..max.len() {
            sum += max[i];
        }
        sum
    }
}