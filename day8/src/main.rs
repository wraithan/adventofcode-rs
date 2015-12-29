use std::fs::File;
use std::io::Read;

type Pair = (usize, usize);

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let result_a = process_a(input.clone());
    println!("Part A: {} syntax overhead", result_a.0 - result_a.1);
    let result_b = process_b(input.clone());
    println!("Part B: {} syntax overhead", result_b.1 - result_b.0);
}

fn process_a(input: String) -> Pair {
    input.lines().fold((0, 0), |acc, line| {
        let code = line.len();
        let mut value = 0;
        let mut state = State::Normal;
        let last = line.len() - 1;
        for (i, c) in line.chars().enumerate() {
            if i == 0 && c == '"' {
                continue;
            } else if i == last && c == '"' {
                continue;
            }
            match state {
                State::Normal => {
                    match c {
                        '\\' => {
                            state = State::MaybeSingle;
                        }
                        _ => {
                            value += 1;
                        }
                    }
                },
                State::MaybeSingle => {
                    match c {
                        '\\' | '"' => {
                            state = State::Normal;
                            value += 1;
                        },
                        'x' => {
                            state = State::MaybeStartDouble;
                        },
                        _ => {
                            state = State::Normal;
                            value += 2;
                        }
                    }
                },
                State::MaybeStartDouble => {
                    match c {
                        '0' | '1' | '2' | '3' | '4' | '5' |
                        '6' | '7' | '8' | '9' | 'a' | 'b' |
                        'c' | 'd' | 'e' | 'f' => {
                            state = State::MaybeDouble;
                        }
                        _ => {
                            state = State::Normal;
                            value += 3;
                        }
                    }
                },
                State::MaybeDouble => {
                    state = State::Normal;
                    match c {
                        '0' | '1' | '2' | '3' | '4' | '5' |
                        '6' | '7' | '8' | '9' | 'a' | 'b' |
                        'c' | 'd' | 'e' | 'f' => {
                            value += 1;
                        }
                        _ => {
                            value += 4;
                        }
                    }
                }
            }
        }
        (acc.0 + code, acc.1 + value)

    })
}

fn process_b(input: String) -> Pair {
    input.lines().fold((0, 0), |acc, line| {
        let code = line.len();
        let mut value = 2;
        for c in line.chars() {
            match c {
                '\\' => {
                    value += 2;
                }
                '"' => {
                    value += 2;
                }
                _ => {
                    value += 1;
                }
            }
        }
        (acc.0 + code, acc.1 + value)

    })
}

#[derive(Debug)]
enum State {
    Normal,
    MaybeSingle,
    MaybeStartDouble,
    MaybeDouble,
}

#[cfg(test)]
mod tests {
    use super::{Pair, process_a, process_b};

    fn run_a(input: &str, expected: Pair) {
        assert_eq!(process_a(input.to_owned()), expected);
    }

    fn run_b(input: &str, expected: Pair) {
        assert_eq!(process_b(input.to_owned()), expected);
    }

    #[test]
    fn example_a_1() {
        run_a(r#""""#, (2, 0));
    }

    #[test]
    fn example_a_2() {
        run_a(r#""abc""#, (5, 3));
    }

    #[test]
    fn example_a_3() {
        run_a(r#""aaa\"aaa""#, (10, 7));
    }

    #[test]
    fn example_a_4() {
        run_a(r#""\x27""#, (6, 1));
    }

    #[test]
    fn example_b_1() {
        run_b(r#""""#, (2, 6));
    }

    #[test]
    fn example_b_2() {
        run_b(r#""abc""#, (5, 9));
    }

    #[test]
    fn example_b_3() {
        run_b(r#""aaa\"aaa""#, (10, 16));
    }

    #[test]
    fn example_b_4() {
        run_b(r#""\x27""#, (6, 11));
    }
}
