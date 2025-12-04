use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("document.txt").expect("Unable to open document.txt");
    let reader = BufReader::new(file);

    let mut dial_position = 50;
    let mut count = 0;

    for line in reader.lines() {
        let instruction = line.as_ref().expect("Error");
        let (direction, turns) = instruction.split_at(1);

        let value = turns.parse::<i32>().expect("Error parsing String to i32");

        count += (value) / 100;

        match direction {
            "L" => {
                if dial_position - value % 100 < 0 {
                    count += 1;
                }
                dial_position += 100 - value;
            }
            "R" => {
                if dial_position + value % 100 > 99 {
                    count += 1;
                }
                dial_position += value;
            }
            _ => unreachable!(),
        }

        dial_position %= 100;

        if dial_position == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
