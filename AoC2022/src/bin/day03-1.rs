use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
// use std::fmt::Error;


fn main() {
    let file = File::open("./resources/day03/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;

    for line in reader {
        let line = line.unwrap();
        let item = common_item(line).unwrap();
        sum += item_to_priority(item)
    };

    println!("Sum: {sum}");
}

fn common_item(bags: String) -> Option<char> {
    let bag1 = bags[0..bags.len()/2].chars();
    let bag2: HashSet<char> = bags[bags.len()/2..].chars().collect();
    println!("Bag1: {:?} \nBag2: {:?}", bag1.as_str(), bags[bags.len()/2..].to_owned());
    for char in bag1 {
        if bag2.contains(&char) {
            println!("Both contain: {char}");
            return Some(char);
        }
    }
    None
}

fn item_to_priority(item: char) -> u64 {
    println!("{}", item as u64);
    let item = item as u64;
    // If item is from a to z
    if 97 <= item && item <= 122 {
        return item-96;
    }
    // If item is from A to Z
    if 65 <= item && item <= 90 {
        return item-38;
    }
    panic!()
}