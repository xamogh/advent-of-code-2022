use std::fs;

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

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Error while reading file");
    let result = part_one(&contents);

    // println!("{:?}", get_points(&'Z'));
    println!("{:?}", result);
}
