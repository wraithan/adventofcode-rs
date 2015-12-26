use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} sq ft", process_a(input.clone()));
    //println!("Part B: instruction {}", process_b(input.clone()));
}


fn process_a(input: String) -> usize {
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
