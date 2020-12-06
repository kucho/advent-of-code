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
        for number2 in &numbers {
            if number == number2 || number + number2 >= 2020 {
                continue;
            }
            let number3 = 2020 - number - number2;
            if numbers.contains(&number3) {
                println!("{}", number * number2 * number3);
                exit(1);
            }
        }
    }
}
