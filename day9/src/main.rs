extern crate permutohedron;

use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use permutohedron::Heap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let result = process_a(input.clone());
    println!("Part A: {} distance", result.0);
    println!("Part B: {} distance", result.1);
}

macro_rules! get_or_continue(
    ($e:expr) => (match $e { Some(e) => e, None => continue })
);


fn process_a(input: String) -> (usize, usize) {
    let mut distances: HashMap<(&str, &str), usize> = HashMap::new();
    let mut keys = HashSet::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let first = get_or_continue!(words.next());
        if get_or_continue!(words.next()) != "to" {
            continue
        }
        let second = get_or_continue!(words.next());
        if get_or_continue!(words.next()) != "=" {
            continue
        }
        let value = get_or_continue!(words.next());
        if let Ok(num) = usize::from_str_radix(value, 10) {
            keys.insert(first);
            keys.insert(second);
            distances.insert((first, second), num);
            distances.insert((second, first), num);
        }
    }
    let mut results = HashSet::new();
    let mut tmp_keys = Vec::from_iter(keys.into_iter());
    let heap = Heap::new(&mut tmp_keys);
    for mixed_keys in heap {
        let mut last = "";
        let mut total = 0;
        for (i, key) in mixed_keys.iter().enumerate() {
            if i == 0 {
                last = key.clone();
                continue
            }
            total += distances.get(&(last, key.clone())).unwrap().clone();
            last = key.clone();
        }
        results.insert(total);
    }
    (results.iter().min().unwrap_or(&0).clone(), results.iter().max().unwrap_or(&0).clone())
}

#[test]
fn example_a() {
    let input = "London to Dublin = 464\n\
                 London to Belfast = 518\n\
                 Dublin to Belfast = 141";
    assert_eq!(process_a(input.to_owned()), (605, 982));
}
