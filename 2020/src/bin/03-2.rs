use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let slope_1 = count_trees(&map, 1, 1);
    let slope_2 = count_trees(&map, 3, 1);
    let slope_3 = count_trees(&map, 5, 1);
    let slope_4 = count_trees(&map, 7, 1);
    let slope_5 = count_trees(&map, 1, 2);
    println!("{}", slope_1 * slope_2 * slope_3 * slope_4 * slope_5)
}

fn count_trees(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut col_pos: usize = 0;
    let mut tree_count: usize = 0;

    for (index, row) in map.iter().enumerate().step_by(down) {
        if index == 0 {
            continue;
        }

        col_pos = (col_pos + right) % row.len();

        if row[col_pos] == '#' {
            tree_count += 1;
        }
    }

    tree_count
}
