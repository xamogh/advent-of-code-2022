use std::fs;

fn part_one(contents: &str) -> i32 {
    contents
        .split("\n\n")
        .map(|el| el.lines().map(|v| v.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap()
}

fn part_two(contents: &str) -> i32 {
    let mut r: Vec<i32> = contents
        .split("\n\n")
        .map(|el| el.lines().map(|v| v.parse::<i32>().unwrap()).sum::<i32>())
        .collect();
    r.sort();
    r.into_iter().rev().take(3).sum()
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Error while reading file");
    let output = part_one(&contents);
    let output2 = part_two(&contents);
    print!("{}", output2);
}
