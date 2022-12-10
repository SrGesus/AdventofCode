use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("./resources/day10/input.txt")?;
    let reader = BufReader::new(file).lines();

    let mut register = Register::new();

    for line in reader {
        let line = line?;
        match &line[0..4] {
            "addx" => register.addx(line[5..].parse().unwrap()),
            "noop" => register.noop(),
            _ => panic!(),
        }
    }
    println!("{}", register.sum);
    println!("{}", register.register);
    Ok(())
}

struct Register {
    sum: i32,
    cycle: i32,
    register: i32,
    monitor: Vec<Vec<char>>,
    line: Vec<char>
}
impl Register {
    fn new() -> Self {
        Register {
            sum: 0,
            cycle: 0,
            register: 1,
            monitor: Vec::new(),
            line: Vec::new(),
        }
    }
    fn addx(&mut self, value: i32) {
        self.noop();
        self.noop();
        self.register += value;
    }
    fn noop(&mut self) {
        self.cycle += 1;
        self.add_to_monitor();
    }

    fn add_to_monitor(&mut self) {
        if (self.cycle - 20) % 40 == 0 {
            let line = self.line;
            self.monitor.push(line)
        }
    }
}
