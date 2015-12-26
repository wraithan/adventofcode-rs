extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} sq ft", process_a(input.clone()));
    println!("Part B: {} ft", process_b(input.clone()));
}

fn process_a(input: String) -> usize {
    let expression = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();
    input
        .lines()
        .filter_map(|line| {
            if let Some(caps) = expression.captures(line) {
                return Some((usize::from_str_radix(caps.at(1).unwrap(), 10).unwrap(),
                             usize::from_str_radix(caps.at(2).unwrap(), 10).unwrap(),
                             usize::from_str_radix(caps.at(3).unwrap(), 10).unwrap()))
            }
            None
        })
        .fold(0, |acc, item| {
            let sides = [item.0 * item.1,
                         item.0 * item.2,
                         item.1 * item.2];
            let total = sides.iter().fold(0, |acc, side| acc + side);
            acc + sides.iter().min().unwrap() + (total * 2)
        })
}

fn process_b(input: String) -> usize {
    input.len()
}

#[test]
fn example_a_1() {
    assert_eq!(process_a("2x3x4".to_owned()), 58);
}

#[test]
fn example_a_2() {
    assert_eq!(process_a("1x1x10".to_owned()), 43);
}

#[test]
fn example_b_1() {
    assert_eq!(process_b("2x3x4".to_owned()), 34);
}

#[test]
fn example_b_2() {
    assert_eq!(process_b("1x1x10".to_owned()), 14);
}
