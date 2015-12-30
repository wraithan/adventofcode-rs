use std::char;

fn main() {
    let input = "hxbxwxba".to_owned();
    let result = next_valid_str(input);
    println!("Part A: {} is the next password", result);
    println!("Part B: {} is the next password", next_valid_str(result));
}

fn next_char(input: char) -> char {
    let mut raw = input.clone() as u32;
    match input {
        'h' | 'k' | 'n' => raw += 2,
        'z' => raw = 'a' as u32,
        _ => raw += 1,
    }
    char::from_u32(raw).unwrap()
}

fn raw_next_char(input: char) -> char {
    let mut raw = input.clone() as u32;
    match input {
        'z' => raw = 'a' as u32,
        _ => raw += 1,
    }
    char::from_u32(raw).unwrap()
}

fn next_str(input: String) -> String {
    let mut vector: Vec<char> = input.chars().collect();
    let last_index = input.len() - 1;
    for (i, c) in input.chars().rev().enumerate() {
        let next = next_char(c);
        {
            let mut real = vector.get_mut(last_index - i).unwrap();
            *real = next;
            if *real != 'a' {
                break
            }
        }
        if i == last_index {
            vector.insert(0, 'a');
        }
    }
    let result: String = vector.into_iter().collect();
    result
}

fn validate_str(input: String) -> Result<String, (bool, bool, bool)> {
    let mut straight = false;
    let mut no_reserved = true;
    let mut double = false;
    let mut straight_state = StraightState::None;
    let mut double_state = DoubleState::None;
    for c in input.chars() {
        match c {
            'i' | 'l' | 'o' => no_reserved = false,
            _ => {}
        }

        straight_state = match straight_state {
            StraightState::None => StraightState::One(c),
            StraightState::One(last) => {
                if c == raw_next_char(last) {
                    StraightState::Pair(c)
                } else {
                    StraightState::One(c)
                }
            }
            StraightState::Pair(last) => {
                if c == raw_next_char(last) {
                    straight = true;
                    StraightState::Found
                } else {
                    StraightState::One(c)
                }
            }
            StraightState::Found => StraightState::Found
        };

        double_state = match double_state {
            DoubleState::None => DoubleState::First(c),
            DoubleState::First(last) => {
                if c != 'z' && c == last {
                    DoubleState::FirstFound
                } else {
                    DoubleState::First(c)
                }
            }
            DoubleState::FirstFound => DoubleState::Second(c),
            DoubleState::Second(last) => {
                if c != 'z' && c == last {
                    double = true;
                    DoubleState::SecondFound
                } else {
                    DoubleState::Second(c)
                }
            }
            DoubleState::SecondFound => DoubleState::SecondFound
        };
    }

    if straight && no_reserved && double {
        Ok(input)
    } else {
        Err((straight, no_reserved, double))
    }
}

enum StraightState {
    None,
    One(char),
    Pair(char),
    Found
}

enum DoubleState {
    None,
    First(char),
    FirstFound,
    Second(char),
    SecondFound
}

fn next_valid_str(input: String) -> String {
    let mut last = input;
    loop {
        let next = next_str(last.clone());

        if let Ok(valid_password) = validate_str(next.clone()) {
            return valid_password
        }

        last = next;
    }
}

#[test]
fn exercise_next_char() {
    assert_eq!(next_char('a'), 'b');
    assert_eq!(next_char('h'), 'j');
    assert_eq!(next_char('k'), 'm');
    assert_eq!(next_char('n'), 'p');
    assert_eq!(next_char('s'), 't');
    assert_eq!(next_char('z'), 'a');
}

#[test]
fn exercise_next_str() {
    assert_eq!(next_str("a".to_owned()), "b".to_owned());
    assert_eq!(next_str("h".to_owned()), "j".to_owned());
    assert_eq!(next_str("k".to_owned()), "m".to_owned());
    assert_eq!(next_str("n".to_owned()), "p".to_owned());
    assert_eq!(next_str("s".to_owned()), "t".to_owned());
    assert_eq!(next_str("z".to_owned()), "aa".to_owned());
    assert_eq!(next_str("ca".to_owned()), "cb".to_owned());
    assert_eq!(next_str("ch".to_owned()), "cj".to_owned());
    assert_eq!(next_str("ck".to_owned()), "cm".to_owned());
    assert_eq!(next_str("cn".to_owned()), "cp".to_owned());
    assert_eq!(next_str("cs".to_owned()), "ct".to_owned());
    assert_eq!(next_str("cz".to_owned()), "da".to_owned());
    assert_eq!(next_str("ha".to_owned()), "hb".to_owned());
    assert_eq!(next_str("hh".to_owned()), "hj".to_owned());
    assert_eq!(next_str("hk".to_owned()), "hm".to_owned());
    assert_eq!(next_str("hn".to_owned()), "hp".to_owned());
    assert_eq!(next_str("hs".to_owned()), "ht".to_owned());
    assert_eq!(next_str("hz".to_owned()), "ja".to_owned());
    assert_eq!(next_str("ia".to_owned()), "ib".to_owned());
}

#[test]
fn exercise_validate_str() {
    assert_eq!(validate_str("hijklmmn".to_owned()), Err((true, false, false)));
    assert_eq!(validate_str("abbceffg".to_owned()), Err((false, true, true)));
    assert_eq!(validate_str("abbcegjk".to_owned()), Err((false, true, false)));
    assert_eq!(validate_str("abbcegjk".to_owned()), Err((false, true, false)));
}

#[test]
fn exercise_next_valid() {
    assert_eq!(next_valid_str("abcdefgh".to_owned()), "abcdffaa".to_owned());
    assert_eq!(next_valid_str("ghijklmn".to_owned()), "ghjaabcc".to_owned());
}
