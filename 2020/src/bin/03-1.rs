use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut col_pos = 0;
    let mut tree_count = 0;
    let right = 3;

    for (index, row) in map.iter().enumerate() {
        if index == 0 {
            continue;
        }

        col_pos = (col_pos + right) % row.len();

        if row[col_pos] == '#' {
            tree_count += 1;
        }
    }

    println!("Tree count: {}", tree_count)
}
