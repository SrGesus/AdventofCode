use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let mut sum = 0;

    let file = File::open("./inputs/day02.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut colors_max: HashMap<&str, u32> = HashMap::new();
    colors_max.insert("red", 12);
    colors_max.insert("green", 13);
    colors_max.insert("blue", 14);

    for line in reader {
        let mut possible = true;
        let line: String = line.unwrap();
        let mut it = line.split(":");
        let id: u32 = it.next().unwrap().split(" ").skip(1).next().unwrap().parse().unwrap();
        for game in it.next().unwrap().split(";") {
            for mut cube in game.split(",").map(|s: &str| s.split(" ").skip(1)) {
                let n: u32 = cube.next().unwrap().parse().unwrap();
                let color = cube.next().unwrap();
                if n > *colors_max.get(color).unwrap() {
                    possible = false;
                }
            }
        }

        if possible {
            sum += id;
        }
    }
    println!("{}", sum);
}
