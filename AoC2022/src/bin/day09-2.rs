use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day09/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut visited: HashSet<Cell> = HashSet::new();
    let mut head: Cell = Cell(0, 0);
    let mut tails: Vec<Cell> = vec![Cell(0, 0); 9];
    let mut sum = 0;

    for line in reader {
        let line = line?;
        sum += move_head(&mut head, &mut tails, &mut visited, line);
    }
    println!("Sum = {sum}");
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cell(isize, isize);

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
}

enum Direction{
    Left,
    Up,
    Right,
    Down,
}

fn move_head(head: &mut Cell, tails: &mut[Cell], visited: &mut HashSet<Cell>, string: String) -> u32 {
    let vector = Direction::get_direction(string.get(0..1).unwrap().chars().next().unwrap()).get_vector();
    let steps = string[2..].parse::<u8>().unwrap();
    let mut sum = 0;
    for _ in 0..steps {
        head.0 = head.0 as isize + vector.0;
        head.1 = head.1 as isize + vector.1;
        let mut last_tail = &head.clone();
        for knot in tails.iter_mut() {
            move_tail(last_tail, knot);
            last_tail = knot
        }
        if !visited.contains(last_tail) {
            visited.insert(*last_tail);
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
