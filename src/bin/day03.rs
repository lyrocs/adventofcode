use core::num;

use adventofcode::read_lines;

fn main() {
    let lines = read_lines(3);

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

    for line in lines {
        let mut first = 0;
        let mut second = 0;

        for (i, item) in line.chars().enumerate() {
            let number = item.to_digit(10).unwrap() as i64;
            if first < number && i + 1 < line.len() {
                first = number;
                second = 0;
            } else if second < number {
                second = number;
            }
        }
        result += first * 10 + second;
    }
    return result;
}

fn solve_part2(lines: &[String]) -> i64 {
    let mut result = 0;
    if lines.len() == 0 {
        return 0;
    }

    for line in lines {
        let mut temp = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        for (i, item) in line.chars().enumerate() {
            let number = item.to_digit(10).unwrap() as i64;

            let mut maxindex: i64 = 0;
            if line.len() - i < 12 {
                maxindex = (12 - (line.len() - i)) as i64;
            }

            let mut edit = false;
            for i in maxindex..temp.len() as i64 {
                if edit == true {
                    temp[i as usize] = 0;
                } else if number > temp[i as usize] {
                    temp[i as usize] = number;
                    edit = true;
                }
            }
        }
        let mut total = 0;
        for i in 0..temp.len() as i64 {
            total += 10_i64.pow((temp.len() - 1 - i as usize) as u32) * temp[i as usize]
        }
        result += total;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_lines(3);
        assert_eq!(solve_part1(&input), 17031);
    }

    #[test]
    fn test_part2() {
        let input = read_lines(3);
        assert_eq!(solve_part2(&input), 168575096286051);
    }
}
