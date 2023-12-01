use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let file = File::open("./resources/day24/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut grid: Vec<Vec<Slot>> = Vec::new();
    for line in reader {
        let line = line?;
        let mut row: Vec<Slot> = Vec::new();
        for ch in line.chars() {
            let slot = match ch {
                '#' => Slot::Wall,
                '.' => Slot::Clear,
                '>' => Slot::right(),
                '<' => Slot::left(),
                '^' => Slot::up(),
                'v' => Slot::down(),
                _ => panic!(),
            };
            row.push(slot);
        }
        grid.push(row);
    }
    let mut bfs = Bfs::new(grid);

    bfs.print_grid();
    let mut count = 0;
    let mut breaker = false;
    loop {
        count += 1;
        // sleep(Duration::from_secs(1));
        bfs.update_grid();
        bfs.print_grid();
        bfs.get_next_moves();
        println!("Count = {count}");
        for (i, j) in bfs.queue.iter() {
            if let Slot::Goal = bfs.grid[*i][*j] {
                bfs.queue = vec![(*i, *j)];
                breaker = true;
                break;
            }
        }
        if breaker {
            breaker = false;
            break;
        }
    }
    for slot in bfs.grid.last_mut().unwrap() {
        if let Slot::Goal = slot {
            *slot = Slot::Clear;
        }
    }
    for slot in bfs.grid.first_mut().unwrap() {
        if let Slot::Clear = slot {
            *slot = Slot::Goal;
        }
    }
    loop {
        count += 1;
        // sleep(Duration::from_secs(1));
        bfs.update_grid();
        bfs.print_grid();
        bfs.get_next_moves();
        println!("Count = {count}");
        for (i, j) in bfs.queue.iter() {
            if let Slot::Goal = bfs.grid[*i][*j] {
                bfs.queue = vec![(*i, *j)];
                breaker = true;
                break;
            }
        }
        if breaker {
            breaker = false;
            break;
        }
    }
    for slot in bfs.grid.first_mut().unwrap() {
        if let Slot::Goal = slot {
            *slot = Slot::Clear;
        }
    }
    for slot in bfs.grid.last_mut().unwrap() {
        if let Slot::Clear = slot {
            *slot = Slot::Goal;
        }
    }
    loop {
        count += 1;
        // sleep(Duration::from_secs(1));
        bfs.update_grid();
        bfs.print_grid();
        bfs.get_next_moves();
        println!("Count = {count}");
        for (i, j) in bfs.queue.iter() {
            if let Slot::Goal = bfs.grid[*i][*j] {
                bfs.queue = vec![(*i, *j)];
                breaker = true;
                break;
            }
        }
        if breaker {
            breaker = false;
            break;
        }
    }
    Ok(())
}

struct Bfs {
    grid: Vec<Vec<Slot>>,
    queue: Vec<(usize, usize)>,
}

impl Bfs {
    fn new(mut grid: Vec<Vec<Slot>>) -> Self {
        // Add the goal as the only clear slot in the last row
        for slot in grid.last_mut().unwrap() {
            if let Slot::Clear = slot {
                *slot = Slot::Goal;
            }
        }
        Self {
            grid,
            queue: vec![(1,1)],
        }
    }

    fn print_grid(&self) {
        for row in self.grid.iter() {
            for slot in row {
                slot.print();
            }
            println!();
        }
    }

    fn new_grid(&self) -> Vec<Vec<Slot>> {
        let mut new_grid = self.grid.clone();
        for row in new_grid.iter_mut() {
            for slot in row.iter_mut() {
                if let Slot::Blizzard(_) = slot {
                    *slot = Slot::Clear;
                }
            }
        }
        new_grid
    }

    fn update_grid(&mut self) {
        let mut new_grid = self.new_grid();
        let height = self.grid.len();
        let width = self.grid[0].len();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, slot) in row.iter().enumerate() {
                if let Slot::Blizzard(blizzard) = slot {
                    if blizzard.up {
                        if i == 1 {
                            let new_slot = &mut new_grid[height - 2][j];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.up = true;
                            } else {
                                *new_slot = Slot::up();
                            }
                        } else {
                            let new_slot = &mut new_grid[i - 1][j];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.up = true;
                            } else {
                                *new_slot = Slot::up();
                            }
                        }
                    }
                    if blizzard.right {
                        if j == width-2 {
                            let new_slot = &mut new_grid[i][1];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.right = true;
                            } else {
                                *new_slot = Slot::right();
                            }
                        } else {
                            let new_slot = &mut new_grid[i][j+1];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.right = true;
                            } else {
                                *new_slot = Slot::right();
                            }
                        }
                    }
                    if blizzard.down {
                        if i == height-2 {
                            let new_slot = &mut new_grid[1][j];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.down = true;
                            } else {
                                *new_slot = Slot::down();
                            }
                        } else {
                            let new_slot = &mut new_grid[i+1][j];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.down = true;
                            } else {
                                *new_slot = Slot::down();
                            }
                        }
                    }
                    if blizzard.left {
                        if j == 1 {
                            let new_slot = &mut new_grid[i][width-2];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.left = true;
                            } else {
                                *new_slot = Slot::left();
                            }
                        } else {
                            let new_slot = &mut new_grid[i][j-1];
                            if let Slot::Blizzard(bliz) = new_slot {
                                bliz.left = true;
                            } else {
                                *new_slot = Slot::left();
                            }
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    fn get_next_moves(&mut self) {
        let mut moves: HashSet<(usize, usize)> = HashSet::new();
        for cell in self.queue.iter() {
            for (i, j) in adjacent_coordinates(*cell) {
                if let Slot::Clear = self.grid[i][j] {
                    moves.insert((i, j));
                }
                if let Slot::Goal = self.grid[i][j] {
                    moves.insert((i, j));
                }
            }
        }
        self.queue = moves.into_iter().collect();
    }

    
}

fn adjacent_coordinates((i, j): (usize, usize)) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    if i != 0 {
        cells.push((i-1, j));
    }
    if i != 26 {
        cells.push((i+1, j));
    }
    cells.push((i, j+1));
    cells.push((i, j-1));
    cells.push((i, j));
    cells
}

#[derive(Clone, Copy)]
enum Slot {
    Wall,
    Clear,
    Blizzard(Blizzard),
    Goal,
}

impl Slot {
    fn up() -> Slot {
        Slot::Blizzard(Blizzard {
            up: true,
            right: false,
            down: false,
            left: false,
        })
    }
    fn right() -> Slot {
        Slot::Blizzard(Blizzard {
            up: false,
            right: true,
            down: false,
            left: false,
        })
    }
    fn down() -> Slot {
        Slot::Blizzard(Blizzard {
            up: false,
            right: false,
            down: true,
            left: false,
        })
    }
    fn left() -> Slot {
        Slot::Blizzard(Blizzard {
            up: false,
            right: false,
            down: false,
            left: true,
        })
    }
    fn print(&self) {
        match self {
            Slot::Wall => print!("#"),
            Slot::Clear => print!("."),
            Slot::Blizzard(Blizzard {
                up: true,
                right: false,
                down: false,
                left: false,
            }) => print!("^"),
            Slot::Blizzard(Blizzard {
                up: false,
                right: true,
                down: false,
                left: false,
            }) => print!(">"),
            Slot::Blizzard(Blizzard {
                up: false,
                right: false,
                down: false,
                left: true,
            }) => print!("<"),
            Slot::Blizzard(Blizzard {
                up: false,
                right: false,
                down: true,
                left: false,
            }) => print!("v"),
            Slot::Goal => print!("G"),
            _ => print!("2"),
        }
    }
}

#[derive(Clone, Copy)]
struct Blizzard {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}
