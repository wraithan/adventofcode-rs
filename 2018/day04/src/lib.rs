#![deny(clippy::all)]

use chrono::{Duration, NaiveDate, NaiveTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_puzzle_part_1(input: &str) -> Result<u32, String> {
    let mut logs: Vec<LogEntry> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect();
    logs.sort_unstable();
    let shifts = sorted_logs_to_shifts(&logs);
    let most_sleep = shifts.values().max_by_key(|v| v.sleep_duration).unwrap();
    let most_common = most_common_minute_slept(most_sleep).0;

    Ok(most_sleep.guard_id * most_common)
}

fn sorted_logs_to_shifts(logs: &[LogEntry]) -> HashMap<u32, Shift> {
    let mut shifts = HashMap::new();

    let mut current_guard = 0;
    let mut can_wake = false;
    for log in logs {
        match log.entry_type {
            EntryType::StartShift(guard_id) => {
                shifts.entry(guard_id).or_insert_with(|| Shift {
                    guard_id,
                    sleep_duration: 0,
                    times_slept: vec![],
                });
                current_guard = guard_id;
                can_wake = false;
            }
            EntryType::FellAsleep => {
                let shift = shifts.get_mut(&current_guard).unwrap();
                shift.times_slept.push((log.minute, 0));
                can_wake = true;
            }
            EntryType::WokeUp => {
                assert!(can_wake, "prior must be a sleep");
                can_wake = false;
                let mut shift = shifts.get_mut(&current_guard).unwrap();
                let time_slept = shift.times_slept.last_mut().unwrap();
                time_slept.1 = log.minute;
                shift.sleep_duration += time_slept.1 - time_slept.0;
            }
        }
    }
    shifts
}

fn most_common_minute_slept(shift: &Shift) -> (u32, u32) {
    if shift.times_slept.is_empty() {
        return (0, 0);
    }
    let sleep_times = shift
        .times_slept
        .iter()
        .fold(HashMap::new(), |mut acc, (start, end)| {
            for i in *start..*end {
                let count = acc.entry(i).or_insert(0u32);
                *count += 1;
            }
            acc
        });

    let most_common = sleep_times.iter().max_by_key(|(_k, v)| *v).unwrap();
    (*most_common.0, *most_common.1)
}

pub fn solve_puzzle_part_2(input: &str) -> Result<u32, String> {
    let mut logs: Vec<LogEntry> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect();
    logs.sort_unstable();
    let shifts = sorted_logs_to_shifts(&logs);
    let most_common_per_guard = shifts
        .values()
        .map(|v| (v.guard_id, most_common_minute_slept(v)));
    let most_sleeps_during_minute = most_common_per_guard
        .max_by_key(|(_gid, (_minute, count))| *count)
        .unwrap();
    Ok(most_sleeps_during_minute.0 * (most_sleeps_during_minute.1).0)
}

#[derive(Debug)]
struct Shift {
    guard_id: u32,
    sleep_duration: u32,
    times_slept: Vec<(u32, u32)>,
}

#[derive(Debug, Eq)]
struct LogEntry {
    entry_type: EntryType,
    date: NaiveDate,
    minute: u32,
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        let date_cmp = self.date.cmp(&other.date);
        if date_cmp == Ordering::Equal {
            let minute_cmp = self.minute.cmp(&other.minute);
            if minute_cmp == Ordering::Equal {
                if let EntryType::StartShift(_) = self.entry_type {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                minute_cmp
            }
        } else {
            date_cmp
        }
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.date == other.date && self.minute == other.minute
    }
}

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    StartShift(u32),
    FellAsleep,
    WokeUp,
}

fn parse_line(input: &str) -> LogEntry {
    lazy_static! {
        static ref DATE_TIME_RE: Regex =
            Regex::new(r"^\[([0-9\-]*) ([0-9:]*)\] (.*)$").expect("date time regex");
        static ref GUARD_NUM_RE: Regex = Regex::new(r"Guard #(\d+)").expect("guard num regex");
    }
    let cap = DATE_TIME_RE.captures(input).unwrap();
    let date_str = cap.get(1).unwrap().as_str();
    let time_str = cap.get(2).unwrap().as_str();
    let entry_type_str = cap.get(3).unwrap().as_str();
    let entry_type = match entry_type_str {
        "wakes up" => EntryType::WokeUp,
        "falls asleep" => EntryType::FellAsleep,
        _ => {
            let cap = GUARD_NUM_RE.captures(entry_type_str).unwrap();
            let guard_num_str = cap.get(1).unwrap().as_str();
            EntryType::StartShift(guard_num_str.parse().unwrap())
        }
    };

    let mut date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let time = NaiveTime::parse_from_str(time_str, "%H:%M").unwrap();
    let minute = if time.hour() == 23 {
        date += Duration::days(1);
        0
    } else {
        time.minute()
    };
    LogEntry {
        entry_type,
        date,
        minute,
    }
}

#[cfg(test)]
mod test_part_1 {
    use super::solve_puzzle_part_1;
    #[test]
    fn example_01() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift™
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let expected = 240;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_01_reversed() {
        let mut input: Vec<&str> = "[1518-11-01 00:00] Guard #10 begins shift™
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            .lines()
            .collect();
        input.reverse();
        let input: String = input.join("\n");
        let expected = 240;
        let result = solve_puzzle_part_1(&input).unwrap();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod test_part_2 {
    use super::solve_puzzle_part_2;
    #[test]
    fn example_01() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift™
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let expected = 4455;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_01_reversed() {
        let mut input: Vec<&str> = "[1518-11-01 00:00] Guard #10 begins shift™
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            .lines()
            .collect();
        input.reverse();
        let input: String = input.join("\n");
        let expected = 4455;
        let result = solve_puzzle_part_2(&input).unwrap();
        assert_eq!(result, expected)
    }
}
