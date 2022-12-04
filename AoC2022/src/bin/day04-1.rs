use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

fn main() -> std::io::Result<()> {
    let file = File::open("./resources/day04/input.txt")?;
    let reader = BufReader::new(file).lines();

        let mut sum = 0;

        for line in reader {

            if SectionID::completely_overlaps(SectionID::from_str(line?)) {
                sum += 1;
            };
        }

        println!("Sum = {sum}");

        Ok(())
}

#[derive(PartialEq)]
struct SectionID {
    start: u8,
    end: u8
}

impl SectionID {
    fn new(start: u8, end: u8) -> Self {
        SectionID {
            start,
            end
        }
    }

    fn from_str(string: String) -> (Self, Self) {
        let mut number = 0;
        let mut range = [0; 4];
        let mut start = 0;
        for (i, ch) in string.chars().enumerate() {
            if ch as u8 == 45 || ch as u8 == 44 {
                range[number] = string[start..i].parse::<u8>().unwrap();
                number += 1;
                start = i+1;
            }
        }
        // get the last number
        range[number] = string[start..].parse::<u8>().unwrap();

        println!("{:?}", range);
        (SectionID::new(range[0], range[1]), SectionID::new(range[2], range[3]))
    }

    fn completely_overlaps((section1, section2): (Self, Self)) -> bool {
        let intersection = SectionID::new(
            max(section1.start, section2.start),
            min(section1.end, section2.end)
        );
        section1 == intersection || section2 == intersection
    }
}

