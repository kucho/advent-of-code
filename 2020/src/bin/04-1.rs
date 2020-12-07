use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<&str> = input.split("\n\n").collect();
    let mut valid_passports = 0;

    for line in lines {
        let passport = str::replace(line, "\n", " ");
        if is_valid_passport(&passport) {
            valid_passports += 1;
        }
    }

    println!("Valid: #{}", valid_passports)
}

fn is_valid_passport(passport_str: &str) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let pairs = passport_str.trim().split(" ");
    let keys: HashSet<&str> = pairs
        .map(|pair| pair.trim().split(":").next().unwrap())
        .collect();
    required_fields.iter().all(|field| keys.contains(field))
}
