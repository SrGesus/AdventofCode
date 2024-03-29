use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day09/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut visited: HashSet<Cell> = HashSet::new();
    let mut head: Cell = (0, 0);
    let mut tail: Cell = (0, 0);
    let mut sum = 0;

    for line in reader {
        let line = line?;
        sum += move_head(&mut head, &mut tail, &mut visited, line);
    }
    println!("Sum = {sum}");
    Ok(())
}

type Cell = (isize, isize);

fn move_tail(head: &Cell, tail: &mut Cell) {
    if tail.0 == head.0 {
        if tail.1 < head.1 - 1 {
            tail.1 += 1;
        }
        if tail.1 > head.1 + 1 {
            tail.1 -= 1;
        }
        return;
    }
    if tail.1 == head.1 {
        if tail.0 < head.0 -1 {
            tail.0 += 1;
        }
        if tail.0 > head.0 + 1 {
            tail.0 -= 1;
        }
        return;
    }
    if tail.0 < head.0 - 1 {
        tail.0 += 1;
        if tail.1 < head.1 {
            tail.1 += 1;
        }
        if tail.1 > head.1 {
            tail.1 -= 1;
        }
        return;
    }
    if tail.0 > head.0 + 1{
        tail.0 -= 1;
        if tail.1 < head.1 {
            tail.1 += 1;
        }
        if tail.1 > head.1 {
            tail.1 -= 1;
        }
        return;
    }
    if tail.1 < head.1 - 1 {
        tail.1 += 1;
        if tail.0 < head.0 {
            tail.0 += 1;
        }
        if tail.0 > head.0 {
            tail.0 -= 1;
        }
        return;
    }
    if tail.1 > head.1 + 1 {
        tail.1 -= 1;
        if tail.0 < head.0 {
            tail.0 += 1;
        }
        if tail.0 > head.0 {
            tail.0 -= 1;
        }
        return;
    }
    println!("Did nothing");
}

enum Direction{
    Left,
    Up,
    Right,
    Down,
}

fn move_head(head: &mut Cell, tail: &mut Cell, visited: &mut HashSet<Cell>, string: String) -> u32 {
    println!("{}", string);
    let vector = Direction::get_direction(string.get(0..1).unwrap().chars().next().unwrap()).get_vector();
    let steps = string[2..].parse::<u8>().unwrap();
    let mut sum = 0;
    for _ in 0..steps {
        head.0 = head.0 as isize + vector.0;
        head.1 = head.1 as isize + vector.1;
        println!("Head: {:?} Tail: {:?}", head, tail);
        move_tail(head, tail);
        if !visited.contains(tail) {
            visited.insert(*tail);
            sum += 1;
        }
    }
    sum
}

impl Direction {
    fn get_direction(ch: char) -> Self {
        match ch {
            'L' => Direction::Left,
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            _ => panic!(),
        }
    }
    fn get_vector(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        }
    }
}