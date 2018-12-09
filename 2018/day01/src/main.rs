#![deny(clippy::all)]

use std::collections::HashSet;

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let result = solve_puzzle_part_1(input)?;
    println!("Day 01 - Part 1: {}", result);
    let result = solve_puzzle_part_2(input)?;
    println!("Day 01 - Part 2: {}", result);
    Ok(())
}

fn solve_puzzle_part_1(input: &str) -> Result<i32, String> {
    let result = input
        .split('\n')
        .fold(Ok(0), |acc: Result<i32, String>, cur: &str| {
            let mut ret_val = acc?;
            if !cur.is_empty() {
                ret_val += cur
                    .parse::<i32>()
                    .map_err(|err| format!("Couldn't parse {}: {}", cur, err))?;
            }
            Ok(ret_val)
        })?;
    Ok(result)
}

fn solve_puzzle_part_2(input: &str) -> Result<i32, String> {
    let input = input.split('\n').cycle();

    let mut acc = 0;
    let mut frequencies_seen = HashSet::new();
    frequencies_seen.insert(acc); // initial accumulator value matters
    for cur in input {
        if !cur.is_empty() {
            acc += cur
                .parse::<i32>()
                .map_err(|err| format!("Couldn't parse {}: {}", cur, err))?;
            if frequencies_seen.contains(&acc) {
                break;
            }
            frequencies_seen.insert(acc);
        }
    }
    Ok(acc)
}

#[cfg(test)]
mod test_part_1 {
    use super::solve_puzzle_part_1;
    #[test]
    fn example_01() {
        let input = "+1\n-2\n+3\n+1\n";
        let expected = 3;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_02() {
        let input = "+1\n+1\n+1\n";
        let expected = 3;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_03() {
        let input = "+1\n+1\n-2\n";
        let expected = 0;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_04() {
        let input = "-1\n-2\n-3\n";
        let expected = -6;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod test_part_2 {
    use super::solve_puzzle_part_2;
    #[test]
    fn example_01() {
        let input = "+1\n-1\n";
        let expected = 0;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_02() {
        let input = "+3\n+3\n+4\n-2\n-4\n";
        let expected = 10;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_03() {
        let input = "-6\n+3\n+8\n+5\n-6\n";
        let expected = 5;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_04() {
        let input = "+7\n+7\n-2\n-7\n-4\n";
        let expected = 14;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }
}
