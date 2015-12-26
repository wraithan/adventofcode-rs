use std::io::Read;
use std::fs::File;

fn main() {
    let mut part_a_file = File::open("part_a.txt").unwrap();
    let mut part_a_input = String::new();
    part_a_file.read_to_string(&mut part_a_input).unwrap();
    println!("Part A: floor {}", process_a(part_a_input));
}

fn process_a(input: String) -> isize {
    input.chars().fold(0, |acc, item| {
        match item {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        }
    })
}

#[test]
fn example_1() {
    assert_eq!(process_a("(())".to_owned()), 0);
    assert_eq!(process_a("()()".to_owned()), 0);
}

#[test]
fn example_2() {
    assert_eq!(process_a("(((".to_owned()), 3);
    assert_eq!(process_a("(()(()(".to_owned()), 3);
}

#[test]
fn example_3() {
    assert_eq!(process_a("))(((((".to_owned()), 3);
}

#[test]
fn example_4() {
    assert_eq!(process_a("())".to_owned()), -1);
    assert_eq!(process_a("))(".to_owned()), -1);
}

#[test]
fn example_5() {
    assert_eq!(process_a(")))".to_owned()), -3);
    assert_eq!(process_a(")())())".to_owned()), -3);
}
