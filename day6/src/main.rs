#[macro_use]
extern crate nom;

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::Read;

type Pair = (usize, usize);

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} lights are on", process_a(input.clone()));
    println!("Part B: {} brightness", process_b(input.clone()));
}

fn process_a(input: String) -> usize {
    let mut grid = LightGridA::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let mut command = "".to_owned();
        let corner_a;
        let corner_b;
        match words.next() {
            Some("turn") => command = command + "turn",
            Some("toggle") => command = command + "toggle",
            Some(_) | None => continue
        }

        if command == "turn" {
            if let Some(dir) = words.next() {
                command = command + " " + dir;
            } else {
                continue
            }
        }

        if let Some(text_pair) = words.next() {
            if let Some(corner) = string_to_pair(text_pair) {
                corner_a = corner;
            } else {
                continue
            }
        } else {
            continue
        }

        match words.next() {
            Some("through") => {},
            Some(_) | None => continue
        }

        if let Some(text_pair) = words.next() {
            if let Some(corner) = string_to_pair(text_pair) {
                corner_b = corner;
            } else {
                continue
            }
        } else {
            continue
        }
        match &command[..] {
            "turn on" => grid.turn_on(corner_a, corner_b),
            "turn off" => grid.turn_off(corner_a, corner_b),
            "toggle" => grid.toggle(corner_a, corner_b),
            _ => continue
        }
    }
    grid.lights.len()
}

fn process_b(input: String) -> usize {
    let mut grid = LightGridB::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let mut command = "".to_owned();
        let corner_a;
        let corner_b;
        match words.next() {
            Some("turn") => command = command + "turn",
            Some("toggle") => command = command + "toggle",
            Some(_) | None => continue
        }

        if command == "turn" {
            if let Some(dir) = words.next() {
                command = command + " " + dir;
            } else {
                continue
            }
        }

        if let Some(text_pair) = words.next() {
            if let Some(corner) = string_to_pair(text_pair) {
                corner_a = corner;
            } else {
                continue
            }
        } else {
            continue
        }

        match words.next() {
            Some("through") => {},
            Some(_) | None => continue
        }

        if let Some(text_pair) = words.next() {
            if let Some(corner) = string_to_pair(text_pair) {
                corner_b = corner;
            } else {
                continue
            }
        } else {
            continue
        }
        match &command[..] {
            "turn on" => grid.turn_on(corner_a, corner_b),
            "turn off" => grid.turn_off(corner_a, corner_b),
            "toggle" => grid.toggle(corner_a, corner_b),
            _ => continue
        }
    }
    grid.brightness()
}

fn string_to_pair(input: &str) -> Option<Pair> {
    let mut sides = input.split(',');
    let a;
    let b;
    if let Some(text) = sides.next() {
        if let Ok(num) = usize::from_str_radix(text, 10) {
            a = num;
        } else {
            return None
        }
    } else {
        return None
    }

    if let Some(text) = sides.next() {
        if let Ok(num) = usize::from_str_radix(text, 10) {
            b = num;
        } else {
            return None
        }
    } else {
        return None
    }
    Some((a, b))
}

#[derive(Default)]
struct LightGridA {
    lights: HashSet<Pair>
}

impl LightGridA {
    fn new() -> LightGridA {
        Default::default()
    }

    fn turn_on(&mut self, corner_a: Pair, corner_b: Pair) {
        self.lights.extend(GridIterator::new(corner_a, corner_b));
    }

    fn turn_off(&mut self, corner_a: Pair, corner_b: Pair) {
        for light in GridIterator::new(corner_a, corner_b) {
            self.lights.remove(&light);
        }
    }

    fn toggle(&mut self, corner_a: Pair, corner_b: Pair) {
        for light in GridIterator::new(corner_a, corner_b) {
            if !self.lights.remove(&light) {
                self.lights.insert(light);
            }
        }
    }
}

#[derive(Default)]
struct LightGridB {
    lights: HashMap<Pair, usize>
}

impl LightGridB {
    fn new() -> LightGridB {
        Default::default()
    }

    fn turn_on(&mut self, corner_a: Pair, corner_b: Pair) {
        for light in GridIterator::new(corner_a, corner_b) {
            let mut target = self.lights.entry(light).or_insert(0);
            *target += 1;
        }
    }

    fn turn_off(&mut self, corner_a: Pair, corner_b: Pair) {
        for light in GridIterator::new(corner_a, corner_b) {
            let mut target = self.lights.entry(light).or_insert(0);
            if *target > 0 {
                *target -= 1;
            }
        }
    }

    fn toggle(&mut self, corner_a: Pair, corner_b: Pair) {
        for light in GridIterator::new(corner_a, corner_b) {
            let mut target = self.lights.entry(light).or_insert(0);
            *target += 2;
        }
    }

    fn brightness(&self) -> usize {
        self.lights.values().fold(0, |acc, value| acc + value)
    }
}


#[derive(Default)]
struct GridIterator {
    corner_a: Pair,
    corner_b: Pair,
    index_x: usize,
    index_y: usize,
    finished: bool
}

impl GridIterator {
    fn new(a: Pair, b: Pair) -> GridIterator {
        GridIterator{
            corner_a: a,
            corner_b: b,
            index_x: a.0,
            index_y: a.1,
            finished: false
        }
    }
}

impl Iterator for GridIterator {
    type Item = Pair;

    fn next(&mut self) -> Option<Pair>{
        if self.finished {
            return None
        }
        let next = (self.index_x, self.index_y);
        if next == self.corner_b {
            self.finished = true;
            return Some(next)
        }
        if self.index_x == self.corner_b.0 {
            self.index_x = self.corner_a.0;
            self.index_y += 1;
        } else {
            self.index_x += 1;
        }
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::{GridIterator, process_a, process_b};

    fn run_a(input: &'static str, expected: usize) {
        assert_eq!(process_a(input.to_owned()), expected);
    }

    fn run_a_all_on(input: &'static str, expected: usize) {
        let wrapped_input = format!("turn on 0,0 through 999,999\n{}", input);
        assert_eq!(process_a(wrapped_input), expected);
    }

    fn run_b(input: &'static str, expected: usize) {
        assert_eq!(process_b(input.to_owned()), expected);
    }

    fn run_b_all_1(input: &'static str, expected: usize) {
        let wrapped_input = format!("turn on 0,0 through 999,999\n{}", input);
        assert_eq!(process_b(wrapped_input), expected);
    }

    fn run_b_all_2(input: &'static str, expected: usize) {
        let wrapped_input = format!("turn on 0,0 through 999,999\nturn on 0,0 through 999,999\n{}", input);
        assert_eq!(process_b(wrapped_input), expected);
    }

    #[test]
    fn example_a_1() {
        let input = "turn on 0,0 through 999,999";
        run_a(input, 1_000_000);
        run_a_all_on(input, 1_000_000);
    }

    #[test]
    fn example_a_2() {
        let input = "toggle 0,0 through 999,0";
        run_a(input, 1_000);
        run_a_all_on(input, 999_000);
    }

    #[test]
    fn example_a_3() {
        let input = "turn off 499,499 through 500,500";
        run_a(input, 0);
        run_a_all_on(input, 999_996);
    }

    #[test]
    fn example_b_1() {
        let input = "turn on 0,0 through 0,0";
        run_b(input, 1);
        run_b_all_1(input, 1_000_001);
        run_b_all_2(input, 2_000_001);
    }

    #[test]
    fn example_b_2() {
        let input = "toggle 0,0 through 999,999";
        run_b(input, 2_000_000);
        run_b_all_1(input, 3_000_000);
        run_b_all_2(input, 4_000_000);
    }

    #[test]
    fn grid_iterator_basics() {
        let mut iter = GridIterator::new((0,0), (3,3));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), Some((3, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), Some((3, 1)));
        assert_eq!(iter.next(), Some((0, 2)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), Some((3, 2)));
        assert_eq!(iter.next(), Some((0, 3)));
        assert_eq!(iter.next(), Some((1, 3)));
        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), Some((3, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn grid_iterator_x_row() {
        let mut iter = GridIterator::new((0,0), (10,0));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), Some((3, 0)));
        assert_eq!(iter.next(), Some((4, 0)));
        assert_eq!(iter.next(), Some((5, 0)));
        assert_eq!(iter.next(), Some((6, 0)));
        assert_eq!(iter.next(), Some((7, 0)));
        assert_eq!(iter.next(), Some((8, 0)));
        assert_eq!(iter.next(), Some((9, 0)));
        assert_eq!(iter.next(), Some((10, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn grid_iterator_y_row() {
        let mut iter = GridIterator::new((0,0), (0,10));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((0, 2)));
        assert_eq!(iter.next(), Some((0, 3)));
        assert_eq!(iter.next(), Some((0, 4)));
        assert_eq!(iter.next(), Some((0, 5)));
        assert_eq!(iter.next(), Some((0, 6)));
        assert_eq!(iter.next(), Some((0, 7)));
        assert_eq!(iter.next(), Some((0, 8)));
        assert_eq!(iter.next(), Some((0, 9)));
        assert_eq!(iter.next(), Some((0, 10)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn grid_iterator_basics_no_zero() {
        let mut iter = GridIterator::new((10,10), (13,13));
        assert_eq!(iter.next(), Some((10, 10)));
        assert_eq!(iter.next(), Some((11, 10)));
        assert_eq!(iter.next(), Some((12, 10)));
        assert_eq!(iter.next(), Some((13, 10)));
        assert_eq!(iter.next(), Some((10, 11)));
        assert_eq!(iter.next(), Some((11, 11)));
        assert_eq!(iter.next(), Some((12, 11)));
        assert_eq!(iter.next(), Some((13, 11)));
        assert_eq!(iter.next(), Some((10, 12)));
        assert_eq!(iter.next(), Some((11, 12)));
        assert_eq!(iter.next(), Some((12, 12)));
        assert_eq!(iter.next(), Some((13, 12)));
        assert_eq!(iter.next(), Some((10, 13)));
        assert_eq!(iter.next(), Some((11, 13)));
        assert_eq!(iter.next(), Some((12, 13)));
        assert_eq!(iter.next(), Some((13, 13)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn grid_iterator_x_row_no_zero() {
        let mut iter = GridIterator::new((10,10), (20,10));
        assert_eq!(iter.next(), Some((10, 10)));
        assert_eq!(iter.next(), Some((11, 10)));
        assert_eq!(iter.next(), Some((12, 10)));
        assert_eq!(iter.next(), Some((13, 10)));
        assert_eq!(iter.next(), Some((14, 10)));
        assert_eq!(iter.next(), Some((15, 10)));
        assert_eq!(iter.next(), Some((16, 10)));
        assert_eq!(iter.next(), Some((17, 10)));
        assert_eq!(iter.next(), Some((18, 10)));
        assert_eq!(iter.next(), Some((19, 10)));
        assert_eq!(iter.next(), Some((20, 10)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn grid_iterator_y_row_no_zero() {
        let mut iter = GridIterator::new((10,10), (10,20));
        assert_eq!(iter.next(), Some((10, 10)));
        assert_eq!(iter.next(), Some((10, 11)));
        assert_eq!(iter.next(), Some((10, 12)));
        assert_eq!(iter.next(), Some((10, 13)));
        assert_eq!(iter.next(), Some((10, 14)));
        assert_eq!(iter.next(), Some((10, 15)));
        assert_eq!(iter.next(), Some((10, 16)));
        assert_eq!(iter.next(), Some((10, 17)));
        assert_eq!(iter.next(), Some((10, 18)));
        assert_eq!(iter.next(), Some((10, 19)));
        assert_eq!(iter.next(), Some((10, 20)));
        assert_eq!(iter.next(), None);
    }
}
