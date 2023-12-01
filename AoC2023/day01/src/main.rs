use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut sum = 0;

    let file = File::open("./inputs/day01.txt").unwrap();
    let reader = BufReader::new(file).lines();

    for line in reader {
        let line: String = line.unwrap();
        let mut first_digit = None;
        let mut last_digit = None;
        let mut num_string = String::with_capacity(5);

        for c in line.chars() {
            if c.is_digit(10) {
                // Erase String
                num_string = String::with_capacity(5);
                if let None = first_digit {
                    first_digit = Some(c.to_digit(10).unwrap());
                }
                last_digit = Some(c.to_digit(10).unwrap());
            } else {
                num_string.push(c);
                for i in 0..num_string.len() {
                    let string = &num_string[i..];
                    let digit;
                    match string {
                        "one" => digit = 1,
                        "two" => digit = 2,
                        "three" => digit = 3,
                        "four" => digit = 4,
                        "five" => digit = 5,
                        "six" => digit = 6,
                        "seven" => digit = 7,
                        "eight" => digit = 8,
                        "nine" => digit = 9,
                        _ => continue,
                    }
                    if let None = first_digit {
                        first_digit = Some(digit);
                    }
                    last_digit = Some(digit);
                }
            }
        }
        println!("{}{}", first_digit.unwrap(), last_digit.unwrap());
        sum += first_digit.unwrap()*10 + last_digit.unwrap();
    }
    println!("Sum: {}", sum);
}
