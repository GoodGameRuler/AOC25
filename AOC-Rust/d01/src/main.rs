extern crate nom;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, value},
    sequence::pair,
};
use std::env;
use std::fs;

pub struct Rotation {
    pub sign: bool,
    pub degree: u8,
}

fn main() {
    assert_eq!(match_rotation("R10"), Ok(("", 10i32)));
    assert_eq!(match_rotation("L99"), Ok(("", -99i32)));

    let args: Vec<String> = env::args().collect();
    let lines = fs::read_to_string(&args[1]).expect("Wasn't able to read file passed in");

    dbg!(lines.match_indices("\n").count());
    let lines = lines.strip_suffix("\n").unwrap_or(&lines);
    let rotations: Vec<i32> = lines
        .split("\n")
        .map(match_rotation)
        .map(|s| s.unwrap().1)
        .collect();
    dbg!(rotations.len());

    println!("{:?}", calculate_rot(&rotations));
}

fn match_rotation(input: &str) -> IResult<&str, i32> {
    map_res(
        pair(alt((value(1i32, tag("R")), value(-1i32, tag("L")))), digit1),
        |(sign, digits): (i32, &str)| digits.parse::<i32>().map(|n| sign * n),
    )
    .parse(input)
}

fn calculate_rot(rotations: &Vec<i32>) -> (u32, u32) {
    let mut degree: i32 = 50;
    let mut count_zero: u32 = 0;
    let mut count_crossing: u32 = 0;
    for rot in rotations {
        let abs_rot = rot.abs() as u32;
        degree = degree + rot % 100;

        if degree <= 0 || degree >= 100 {
            count_crossing += 1;

            if abs_rot > 100 {
                let remainder_rot = abs_rot.saturating_sub((rot.abs() % 100) as u32);
                count_crossing += remainder_rot / 100;
                println!(
                    "Count {:?}, Remainder {:?}, Rot {:?}, Abs {:?},",
                    count_crossing,
                    remainder_rot,
                    rot,
                    abs_rot % 100
                );
            }
        }
        degree = ((degree % 100) + 100) % 100;

        if degree == 0 {
            count_zero += 1;
        }
    }

    (count_zero, count_crossing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_rotation_basic() {
        assert_eq!(match_rotation("R10"), Ok(("", 10)));
        assert_eq!(match_rotation("L99"), Ok(("", -99)));
        assert_eq!(match_rotation("L139"), Ok(("", -139))); // tests > 100
    }

    #[test]
    fn test_mod_wrap_equivalence() {
        // simulate your degree wrapping
        let degree = 50;
        let rot = -50;
        let new_degree = ((degree + rot) % 100 + 100) % 100;
        assert_eq!(new_degree, 0); // -50 from 50 wraps to 0
    }

    #[test]
    fn test_calculate_rotations() {
        let rotations = vec![10, -50, 75, -200];
        let (count_zero, count_crossing) = calculate_rot(&rotations);
        // check expected outputs manually
        // 50+10=60, -50 → 10, 10+75 → 85, 85-200 → -115 -> wrap
        assert_eq!(count_zero, 0); // degree never exactly 0 after wrapping
        assert!(count_crossing > 0); // some crossings occurred
    }

    #[test]
    fn test_apply_parser_to_lines() {
        let input = "R10\nL50\nR75\nL200";
        let rotations: Vec<i32> = input
            .lines()
            .map(match_rotation)
            .map(|r| r.unwrap().1)
            .collect();

        let (_, count_crossing) = calculate_rot(&rotations);
        assert_eq!(rotations, vec![10, -50, 75, -200]);
        assert!(count_crossing > 0);
    }
}
