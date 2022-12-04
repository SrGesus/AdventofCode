# Simple shell script that generates the files for the day in $1

create_file () {
    if test -f "$FILE"; then
        echo "$FILE already exists."
    else
        echo 'use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {' >> $FILE

        echo $(printf 'let file = File::open("./resources/day%02d/input.txt")?;' $1) >> $FILE

        echo '    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}' >> $FILE

    fi
}

FILE=$(printf './src/bin/day%02d-1.rs' $1)

create_file $1

FILE=$(printf './src/bin/day%02d-2.rs' $1)

create_file $1

mkdir -p $(printf './resources/day%02d/' $1)
touch $(printf './resources/day%02d/input.txt' $1)