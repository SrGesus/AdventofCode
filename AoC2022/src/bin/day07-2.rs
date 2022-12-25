use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const FILESYSTEM_SIZE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

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
    println!("{:#?}", sizes);
    let used_space = sizes.get("/").unwrap();
    let needed_space = used_space - (FILESYSTEM_SIZE - REQUIRED_SPACE);
    let mut min = FILESYSTEM_SIZE;
    for size in sizes.values() {
        if size >= &needed_space && size < &min {
            min = *size;
        }
    }
    println!("Smallest: {min}");
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
        let path = get_path(&current_dir[..i]);
        sizes.insert(path.to_owned(), sizes.get(&path).unwrap() + file_size);
    }
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
