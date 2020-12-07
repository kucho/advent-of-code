use std::collections::HashMap;
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

    println!("Valid docs: {}", valid_passports)
}

fn is_valid_passport(passport_str: &str) -> bool {
    let pairs = passport_str.trim().split(" ");
    let mut required_fields: HashMap<&str, bool> = HashMap::new();
    required_fields.insert("byr", false);
    required_fields.insert("iyr", false);
    required_fields.insert("eyr", false);
    required_fields.insert("hgt", false);
    required_fields.insert("hcl", false);
    required_fields.insert("ecl", false);
    required_fields.insert("pid", false);

    for pair in pairs {
        let mut pair_it = pair.trim().split(":");
        let key = pair_it.next().unwrap();
        let value = pair_it.next().unwrap();

        if key == "cid" {
            continue;
        }

        if !required_fields.contains_key(key) {
            break;
        }

        let result = validate_data(key, value);

        required_fields.insert(key, result);
    }

    required_fields.values().all(|&result| result == true)
}

fn validate_data(key: &str, value: &str) -> bool {
    match key {
        "byr" => validate_date_range(value, 1920, 2002),
        "iyr" => validate_date_range(value, 2010, 2020),
        "eyr" => validate_date_range(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hair_color(value),
        "ecl" => validate_eye_color(value),
        "pid" => validate_id(value),
        _ => true,
    }
}

fn validate_date_range(value: &str, start: usize, end: usize) -> bool {
    let date: usize = value.parse().unwrap();
    date >= start && date <= end
}

fn validate_height(value: &str) -> bool {
    let data: Vec<_> = value.chars().map(|char| char.to_string()).collect();
    let unit_start: usize = data.len() - 2;
    let unit: String = data[unit_start..].join("");

    if unit != "cm" && unit != "in" {
        return false;
    }

    let number: usize = data[..unit_start].join("").parse().unwrap();

    match unit.as_ref() {
        "cm" => number >= 150 && number <= 193,
        "in" => number >= 59 && number <= 76,
        _ => false,
    }
}

fn validate_hair_color(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }

    let mut valid_chars: Vec<char> = ('a'..='f').collect();
    let mut valid_nums: Vec<char> = ('0'..='9').collect();
    valid_chars.append(&mut valid_nums);

    let mut valid = true;

    for (index, char) in value.chars().enumerate() {
        if index == 0 {
            if char != '#' {
                valid = false;
                break;
            } else {
                continue;
            }
        }

        if !valid_chars.contains(&char) {
            valid = false;
            break;
        }
    }

    valid
}

fn validate_eye_color(value: &str) -> bool {
    let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_colors.contains(&value)
}

fn validate_id(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }

    let valid_nums: Vec<char> = ('0'..='9').collect();

    for char in value.chars() {
        if !valid_nums.contains(&char) {
            return false;
        }
    }

    true
}
