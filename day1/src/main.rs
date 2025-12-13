fn solve_part1(input: &str) -> i32 {
    let mut dial_position = 50;
    let mut count = 0;

    for line in input.lines() {
        let (direction, turns) = line.split_at(1);

        let value = turns.parse::<i32>().expect("Error parsing String to i32");

        match direction {
            "L" => {
                for _ in 0..value {
                    dial_position = (dial_position - 1 as i32).rem_euclid(100);
                }
            }
            "R" => {
                for _ in 0..value {
                    dial_position = (dial_position + 1 as i32).rem_euclid(100);
                }
            }
            _ => unreachable!(),
        }
        if dial_position == 0 {
            count += 1;
        }
    }
    count
}

fn solve_part2(input: &str) -> i32 {
    let mut dial_position = 50;
    let mut count = 0;

    for line in input.lines() {
        let (direction, turns) = line.split_at(1);

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
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input error");

    let count = solve_part1(&input);
    println!("Part 1 password: {}", count);

    let count = solve_part2(&input);
    println!("Part 2 password: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_test() {
        let input = std::fs::read_to_string("example.txt").expect("Input error");
        assert_eq!(solve_part1(&input), 3);
    }

    #[test]
    fn example_part2_test() {
        let input = std::fs::read_to_string("example.txt").expect("Input error");
        assert_eq!(solve_part2(&input), 6);
    }
}
