use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day08/input.txt")?;
    let reader = BufReader::new(file).lines();

    // collects vectors of the rows and columns on the grid
    let rows: Vec<Vec<u8>> = reader.map(|line| line.unwrap().chars().map(|x| x as u8).collect()).collect();
    let columns: Vec<Vec<u8>> = {
        let mut vec: Vec<Vec<u8>> = vec![vec![0; rows.len()]; rows[0].len()];
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                vec[j][i] = rows[i][j];
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
            let size = &rows[i][j];
            // Check left
            score *= get_distance(rows[i][0..j].iter(), size);
            // Check right
            score *= get_distance(rows[i][j+1..].iter().rev(), size);
            // Check up
            score *= get_distance(columns[j][0..i].iter(), size);
            // Check down
            score *= get_distance(columns[j][i+1..].iter().rev(), size);
            if score > max {
                max = score;
            }
        }
    }
    println!("Max: {}", max);

    Ok(())
}

// Will take a line of trees
// Return the amount of trees it can see to its left
fn get_distance<'a, I>(line: I, size: &u8) -> usize
where  
    I: Iterator<Item = &'a u8>,
    I: DoubleEndedIterator,
{
    let mut distance = 0;
    for tree in line.rev() {
        distance += 1;
        if size <= tree {
            return distance;
        }
    }
    distance
}