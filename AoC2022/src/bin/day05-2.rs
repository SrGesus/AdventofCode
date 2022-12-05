use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{VecDeque};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day05/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut cargo: Cargo = vec![VecDeque::new(); 9];
    let mut collecting_cargo = true;

    for line in reader {
        let line = line?;

        if collecting_cargo {
            match cargo.push_line(line) {
                Result::EndOfCargo => collecting_cargo = false,
                _ => (),
            }
        } else {
            let words: Vec<&str> = line.as_str().split(" ").collect();
            println!("{:?}", cargo);
            println!("{:?}", words);
            if words.len() == 6 {
                let (number, source, destination) = (
                    words[1].parse::<usize>().unwrap(),
                    words[3].parse::<usize>().unwrap() - 1,
                    words[5].parse::<usize>().unwrap() - 1,
                );
                cargo.move_crates(number, source, destination);
            }
        }
    }
    let mut result = String::new();
    for column in cargo {
        match column.back() {
            Some(char) => result.push(*char),
            None => (),
        }
    }
    println!("{}", result);
    Ok(())
}

type Cargo = Vec<VecDeque<char>>;

enum Result {
    Pushed,
    EndOfCargo
}

trait CargoTrait {
    fn push_line(&mut self, line: String) -> Result;
    fn move_crates(&mut self, number: usize, source: usize, destination: usize);
}

impl CargoTrait for Cargo {
    fn push_line(&mut self, line: String) -> Result {
        let mut x = 1;
        let mut i = 0;
        loop {
            match line.chars().nth(x) {
                Some(char)  => {
                    match char {
                        'A'..='Z' => {
                            self[i].push_front(char);
                        },
                        ' ' => (),
                        '0'..='9' => return Result::EndOfCargo,
                        _ => panic!()
                    }
                },
                None => {
                    return Result::Pushed;
                },
            };
            x += 4;
            i += 1;
        }
    }

    fn move_crates(&mut self, number: usize, source: usize, destination: usize) {
        let mut temp_crates: VecDeque<char> = VecDeque::new();
        for _ in 0..number {
            match self[source].pop_back() {
                Some(cargo) => temp_crates.push_back(cargo),
                None => (),
            }
        }
        for _ in 0..number {
            match temp_crates.pop_back() {
                Some(cargo) => self[destination].push_back(cargo),
                None => (),
            }
        }
    }
}