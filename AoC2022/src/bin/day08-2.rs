use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day08/input.txt")?;
    let reader = BufReader::new(file).lines();

    // collects vectors of the rows and columns on the grid
    let rows: Vec<Vec<Cell>> = reader.map(|line| line.unwrap().chars().map(|x| Cell {
        size: x as u8,
        scenic_score: 1,
    }).collect()).collect();
    let columns: Vec<Vec<Cell>> = {
        let mut vec: Vec<Vec<Cell>> = vec![vec![Cell::new(); rows.len()]; rows[0].len()];
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                vec[j][i] = rows[i][j].clone();
            }
        }
        vec
    };

    // Ignore the borders
    //
    let mut max = 0;
    for i in 1..rows.len()-1 {
        for j in 1..rows[i].len()-1 {
            let mut score = 1;
            // Check left
            score *= get_distance(rows[i][0..j].iter(), &rows[i][j]);
            // Check right
            score *= get_distance(rows[i][j+1..].iter().rev(), &rows[i][j]);
            // Check up
            score *= get_distance(columns[j][0..i].iter(), &rows[i][j]);
            // Check down
            score *= get_distance(columns[j][i+1..].iter().rev(), &rows[i][j]);
            if score > max {
                max = score;
            }
        }
    }
    println!("Rows: {:?}", rows);
    println!("Columns: {:?}", columns);
    println!("Max: {}", max);

    Ok(())
}

// Will take a line of trees
// Return the amount of trees it can see to its left
fn get_distance<'a, I>(line: I, cell: &Cell) -> usize
where  
    I: Iterator<Item = &'a Cell>,
    I: DoubleEndedIterator,
{
    if cell.scenic_score == 0 {
        return 0;
    }

    let mut distance = 0;
    for tree in line.rev() {
        distance += 1;
        if cell.size <= tree.size {
            return distance;
        }
    }
    distance
}

#[derive(Clone)]
struct Cell {
    size: u8,
    scenic_score: usize,
}

impl Cell {
    fn new() -> Self {
        Cell { size: 0, scenic_score: 0 }
    }
}
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.size - 48)
    }
}