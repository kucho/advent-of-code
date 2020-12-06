use std::collections::HashSet;
use std::io::{stdin, Read};
use std::process::exit;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let numbers = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<HashSet<u32>>();

    for number in &numbers {
        let complement = 2020 - number;
        if numbers.contains(&complement) {
            println!("{}", number * complement);
            exit(1);
        }
    }
}
