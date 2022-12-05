use std::{collections::HashSet, fs};

fn get_points(c: &char) -> i32 {
    if c.is_uppercase() {
        (*c as u32) as i32 - 38
    } else {
        (*c as u32) as i32 - 96
    }
}

fn part_one(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let s = String::from(line);
            let size = line.chars().count();
            let first_half = &s[0..(size / 2)];
            let second_half = &s[size / 2..size];

            let same_char = first_half.chars().find(|&c| second_half.contains(c));

            let points = match same_char {
                Some(v) => get_points(&v),
                None => 0,
            };

            points
        })
        .sum()
}

fn part_two(contents: &str) -> i32 {
    let lines_vec: Vec<&str> = contents.lines().collect();
    let mut sum = 0;
    for (idx, _) in contents.lines().enumerate().step_by(3) {
        let set_first: HashSet<char> = HashSet::from_iter(lines_vec[idx].chars());
        let set_second: HashSet<char> = HashSet::from_iter(lines_vec[idx + 1].chars());
        let set_third: HashSet<char> = HashSet::from_iter(lines_vec[idx].chars());
        let common_char = set_first
            .iter()
            .find(|x| set_second.contains(x) && set_third.contains(x));
        let points = match common_char {
            Some(v) => get_points(v),
            None => 0,
        };
        sum += points;
    }
    sum
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Error while reading file");
    // let result = part_one(&contents);
    let result = part_two(&contents);

    println!("{result}");
}
