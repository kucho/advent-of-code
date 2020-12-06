use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let valid_pws = input.lines().filter(|&line| parse_line(line)).count();
    println!("{}", valid_pws);
}

fn is_valid(min: usize, max: usize, letter: char, pw: &str) -> bool {
    let letters: Vec<char> = pw.chars().collect();
    let count = letters.iter().filter(|&c| c == &letter).count();
    count >= min && count <= max
}

fn parse_line(line: &str) -> bool {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    let range_str: Vec<&str> = parts[0].split("-").collect();
    let min = range_str[0].parse().unwrap();
    let max = range_str[1].parse().unwrap();
    let letter = parts[1].chars().next().unwrap();
    let pw = parts[2];
    is_valid(min, max, letter, pw)
}
