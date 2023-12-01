use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./resources/day01/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut elf = 0;
    let mut max = 0;

    for line in reader {
        let line: Result<u32, _> = line.unwrap().parse();
        match line {
            // Line will either be the number, or Err if blank
            Ok(num) => elf += num,
            Err(_) => {
                if elf > max {
                    max = elf
                }
                elf = 0;
            }
        }
    }

    // Check the last elf (if it doesn't end in a blank line)
    if elf > max {
        max = elf
    }

    println!("{max}")
}
