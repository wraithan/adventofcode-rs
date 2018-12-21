// use std::collections::HashMap;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let result = solve_puzzle_part_1(input)?;
    println!("Day 04 - Part 1: {}", result);
    let result = solve_puzzle_part_2(input)?;
    println!("Day 04 - Part 2: {}", result);
    Ok(())
}

fn solve_puzzle_part_1(input: &str) -> Result<u32, String> {
    let input = input.trim();
    let mut result = vec![];
    for unit in input.chars() {
        // println!("Testing {} ({:?})", unit, result);
        if result.is_empty() {
            // println!("init");
            result.push(unit);
        } else {
            let last = result.last().expect("last char");
            if last == &unit {
                result.push(unit);
            } else if last.to_lowercase().next().expect("first char")
                == unit.to_lowercase().next().expect("first char")
            {
                result.pop();
            } else {
                result.push(unit);
            }
        }
    }
    Ok(result.len() as u32)
}

fn solve_puzzle_part_2(input: &str) -> Result<u32, String> {
    Ok(ASCII_LOWER
        .iter()
        .map(|a| {
            let reduced_input: String = input
                .chars()
                .filter(|c| c != a && &c.to_lowercase().next().expect("char") != a)
                .collect();
            solve_puzzle_part_1(&reduced_input).expect("first solver")
        })
        .min()
        .expect("min value"))
}

#[cfg(test)]
mod test_part_1 {
    use super::solve_puzzle_part_1;
    #[test]
    fn example_01() {
        let input = "aA";
        let expected = 0;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_02() {
        let input = "abBA";
        let expected = 0;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_03() {
        let input = "abBA";
        let expected = 0;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_04() {
        let input = "abAB";
        let expected = 4;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_05() {
        let input = "aabAAB";
        let expected = 6;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn example_06() {
        let input = "dabAcCaCBAcCcaDA";
        let expected = 10;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod test_part_2 {
    use super::solve_puzzle_part_2;
    #[test]
    fn example_01() {
        let input = "dabAcCaCBAcCcaDA";
        let expected = 4;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }
}
