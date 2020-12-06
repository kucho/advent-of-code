use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let valid_pws = input.lines().filter(|&line| parse_line(line)).count();
    println!("{}", valid_pws);
}

fn is_valid(pos_a: usize, pos_b: usize, letter: char, pw: &str) -> bool {
    let letters: Vec<char> = pw.chars().collect();
    let valid_a = letter == letters[pos_a];
    let valid_b = letter == letters[pos_b];

    return if valid_a && valid_b {
        false
    } else {
        valid_a || valid_b
    };
}

fn parse_line(line: &str) -> bool {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    let range_str: Vec<&str> = parts[0].split("-").collect();
    let pos_a: usize = range_str[0].parse().unwrap();
    let pos_b: usize = range_str[1].parse().unwrap();
    let letter = parts[1].chars().next().unwrap();
    let pw = parts[2];
    is_valid(pos_a - 1, pos_b - 1, letter, pw)
}
