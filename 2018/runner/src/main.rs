#![deny(clippy::all)]

use day01;
use day02;
use day03;
use day04;
use day05;
use std::time::{Duration, Instant};

macro_rules! expand_day {
    ($day_name:ident, $display_name:expr) => {
        // Load input file
        let (input, load_time) = time_func(|| {
            let day_name = stringify!($day_name);
            let mut input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            input_path.pop();
            input_path.push(day_name);
            input_path.push("input.txt");
            std::fs::read_to_string(input_path).expect("input text")
        });

        // Solve puzzles
        let (part1_answer, part1_time) =
            time_func(|| $day_name::solve_puzzle_part_1(&input).expect("don't crash"));

        let (part2_answer, part2_time) =
            time_func(|| $day_name::solve_puzzle_part_2(&input).expect("don't crash"));

        // Print result
        println!(
            "{} ({}.{:06})\n  Part 1 ({}.{:06}): {}\n  Part 2 ({}.{:06}): {}",
            $display_name,
            load_time.as_secs(),
            load_time.subsec_micros(),
            part1_time.as_secs(),
            part1_time.subsec_micros(),
            part1_answer,
            part2_time.as_secs(),
            part2_time.subsec_micros(),
            part2_answer,
        );
    };
}

fn time_func<F, T>(func: F) -> (T, Duration)
where
    F: FnOnce() -> T,
    T: Sized,
{
    let start = Instant::now();
    (func(), start.elapsed())
}

fn main() {
    expand_day!(day01, "Day 01");
    expand_day!(day02, "Day 02");
    expand_day!(day03, "Day 03");
    expand_day!(day03_query, "Day 03 Query");
    expand_day!(day04, "Day 04");
    expand_day!(day05, "Day 05");
}
