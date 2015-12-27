use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} are nice", process_a(input.clone()));
    println!("Part B: {} are nice", process_b(input.clone()));
}

fn process_a(input: String) -> usize {
    input.lines().fold(0, |acc, word| {
        let mut last = ' ';
        let mut double = false;
        let mut vowels = 0;
        for letter in word.chars() {
            match last {
                'a' => {
                    if letter == 'b' {
                        return acc
                    }
                },
                'c' => {
                    if letter == 'd' {
                        return acc
                    }
                },
                'p' => {
                    if letter == 'q' {
                        return acc
                    }
                },
                'x' => {
                    if letter == 'y' {
                        return acc
                    }
                }
                _ => {}
            }
            match letter {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => {}
            }
            if last == letter {
                double = true
            }
            last = letter
        }
        if double && vowels >= 3 {
            return acc + 1
        }
        acc
    })
}

fn process_b(input: String) -> usize {
    input.lines().fold(0, |acc, word| {
        let mut last_1 = ' ';
        let mut last_2 = ' ';
        let mut set = false;
        let mut pair = false;

        for (i, letter) in word.chars().enumerate() {
            if last_2 == letter {
                set = true
            }
            if i < word.rfind(&format!("{}{}", last_1, letter)).unwrap_or(0) {
                pair = true;
            }
            last_2 = last_1;
            last_1 = letter;
        }
        if set && pair {
            return acc + 1
        }
        acc
    })
}

#[test]
fn example_a_1() {
    assert_eq!(process_a("ugknbfddgicrmopn".to_owned()), 1)
}

#[test]
fn example_a_2() {
    assert_eq!(process_a("aaa".to_owned()), 1)
}

#[test]
fn example_a_3() {
    assert_eq!(process_a("jchzalrnumimnmhp".to_owned()), 0)
}

#[test]
fn example_a_4() {
    assert_eq!(process_a("haegwjzuvuyypxyu".to_owned()), 0)
}

#[test]
fn example_a_5() {
    assert_eq!(process_a("dvszwmarrgswjxmb".to_owned()), 0)
}

#[test]
fn example_b_1() {
    assert_eq!(process_b("qjhvhtzxzqqjkmpb".to_owned()), 1)
}

#[test]
fn example_b_2() {
    assert_eq!(process_b("xxyxx".to_owned()), 1)
}

#[test]
fn example_b_3() {
    assert_eq!(process_b("uurcxstgmygtbstg".to_owned()), 0)
}

#[test]
fn example_b_4() {
    assert_eq!(process_b("ieodomkazucvgmuy".to_owned()), 0)
}
