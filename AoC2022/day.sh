# Simple shell script that generates the files for the day in $1

FILE=$(printf './src/bin/day%02d-1.rs' $1)

if test -f '$FILE'; then
    echo '$FILE already exists.'
else
    echo 'use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {' >> $FILE
    echo $(printf 'let file = File::open("./resources/day%02d/input.txt").unwrap();' $1) >> $FILE
    echo '    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line.unwrap();
        println!("{}", line);
    }
}' >> $FILE
fi

FILE=$(printf './src/bin/day%02d-2.rs' $1)

if test -f '$FILE'; then
    echo '$FILE already exists.'
else
    echo 'use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {' >> $FILE
    echo $(printf 'let file = File::open("./resources/day%02d/input.txt").unwrap();' $1) >> $FILE
    echo '    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line.unwrap();
        println!("{}", line);
    }
}' >> $FILE
fi