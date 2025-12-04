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

        match direction {
            "L" => {
                for _ in 0..value {
                    dial_position = (dial_position - 1 as i32).rem_euclid(100);
                    if dial_position == 0 {
                        count += 1;
                    }
                }
            }
            "R" => {
                for _ in 0..value {
                    dial_position = (dial_position + 1 as i32).rem_euclid(100);
                    if dial_position == 0 {
                        count += 1;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", count);
}
