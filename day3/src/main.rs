fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");

    let res = solve_part1(&input);
    println!("{}", res);
    let res = solve_part2(&input);
    println!("{}", res);
}

fn solve_part1(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let mut highest_tens: u64 = 0;
        let mut highest_ones: u64 = 0;
        for (i, c) in line.char_indices() {
            let current_num: u64 = u64::from(c.to_digit(10).expect("Error parsing char"));

            if line.len() - 1 != i && highest_tens < current_num {
                highest_tens = current_num;
                highest_ones = 0;
                continue;
            }

            if highest_ones < current_num {
                highest_ones = current_num;
            }
        }

        sum += highest_tens * 10 + highest_ones;
    }
    sum
}

fn solve_part2(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let mut highest_12th: u64 = 0;
        let mut highest_11th: u64 = 0;
        let mut highest_10th: u64 = 0;
        let mut highest_9th: u64 = 0;
        let mut highest_8th: u64 = 0;
        let mut highest_7th: u64 = 0;
        let mut highest_6th: u64 = 0;
        let mut highest_5th: u64 = 0;
        let mut highest_4th: u64 = 0;
        let mut highest_3th: u64 = 0;
        let mut highest_2th: u64 = 0;
        let mut highest_1th: u64 = 0;
        for (i, c) in line.char_indices() {
            let current_num: u64 = u64::from(c.to_digit(10).expect("Error parsing char"));

            if line.len() - 11 > i && highest_12th < current_num {
                highest_12th = current_num;
                highest_11th = 0;
                continue;
            }
            if line.len() - 10 > i && highest_11th < current_num {
                highest_11th = current_num;
                highest_10th = 0;
                continue;
            }
            if line.len() - 9 > i && highest_10th < current_num {
                highest_10th = current_num;
                highest_9th = 0;
                continue;
            }
            if line.len() - 8 > i && highest_9th < current_num {
                highest_9th = current_num;
                highest_8th = 0;
                continue;
            }
            if line.len() - 7 > i && highest_8th < current_num {
                highest_8th = current_num;
                highest_7th = 0;
                continue;
            }
            if line.len() - 6 > i && highest_7th < current_num {
                highest_7th = current_num;
                highest_6th = 0;
                continue;
            }
            if line.len() - 5 > i && highest_6th < current_num {
                highest_6th = current_num;
                highest_5th = 0;
                continue;
            }
            if line.len() - 4 > i && highest_5th < current_num {
                highest_5th = current_num;
                highest_4th = 0;
                continue;
            }
            if line.len() - 3 > i && highest_4th < current_num {
                highest_4th = current_num;
                highest_3th = 0;
                continue;
            }
            if line.len() - 2 > i && highest_3th < current_num {
                highest_3th = current_num;
                highest_2th = 0;
                continue;
            }
            if line.len() - 1 > i && highest_2th < current_num {
                highest_2th = current_num;
                highest_1th = 0;
                continue;
            }
            if highest_1th < current_num {
                highest_1th = current_num;
            }
        }

        sum += highest_12th * 10u64.pow(11)
            + highest_11th * 10u64.pow(10)
            + highest_10th * 10u64.pow(9)
            + highest_9th * 10u64.pow(8)
            + highest_8th * 10u64.pow(7)
            + highest_7th * 10u64.pow(6)
            + highest_6th * 10u64.pow(5)
            + highest_5th * 10u64.pow(4)
            + highest_4th * 10u64.pow(3)
            + highest_3th * 10u64.pow(2)
            + highest_2th * 10
            + highest_1th;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_test() {
        let input = std::fs::read_to_string("example.txt").expect("Error reading input.txt");
        assert_eq!(357, solve_part1(&input));
    }

    #[test]
    fn example_part2_test() {
        let input = std::fs::read_to_string("example.txt").expect("Error reading input.txt");
        assert_eq!(3121910778619, solve_part2(&input));
    }
}
