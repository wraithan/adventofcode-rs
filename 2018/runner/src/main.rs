#![deny(clippy::all)]

use day01;
use day02;
use day03;
use day04;
use day05;

macro_rules! expand_day {
    ($day_name:ident, $display_name:expr) => {
        // Load input file
        let start = std::time::Instant::now();
        let day_name = stringify!($day_name);
        let mut input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.pop();
        input_path.push(day_name);
        input_path.push("input.txt");
        let input = std::fs::read_to_string(input_path).expect("input text");
        let load_time = start.elapsed();

        // Solve puzzles
        let start = std::time::Instant::now();
        let part1_answer = $day_name::solve_puzzle_part_1(&input).expect("don't crash");
        let part1_time = start.elapsed();
        let start = std::time::Instant::now();
        let part2_answer = $day_name::solve_puzzle_part_2(&input).expect("don't crash");
        let part2_time = start.elapsed();

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

fn main() {
    expand_day!(day01, "Day 01");
    expand_day!(day02, "Day 02");
    expand_day!(day03, "Day 03");
    expand_day!(day04, "Day 04");
    expand_day!(day05, "Day 05");
}
