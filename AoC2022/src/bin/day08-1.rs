use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day08/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut sum = 0;
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let mut max_top: Vec<u8> = Vec::new();

    for line in reader {
        let line = line?;
        let mut row: Vec<Cell> = Vec::new();
        // First scan left right
        let mut max_left = 0; //The minimum is 48, the ASCII number for 0
        for (j, ch) in line.chars().enumerate() {
            let mut cell = {
                if max_left < ch as u8 {
                    max_left = ch as u8;
                    sum += 1; 
                    Cell {
                        visible: true,
                        size: ch as u8,
                    }
                } else {
                    Cell { 
                        visible: false,
                        size: ch as u8,
                    }
                }
            };
            // Scan top down
            match max_top.get(j) {
                // First row
                None => {
                    if !cell.visible {
                        cell.visible = true;
                        sum += 1;
                    }
                    max_top.push(cell.size);
                },
                Some(max) => {
                    if *max < cell.size {
                        if !cell.visible {
                            cell.visible = true;
                            sum += 1;
                        }
                        max_top[j] = cell.size;
                    }
                }
            }
            row.push(cell);
        }
        grid.push(row);
    }

    // Scan down top, right left
    let mut max_down: Vec<u8> = vec![0; grid[0].len()];
    for row in grid.iter_mut().rev() {
        // right to left
        let mut max_right = 0; //The minimum is 48, the ASCII number for 0
        for (j, cell) in row.iter_mut().rev().enumerate() {
            if max_right < cell.size {
                if !cell.visible {
                    cell.visible = true;
                    sum += 1;
                }
                max_right = cell.size;
            }
            if max_down[j] < cell.size {
                if !cell.visible {
                    cell.visible = true;
                    sum += 1;
                }
                max_down[j] = cell.size;
            }
        }
    }
    println!("Grid: \n{:?}\nVisible: {}", grid, sum);
    Ok(())
}

struct Cell {
    visible: bool,
    size: u8,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.size - 48)
    }
}