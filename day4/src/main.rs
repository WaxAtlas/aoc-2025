fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");

    let res = solve_part1(&input);
    println!("{}", res);
    let res = solve_part2(&input);
    println!("{}", res);
}

fn solve_part1(input: &str) -> u64 {
    let mut sum: u64 = 0;

    sum
}

fn solve_part2(input: &str) -> u64 {
    let mut sum: u64 = 0;

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_test() {
        let input = std::fs::read_to_string("example.txt").expect("Error reading input.txt");
        assert_eq!(13, solve_part1(&input));
    }

    #[test]
    fn example_part2_test() {
        let input = std::fs::read_to_string("example.txt").expect("Error reading input.txt");
        assert_eq!(3121910778619, solve_part2(&input));
    }
}
