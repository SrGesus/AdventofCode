use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file = File::open("./resources/day01/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut elf = 0;
    let mut max = Array::new();

    for line in reader {
        let line: Result<u32, _> = line.unwrap().parse();
        match line {
            Ok(num) => elf += num,
            Err(_) => {
                println!("{elf}");
                max.push(elf);
                elf = 0;
            }
        }
    }
    // Register the last elf
    println!("{elf}");
    max.push(elf);

    println!("{:?}", max.sum());
}


type Array = [u32; 3];

trait SelfSorting {
    fn new() -> Self;
    fn push(&mut self, value: u32);
    fn sum(self) -> u32;
}

impl SelfSorting for Array {
    fn new() -> Self {
        [0, 0, 0]
    }

    fn push(&mut self, value: u32) {
        let max = self;
        for i in 0..max.len() {
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