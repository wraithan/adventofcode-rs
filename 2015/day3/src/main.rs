use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} houses", process_a(input.clone()));
    println!("Part B: {} houses", process_b(input.clone()));
}

#[derive(Default)]
struct GameMap {
    pub points: HashSet<(isize, isize)>,
    position: (isize, isize)
}

impl GameMap {
    fn new() -> GameMap {
        let mut map = GameMap{position: (0, 0), ..Default::default()};
        map.points.insert(map.position);
        map
    }

    fn count(&self) -> usize {
        self.points.len()
    }

    fn move_santa(&mut self, direction: char) {
        match direction {
            '^' => {
                self.position = (self.position.0 + 1, self.position.1);
            },
            '>' => {
                self.position = (self.position.0, self.position.1 + 1);
            },
            'v' => {
                self.position = (self.position.0 - 1, self.position.1);
            },
            '<' => {
                self.position = (self.position.0, self.position.1 - 1);
            }
            _ => {}
        }
        self.points.insert(self.position);
    }
}

fn process_a(input: String) -> usize {
    let mut map = GameMap::new();
    for direction in input.chars() {
        map.move_santa(direction);
    }
    map.count()
}

fn process_b(input: String) -> usize {
    let mut santa = GameMap::new();
    let mut robo_santa = GameMap::new();
    for (i, direction) in input.chars().enumerate() {
        match i % 2 {
            0 => santa.move_santa(direction),
            1 => robo_santa.move_santa(direction),
            _ => {}
        }
    }
    santa.points.union(&robo_santa.points).count()
}

#[test]
fn example_a_1() {
    assert_eq!(process_a(">".to_owned()), 2);
}

#[test]
fn example_a_2() {
    assert_eq!(process_a("^>v<".to_owned()), 4);
}

#[test]
fn example_a_3() {
    assert_eq!(process_a("^v^v^v^v^v".to_owned()), 2);
}

#[test]
fn example_b_1() {
    assert_eq!(process_b("^v".to_owned()), 3);
}

#[test]
fn example_b_2() {
    assert_eq!(process_b("^>v<".to_owned()), 3);
}

#[test]
fn example_b_3() {
    assert_eq!(process_b("^v^v^v^v^v".to_owned()), 11);
}
