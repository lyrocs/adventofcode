use std::fs;

/// Read input file for a given day
pub fn read_input(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).expect(&format!("Failed to read {}", path))
}

/// Parse input into lines
pub fn read_lines(day: u32) -> Vec<String> {
    read_input(day)
        .lines()
        .map(String::from)
        .collect()
}

/// Parse input into a vector of numbers
pub fn read_numbers(day: u32) -> Vec<i64> {
    read_input(day)
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}
