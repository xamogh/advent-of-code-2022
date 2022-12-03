use std::{fs, str::FromStr};

#[derive(Debug, Copy, Clone)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Play {
    pub fn what_is_my_move(opponent: &Play, v: &str) -> Play {
        match v {
            "X" => match opponent {
                Play::Paper => Play::Rock,
                Play::Rock => Play::Scissor,
                Play::Scissor => Play::Paper,
            },
            "Y" => *opponent,
            "Z" => match opponent {
                Play::Paper => Play::Scissor,
                Play::Rock => Play::Paper,
                Play::Scissor => Play::Rock,
            },
            _ => panic!("Err"),
        }
    }

    // self = me, other = other
    pub fn fight(&self, other: &Play) -> i32 {
        // draw case
        if *self as u8 == *other as u8 {
            return 3 + *self as i32;
        }
        // this rock beats other scissor case
        if *self as u8 == Play::Rock as u8 && *other as u8 == Play::Scissor as u8 {
            return 6 + *self as i32;
        }
        // other rock beats this scissor case
        if *self as u8 == Play::Scissor as u8 && *other as u8 == Play::Rock as u8 {
            return 0 + *self as i32;
        }
        // rest of the case
        if *self as u8 > *other as u8 {
            return 6 + *self as i32;
        } else {
            return 0 + *self as i32;
        }
    }
}

impl FromStr for Play {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissor),
            _ => Err("Error".to_string()),
        }
    }
    type Err = String;
}

fn part_one(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let mut moves = line.split(" ");
            let opp_move = moves.next().unwrap().parse::<Play>().unwrap();
            let my_move = moves.next().unwrap().parse::<Play>().unwrap();
            return my_move.fight(&opp_move);
        })
        .sum()
}

fn part_two(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let mut moves = line.split(" ");
            let opp_move = moves.next().unwrap().parse::<Play>().unwrap();
            let game_outcome = moves.next().unwrap();
            let my_move = Play::what_is_my_move(&opp_move, game_outcome);
            return my_move.fight(&opp_move);
        })
        .sum()
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Error while reading file");
    // let output = part_one(&contents);
    let output = part_two(&contents);
    println!("{:?}", output);
}
