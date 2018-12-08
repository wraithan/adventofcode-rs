extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let input = "bgvyzdsv";
    println!("Answer A: {}", process_a(input));
    println!("Answer B: {}", process_b(input));
}

fn process_a(input: &'static str) -> usize {
    with_leader(input, "00000")
}

fn process_b(input: &'static str) -> usize {
    with_leader(input, "000000")
}

fn with_leader(input: &'static str, leader: &'static str) -> usize {
    let mut result = "".to_owned();
    let mut md5 = Md5::new();
    let mut i = 0;
    while !result.starts_with(leader) {
        i += 1;
        md5.input_str(&format!("{}{}", input, i));
        result = md5.result_str();
        md5.reset();
    }
    i
}

#[test]
fn example_a_1() {
    assert_eq!(process_a("abcdef"), 609043);
}

#[test]
fn example_a_2() {
    assert_eq!(process_a("pqrstuv"), 1048970);
}
