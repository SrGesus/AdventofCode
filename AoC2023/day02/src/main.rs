use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let mut sum = 0;

    let file = File::open("./inputs/day02.txt").unwrap();
    let reader = BufReader::new(file).lines();

    for line in reader {

        let mut colors_max: HashMap<&str, u32> = HashMap::new();
        colors_max.insert("red", 0);
        colors_max.insert("green", 0);
        colors_max.insert("blue", 0);

        let line: String = line.unwrap();
        let mut it = line.split(":");
        let id: u32 = it.next().unwrap().split(" ").skip(1).next().unwrap().parse().unwrap();
        for game in it.next().unwrap().split(";") {
            for mut cube in game.split(",").map(|s: &str| s.split(" ").skip(1)) {
                let n: u32 = cube.next().unwrap().parse().unwrap();
                let color = cube.next().unwrap();
                colors_max.insert(color, n.max(*colors_max.get(color).unwrap()));
            }
        }

        sum += colors_max.get("red").unwrap() * colors_max.get("green").unwrap() *colors_max.get("blue").unwrap();
    }
    println!("{}", sum);
}
