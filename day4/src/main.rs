use std::{cmp, collections::HashSet, fs};

fn get_elf_vec(line: &str) -> (Vec<i32>, Vec<i32>) {
    let mut splitter = line.split(",");
    let first_elf = splitter.next().unwrap();
    let second_elf = splitter.next().unwrap();

    splitter = first_elf.split("-");
    let first_elf_start = splitter.next().unwrap().parse::<i32>().unwrap();
    let first_elf_end = splitter.next().unwrap().parse::<i32>().unwrap();

    splitter = second_elf.split("-");
    let second_elf_start = splitter.next().unwrap().parse::<i32>().unwrap();
    let second_elf_end = splitter.next().unwrap().parse::<i32>().unwrap();

    let mut elems_first_elf: Vec<i32> = Vec::new();

    for n in first_elf_start..first_elf_end + 1 {
        elems_first_elf.push(n);
    }

    let mut elems_second_elf: Vec<i32> = Vec::new();

    for n in second_elf_start..second_elf_end + 1 {
        elems_second_elf.push(n);
    }

    return (elems_first_elf, elems_second_elf);
}

fn part_one(contents: &str) -> i32 {
    let mut pair_count = 0;

    for line in contents.lines() {
        let (first_elf_work, second_elf_work) = get_elf_vec(line);
        let first_elf_work_set: HashSet<&i32> = HashSet::from_iter(&first_elf_work);
        let second_elf_work_set: HashSet<&i32> = HashSet::from_iter(&second_elf_work);

        if second_elf_work_set.is_superset(&first_elf_work_set)
            || first_elf_work_set.is_superset(&second_elf_work_set)
        {
            pair_count += 1;
        }
    }

    pair_count
}

fn part_two(contents: &str) -> i32 {
    let mut overlapping_pairs = 0;

    for line in contents.lines() {
        let (pair1, pair2) = line.split_once(",").unwrap();
        let (l1, l2) = pair1.split_once("-").unwrap();
        let (r1, r2) = pair2.split_once("-").unwrap();

        if cmp::max(l1.parse::<i32>().unwrap(), r1.parse::<i32>().unwrap())
            <= cmp::min(l2.parse::<i32>().unwrap(), r2.parse::<i32>().unwrap())
        {
            overlapping_pairs += 1;
        }
    }

    overlapping_pairs
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Error while reading file");
    // let result = part_one(&contents);
    let result = part_two(&contents);
    println!("{result}");
}
