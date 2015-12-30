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
    println!("Part A: {} hapiness delta", process_a(input.clone()));
    println!("Part B: {} hapiness delta", process_b(input.clone()));
}

macro_rules! get_or_continue(
    ($e:expr) => (match $e { Some(e) => e, None => continue })
);


fn process_a(input: String) -> isize {
    let mut distances: HashMap<(&str, &str), isize> = HashMap::new();
    let mut keys = HashSet::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let first = get_or_continue!(words.next());
        let command = get_or_continue!(words.nth(1));
        let value = get_or_continue!(words.next());
        let mut second = get_or_continue!(words.nth(6));
        second = &second[..second.len()-1];
        if let Ok(mut num) = isize::from_str_radix(value, 10) {
            keys.insert(first);
            keys.insert(second);
            if command == "lose" {
                num = -num;
            }
            distances.insert((first, second), num);
        }
    }
    let mut results = HashSet::new();
    let mut tmp_keys = Vec::from_iter(keys.into_iter());
    let heap = Heap::new(&mut tmp_keys);
    for mixed_keys in heap {
        let last = mixed_keys.last().unwrap().clone();
        let mut prev = "";
        let mut total = 0;
        for (i, k) in mixed_keys.iter().enumerate() {
            let key = k.clone();
            if i == 0 {
                prev = key.clone();
                total = distances.get(&(last, key)).unwrap().clone();
                total += distances.get(&(key, last)).unwrap().clone();
                continue
            }
            total += distances.get(&(prev, key)).unwrap().clone();
            total += distances.get(&(key, prev)).unwrap().clone();
            prev = key.clone();
        }
        results.insert(total);
    }
    results.iter().max().unwrap_or(&0).clone()
}

fn process_b(input: String) -> isize {
    let mut distances: HashMap<(&str, &str), isize> = HashMap::new();
    let mut keys = HashSet::new();
    keys.insert("self");
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let first = get_or_continue!(words.next());
        let command = get_or_continue!(words.nth(1));
        let value = get_or_continue!(words.next());
        let mut second = get_or_continue!(words.nth(6));
        second = &second[..second.len()-1];
        if let Ok(mut num) = isize::from_str_radix(value, 10) {
            keys.insert(first);
            keys.insert(second);
            if command == "lose" {
                num = -num;
            }
            distances.insert((first, second), num);
            distances.insert((first, "self"), 0);
            distances.insert(("self", first), 0);
        }
    }
    let mut results = HashSet::new();
    let mut tmp_keys = Vec::from_iter(keys.into_iter());
    let heap = Heap::new(&mut tmp_keys);
    for mixed_keys in heap {
        let last = mixed_keys.last().unwrap().clone();
        let mut prev = "";
        let mut total = 0;
        for (i, k) in mixed_keys.iter().enumerate() {
            let key = k.clone();
            if i == 0 {
                prev = key.clone();
                total = distances.get(&(last, key)).unwrap().clone();
                total += distances.get(&(key, last)).unwrap().clone();
                continue
            }
            total += distances.get(&(prev, key)).unwrap().clone();
            total += distances.get(&(key, prev)).unwrap().clone();
            prev = key.clone();
        }
        results.insert(total);
    }
    results.iter().max().unwrap_or(&0).clone()
}

#[test]
fn example_a() {
    let input = "Alice would gain 54 happiness units by sitting next to Bob.\n\
                 Alice would lose 79 happiness units by sitting next to Carol.\n\
                 Alice would lose 2 happiness units by sitting next to David.\n\
                 Bob would gain 83 happiness units by sitting next to Alice.\n\
                 Bob would lose 7 happiness units by sitting next to Carol.\n\
                 Bob would lose 63 happiness units by sitting next to David.\n\
                 Carol would lose 62 happiness units by sitting next to Alice.\n\
                 Carol would gain 60 happiness units by sitting next to Bob.\n\
                 Carol would gain 55 happiness units by sitting next to David.\n\
                 David would gain 46 happiness units by sitting next to Alice.\n\
                 David would lose 7 happiness units by sitting next to Bob.\n\
                 David would gain 41 happiness units by sitting next to Carol.";
    assert_eq!(process_a(input.to_owned()), 330);
}
