extern crate regex;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

use regex::Regex;
use serde_json::Value;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} total", process_a(input.clone()));
    println!("Part B: {} total", process_b(input));
}

fn process_a(input: String) -> isize {
    let expression = Regex::new(r"(-?\d+)").unwrap();

    expression.captures_iter(&input).fold(0, |acc, caps| {
        match isize::from_str_radix(caps.at(0).unwrap(), 10) {
            Ok(num) => acc + num,
            Err(e) => {
                println!("error parsing: {}", e);
                acc
            }
        }
    })
}

fn process_b(input: String) -> isize {
    let parsed: Value = serde_json::from_str(&input).unwrap();
    handle_value(&parsed)
}

fn handle_value (val: &Value) -> isize {
    match val {
        &Value::I64(n) => n as isize,
        &Value::U64(n) => n as isize,
        &Value::Object(ref obj) => {
            obj.values().fold(ObjState::Happy(0), |acc, v| {
                if let ObjState::Happy(n) = acc {
                    match v {
                        &Value::String(ref s) => {
                            if s == "red" {
                                ObjState::Sad
                            } else {
                                acc
                            }
                        }
                        _ => ObjState::Happy(n + handle_value(v))
                    }
                } else {
                    acc
                }
            }).value()
        }
        &Value::Array(ref arr) => {
            arr.iter().fold(0, |acc, v| {
                acc + handle_value(v)
            })
        }
        _ => 0
    }
}

enum ObjState {
    Happy(isize),
    Sad
}

impl ObjState {
    fn value(self) -> isize {
        match self {
            ObjState::Happy(n) => n,
            ObjState::Sad => 0
        }
    }
}

#[test]
fn example_a_1() {
    assert_eq!(process_a(r#"[1,2,3]"#.to_owned()), 6);
}

#[test]
fn example_a_2() {
    assert_eq!(process_a(r#"{"a":2,"b":4}"#.to_owned()), 6)
}

#[test]
fn example_a_3() {
    assert_eq!(process_a(r#"[[[3]]]"#.to_owned()), 3)
}

#[test]
fn example_a_4() {
    assert_eq!(process_a(r#"{"a":{"b":4},"c":-1}"#.to_owned()), 3)
}

#[test]
fn example_a_5() {
    assert_eq!(process_a(r#"{"a":[-1,1]}"#.to_owned()), 0)
}

#[test]
fn example_a_6() {
    assert_eq!(process_a(r#"[-1,{"a":1}]"#.to_owned()), 0)
}

#[test]
fn example_a_7() {
    assert_eq!(process_a(r#"[]"#.to_owned()), 0);
}

#[test]
fn example_a_8() {
    assert_eq!(process_a(r#"{}"#.to_owned()), 0);
}

#[test]
fn example_b_1() {
    assert_eq!(process_b(r#"[1,2,3]"#.to_owned()), 6);
}

#[test]
fn example_b_2() {
    assert_eq!(process_b(r#"[1,{"c":"red","b":2},3]"#.to_owned()), 4);
}

#[test]
fn example_b_3() {
    assert_eq!(process_b(r#"{"d":"red","e":[1,2,3,4],"f":5}"#.to_owned()), 0);
}

#[test]
fn example_b_4() {
    assert_eq!(process_b(r#"[1,"red",5]"#.to_owned()), 6);
}
