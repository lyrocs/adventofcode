use adventofcode::read_lines;

fn main() {
    let lines = read_lines(1);

    let part1 = solve_part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&lines);
    println!("Part 2: {}", part2);
}

fn solve_part1(lines: &[String]) -> i64 {
    // Example: sum all numbers in the input
    lines
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .sum()
}

fn solve_part2(lines: &[String]) -> i64 {
    // Example: find the first number that appears twice in cumulative sum
    let numbers: Vec<i64> = lines
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();

    let mut seen = std::collections::HashSet::new();
    let mut sum = 0;
    seen.insert(0);

    loop {
        for n in &numbers {
            sum += n;
            if !seen.insert(sum) {
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ];
        assert_eq!(solve_part1(&input), 6);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            String::from("1"),
            String::from("-1"),
        ];
        assert_eq!(solve_part2(&input), 0);
    }
}
