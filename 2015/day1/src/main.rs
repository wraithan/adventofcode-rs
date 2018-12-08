use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: floor {}", process_a(input.clone()));
    println!("Part B: instruction {}", process_b(input.clone()));
}

fn process_a(input: String) -> isize {
    input.chars().fold(0, |floor, instruction| {
        match instruction {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor
        }
    })
}

fn process_b(input: String) -> isize {
    let answer = input.chars().scan(0, |floor, instruction| {
        *floor = match instruction {
            '(' => *floor + 1,
            ')' => *floor - 1,
            _ => *floor
        };
        Some(*floor)
    }).position(|floor| floor == -1).unwrap_or(0);
    answer as isize + 1
}

#[test]
fn example_a_1() {
    assert_eq!(process_a("(())".to_owned()), 0);
    assert_eq!(process_a("()()".to_owned()), 0);
}

#[test]
fn example_a_2() {
    assert_eq!(process_a("(((".to_owned()), 3);
    assert_eq!(process_a("(()(()(".to_owned()), 3);
}

#[test]
fn example_a_3() {
    assert_eq!(process_a("))(((((".to_owned()), 3);
}

#[test]
fn example_a_4() {
    assert_eq!(process_a("())".to_owned()), -1);
    assert_eq!(process_a("))(".to_owned()), -1);
}

#[test]
fn example_a_5() {
    assert_eq!(process_a(")))".to_owned()), -3);
    assert_eq!(process_a(")())())".to_owned()), -3);
}

#[test]
fn example_b_1() {
    assert_eq!(process_b(")".to_owned()), 1);
}
#[test]
fn example_b_2() {
    assert_eq!(process_b("()())".to_owned()), 5);
}
