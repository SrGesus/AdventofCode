use std::fs::File;
use std::io::{BufRead, BufReader};

const MARKER_SIZE: usize = 14;

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day06/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut sum = 0;

    for line in reader {
        let line = line?;
       sum += find_marker(line);
    }
    println!("{sum}");
    Ok(())
}

fn find_marker(packet: String) -> usize {
    for i in MARKER_SIZE..packet.len() {
        if is_marker(&packet[i-MARKER_SIZE..i]) {
            return i;
        }
    }
    0
}

fn is_marker(marker: &str) -> bool {
    for (i, ch1) in marker.chars().enumerate() {
        for ch2 in marker[i+1..].chars() {
            if ch1 == ch2 {
                return false;
            }
        }
    }
    true
}