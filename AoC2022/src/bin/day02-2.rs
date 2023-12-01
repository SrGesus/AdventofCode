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
        println!("Elf Plays: {:?}", opponent);
        let desired_outcome = Outcome::from_str(&line[2..3]).unwrap();
        let me = opponent.get_me(desired_outcome);
        println!("You Play: {:?}", me);
        let result = me.get_points(opponent);
        sum += result;
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

    fn from_str(str: &str) -> Result<Outcome, Error> {
        match str {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(Error)
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

    fn get_me(&self, desired: Outcome) -> RPS {
        match self {
            RPS::Rock => match desired {
                 Outcome::Draw => RPS::Rock,
                 Outcome::Win => RPS::Paper,
                 Outcome::Loss => RPS::Scissors,
            },
            RPS::Paper => match desired {
                Outcome::Loss => RPS::Rock,
                Outcome::Draw => RPS::Paper,
                Outcome::Win => RPS::Scissors,
            },
            RPS::Scissors => match desired {
                Outcome::Win => RPS::Rock,
                Outcome::Loss => RPS::Paper,
                Outcome::Draw => RPS::Scissors,
            },
        }
    }

    fn from_str(str: &str) -> Result<RPS, Error> {
        match str {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            _ => Err(Error)
        }
    }

    fn get_points(self, opponent: RPS) -> u16 {

        let choice_score = match &self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        let outcome_score = self.get_outcome(opponent).get_points();
        println!("Score: {choice_score} + {outcome_score}");
        choice_score + outcome_score
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