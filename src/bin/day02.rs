use adventofcode::read_lines;

fn main() {
    let lines = read_lines(2);

    let part1 = solve_part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&lines);
    println!("Part 2: {}", part2);
}

fn solve_part1(lines: &[String]) -> i64 {
    let mut result = 0;
    if lines.len() == 0 {
        return 0;
    }
    let ranges = lines[0].split(',').collect::<Vec<&str>>();

    for range in ranges {
        let mut iter = range.split('-');
        let first = iter.next().unwrap_or("0").parse::<i64>().ok().unwrap_or(0);
        let last = iter.next().unwrap_or("0").parse::<i64>().ok().unwrap_or(0);
        if first.to_string().len() % 2 > 0 && last.to_string().len() % 2 > 0 {
            continue;
        }
        for i in first..=last {
            let to_str = i.to_string();
            let len = to_str.len();
            if len % 2 > 0 {
                continue;
            }
            let (left, right) = to_str.split_at(len / 2);
            if left == right {
                result += i;
            }
        }
    }

    return result;
}

fn solve_part2(lines: &[String]) -> i64 {
    let mut result = 0;
    if lines.len() == 0 {
        return 0;
    }
    let ranges = lines[0].split(',').collect::<Vec<&str>>();

    for range in ranges {
        let mut iter = range.split('-');
        let first = iter.next().unwrap_or("0").parse::<i64>().ok().unwrap_or(0);
        let last = iter.next().unwrap_or("0").parse::<i64>().ok().unwrap_or(0);
        for i in first..=last {
            if recursive_repeat(i, 2) {
                result += i;
            }
        }
    }

    return result;
}

fn recursive_repeat(number: i64, divided: i64) -> bool {
    let to_str = number.to_string();
    if divided as usize > to_str.len() {
        return false;
    }
    if to_str.len() % divided as usize > 0 {
        return recursive_repeat(number, divided + 1);
    }
    let chars: Vec<char> = to_str.chars().collect();
    let chunks = chars
        .chunks(to_str.len() / divided as usize)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    if chunks.iter().all(|item| *item == chunks[0]) {
        return true;
    }

    return recursive_repeat(number, divided + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_lines(2);
        assert_eq!(solve_part1(&input), 28846518423);
    }

    #[test]
    fn test_part2() {
        let input = read_lines(2);
        assert_eq!(solve_part2(&input), 31578210022);
    }
}
