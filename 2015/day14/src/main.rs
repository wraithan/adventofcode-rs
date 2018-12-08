use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("Part A: {} m", race_a(input.clone(), 2503));
    println!("Part B: {} points", race_b(input.clone(), 2503));
}

fn race_a(input: String, duration: usize) -> usize {
    input
        .lines()
        .filter_map(Reindeer::from_string)
        .map(|deer| deer.run(duration))
        .max()
        .unwrap()
}

fn race_b(input: String, duration: usize) -> usize {
    let mut results: HashMap<String, usize> = HashMap::new();
    let mut team: Vec<DeerRaceIterator> = input
        .lines()
        .filter_map(Reindeer::from_string)
        .map(|deer| deer.start_race())
        .collect();
    for _ in 0..duration {
        let mut winning = Vec::new();
        let mut highest = 0;
        for deer in team.iter_mut() {
            let distance = deer.next().unwrap();
            if distance > highest {
                winning.clear();
                winning.push(deer.deer.name.clone());
                highest = distance;
            } else if distance == highest {
                winning.push(deer.deer.name.clone());
            }
        }
        for deer in winning {
            let value = results.entry(deer).or_insert(0);
            *value += 1
        }
    }
    results.values().max().unwrap().clone()
}

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: usize,
    duration: usize,
    rest: usize
}

impl Reindeer {
    fn from_string(input: &str) -> Option<Reindeer> {
        let mut words = input.split_whitespace();
        let name = get!(words.next());
        let speed = get!(usize::from_str_radix(get!(words.nth(2)), 10).ok());
        let duration = get!(usize::from_str_radix(get!(words.nth(2)), 10).ok());
        let rest = get!(usize::from_str_radix(get!(words.nth(6)), 10).ok());
        Some(Reindeer {
            name: name.to_owned(),
            speed: speed,
            duration: duration,
            rest: rest
        })
    }

    fn run(&self, seconds: usize) -> usize {
        let mut distance = 0;
        let mut remaining = seconds;
        while remaining != 0 {
            if remaining >= self.duration {
                distance += self.duration * self.speed;
                remaining -= self.duration;
            } else {
                distance += self.speed * remaining;
                break;
            }

            if remaining > self.rest {
                remaining -= self.rest;
            } else {
                break;
            }
        }
        distance
    }

    fn start_race(self) -> DeerRaceIterator {
        DeerRaceIterator{
            state: DeerRaceState::Running(self.duration),
            deer: self,
            distance: 0,
        }
    }
}

enum DeerRaceState {
    Running(usize),
    Resting(usize)
}

struct DeerRaceIterator {
    deer: Reindeer,
    distance: usize,
    state: DeerRaceState
}

impl Iterator for DeerRaceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.state = match self.state {
            DeerRaceState::Running(left) => {
                self.distance += self.deer.speed;
                if left == 1 {
                    DeerRaceState::Resting(self.deer.rest)
                } else {
                    DeerRaceState::Running(left - 1)
                }
            }
            DeerRaceState::Resting(left) => {
                if left == 1 {
                    DeerRaceState::Running(self.deer.duration)
                } else {
                    DeerRaceState::Resting(left - 1)
                }
            }
        };
        Some(self.distance)
    }
}

#[test]
fn exercise_from_string() {
    let input_a = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
    assert_eq!(Reindeer::from_string(input_a),
               Some(Reindeer{
                   name: "Comet".to_owned(),
                   speed: 14,
                   duration: 10,
                   rest: 127
               }));
    let input_b = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(Reindeer::from_string(input_b),
               Some(Reindeer{
                   name: "Dancer".to_owned(),
                   speed: 16,
                   duration: 11,
                   rest: 162
               }));
}

#[test]
fn exercise_run() {
    let input_a = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
    let comet = Reindeer::from_string(input_a).unwrap();
    assert_eq!(comet.run(10), 140);
    assert_eq!(comet.run(11), 140);
    assert_eq!(comet.run(1000), 1120);
    let input_b = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    let dancer = Reindeer::from_string(input_b).unwrap();
    assert_eq!(dancer.run(10), 160);
    assert_eq!(dancer.run(11), 176);
    assert_eq!(dancer.run(1000), 1056);
}

#[test]
fn exercise_iterator() {
    let input_a = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
    let comet = Reindeer::from_string(input_a).unwrap();
    let mut comet_race = comet.start_race();
    assert_eq!(comet_race.next(), Some(14));
    let input_b = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    let dancer = Reindeer::from_string(input_b).unwrap();
    let mut dancer_race = dancer.start_race();
    assert_eq!(dancer_race.next(), Some(16));
}

#[test]
fn exercise_race_b() {
    let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
                 Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(race_b(input.to_owned(), 1000), 689);
}
