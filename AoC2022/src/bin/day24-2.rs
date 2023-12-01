use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day24/input.txt")?;
    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
