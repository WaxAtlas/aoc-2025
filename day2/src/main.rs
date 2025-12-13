fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");

    let solution = solve_part1(&input);
    println!("Part 1 solution: {}", solution);
    let solution = solve_part2(&input);
    println!("Part 2 solution: {}", solution);
}

fn solve_part1(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.trim().split(",") {
        let (start, end) = range.split_once("-").expect("Error splitting range");
        for num in
            start.parse::<u64>().expect("Start error")..=end.parse::<u64>().expect("End error")
        {
            let num_digits = num.ilog10() + 1;
            if num_digits % 2 == 0 {
                let n = 10u64.pow(num_digits / 2);
                if num / n == num % n {
                    sum += num;
                }
            }
        }
    }
    sum
}

fn solve_part2(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.trim().split(",") {
        let (start, end) = range.split_once("-").expect("Error splitting range");
        for num in
            start.parse::<u64>().expect("Start error")..=end.parse::<u64>().expect("End error")
        {
            let num_digits = num.ilog10() + 1;
            let mut remainder;
            'outer: for i in 1..=num_digits / 2 {
                let n = 10u64.pow(i);

                if num_digits % i != 0 {
                    continue 'outer;
                }

                let first_remainder = num % n;
                remainder = num % n;

                let mut inner_num = num;
                while remainder == first_remainder {
                    inner_num = (inner_num - remainder) / n;
                    remainder = inner_num % n;
                    if inner_num == 0 {
                        sum += num;
                        break 'outer;
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_test() {
        let input = std::fs::read_to_string("example.txt").expect("Error reading example.txt");
        assert_eq!(solve_part1(&input), 1227775554);
    }

    #[test]
    fn example_part2_test() {
        let input = std::fs::read_to_string("example.txt").expect("Error reading example.txt");
        assert_eq!(solve_part2(&input), 4174379265);
    }
}
