use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const MAX_SELECT: usize = 100000;

fn main() -> std::io::Result<()> {
let file = File::open("./resources/day07/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut current_dir: Vec<String> = Vec::new();

    for line in reader {
        let line = line?;
        if &line[0..1] == "$" {
            if &line[2..4] == "cd" {
                cd_into(&mut sizes, &mut current_dir, &line[5..])
            }
        } else {
            match line.split(' ').next().unwrap() {
                "dir" => (),
                number => add_to_size(&mut sizes, &current_dir, number.parse::<usize>().unwrap()),
            }
        }
    }
    println!("{:?}", sizes);
    let mut sum = 0;
    for dir in sizes.values() {
        if dir <= &MAX_SELECT {
            sum += dir;
        }
    }
    println!("Sum: {sum}");
    Ok(())
}

fn get_path(current_dir: &[String]) -> String {
    let mut path = String::from("/");
    for i in current_dir.iter().skip(1) {
        path += i;
        path += "/";
    }
    path
}

fn add_to_size(sizes: &mut HashMap<String, usize>, current_dir: &Vec<String>, file_size: usize) {
    for i in 1..current_dir.len()+1  {
        let path = get_path(&Vec::from(&current_dir[..i]));
        sizes.insert(path.to_owned(), sizes.get(&path).unwrap() + file_size);
    }
    // for dir in current_dir {
    //     sizes.insert(dir.to_string(), sizes.get(dir).unwrap() + file_size);
    // }
}

fn cd_into(sizes: &mut HashMap<String, usize>, current_dir: &mut Vec<String>, dir: &str) {
    if dir == ".." {
        current_dir.pop();
    } else {
        current_dir.push(dir.to_string());
        if sizes.get(get_path(current_dir).as_str()).is_none() {
            sizes.insert(get_path(current_dir), 0);
        }
    }
}
