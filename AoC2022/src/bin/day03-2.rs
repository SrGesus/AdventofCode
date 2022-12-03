use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const GROUP_SIZE: u64 = 3;

fn main() {
    let file = File::open("./resources/day03/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;
    let mut elves = Elves::new();

    for line in reader {
        let line = line.unwrap();
        sum += elves.push_bag(line);
    }

    println!("Sum: {sum}");
}

struct Elves {
    number: u64,
    common_items: HashSet<char>,
}

impl Elves {
    fn new() -> Self {
        Elves {
            number: 0,
            common_items: HashSet::new(),
        }
    }

    // Will return the value to be added to the sum (either 0 or the badge's priority if on the last bag)
    fn push_bag(&mut self, bag: String) -> u64 {
        self.number += 1;
        let bag_items: HashSet<char> = bag.chars().collect();
        match self.number {
            GROUP_SIZE => {
                // We know there's only one item in common between the bags in the group,
                // so .iter().next() will return the only item
                let badge: char = bag_items
                    .intersection(&self.common_items)
                    .map(|ch| ch.to_owned())
                    .collect::<HashSet<char>>()
                    .iter()
                    .next()
                    .unwrap()
                    .to_owned();
                self.number = 0;
                item_to_priority(badge)
            }
            1 => {
                // First bag reset the common items
                self.common_items = bag.chars().collect();
                0
            }
            _ => {
                // Not first nor last bag, intersect them
                // intersection will yield an iterator of &chars, thus clone the chars and collect
                self.common_items = bag_items
                    .intersection(&self.common_items)
                    .map(|ch| ch.to_owned())
                    .collect();
                0
            }
        }
    }
}

fn item_to_priority(item: char) -> u64 {
    // char's ASCII number
    let item = item as u64;
    // If item is from a to z
    if 97 <= item && item <= 122 {
        return item - 96;
    }
    // If item is from A to Z
    if 65 <= item && item <= 90 {
        return item - 38;
    }
    panic!()
}
