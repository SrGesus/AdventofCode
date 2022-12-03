use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Error;


fn main() {
    let file = File::open("./resources/day02/input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;

    for line in reader {
        let line = line.unwrap();
        let opponent = RPS::from_str(&line[0..1]).unwrap();
        let me = RPS::from_str(&line[2..3]).unwrap();
        sum += me.get_points(opponent);
    }
    println!("Total Score: \n{sum}")

}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw
}


impl Outcome {
    fn get_points(self) -> u16 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn from_str(str: &str) -> Result<RPS, Error> {
        match str {
            "X" | "A" => Ok(RPS::Rock),
            "Y" | "B" => Ok(RPS::Paper),
            "Z" | "C" => Ok(RPS::Scissors),
            _ => Err(Error)
        }
    }

    fn get_points(self, opponent: RPS) -> u16 {

        let score = match &self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        println!("Score: {score}");
        score + self.get_outcome(opponent).get_points()
    }

    fn get_outcome(self, opponent: RPS) -> Outcome {
        match self {
            RPS::Rock => match opponent {
                RPS::Rock => Outcome::Draw,
                RPS::Paper => Outcome::Loss,
                RPS::Scissors => Outcome::Win,
            },
            RPS::Paper => match opponent {
                RPS::Rock => Outcome::Win,
                RPS::Paper => Outcome::Draw,
                RPS::Scissors => Outcome::Loss,
            },
            RPS::Scissors => match opponent {
                RPS::Rock => Outcome::Loss,
                RPS::Paper => Outcome::Win,
                RPS::Scissors => Outcome::Draw,
            },
        }
    }
}