use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: floor {}", process_a(input));
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
