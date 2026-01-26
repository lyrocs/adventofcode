use adventofcode::read_lines;

fn main() {
    let lines = read_lines(1);

    let part1 = solve_part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&lines);
    println!("Part 2: {}", part2);
}

fn solve_part1(lines: &[String]) -> i64 {

    let mut r = 50;
    let mut result = 0;
    for line in lines {
        if line.contains("R") {
            let number = line.strip_prefix('R').unwrap_or(line).parse::<i64>().ok().unwrap_or(0);
            r += number % 100
        }
        if line.contains("L") {
            let number = line.strip_prefix('L').unwrap_or(line).parse::<i64>().ok().unwrap_or(0);
            r -= number % 100
        }
        if r < 0 {
            r = 100 + r;
        }
        if r > 99 {
            r = r - 100;
        }
        if r == 0 {
            result += 1;
        }
    }
    return result;
}

fn solve_part2(lines: &[String]) -> i64 {
    let mut r = 50;
    let mut result = 0;
    for line in lines {
        if line.contains("R") {
            let number = line.strip_prefix('R').unwrap_or(line).parse::<i64>().ok().unwrap_or(0);
            if number > 100 {
                result += number / 100
            }
            let rest = number % 100;
            if (r + rest) >= 100 {
                r = (r + rest) % 100;
                result += 1
            } else {
                r += rest
            }
        }
        if line.contains("L") {
            let number = line.strip_prefix('L').unwrap_or(line).parse::<i64>().ok().unwrap_or(0);
             if number > 100 {
                result += number / 100
            }
            let rest = number % 100;
            if (r - rest) < 0 {
                if r != 0 {
                    result += 1;
                }
                r = 100 + r - rest;
                
            } else {
                r -= rest;
                if r == 0 {
                    result += 1;
                }
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_lines(1);
        assert_eq!(solve_part1(&input), 1132);
    }

    #[test]
    fn test_part2() {
        let input = read_lines(1);
        assert_eq!(solve_part2(&input), 6623);
    }
}
