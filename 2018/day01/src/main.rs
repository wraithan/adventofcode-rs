#![deny(clippy::all)]

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let result = solve_puzzle_part_1(input)?;
    println!("Day 01 - Part 1: {}", result);
    let result = solve_puzzle_part_2(input)?;
    println!("Day 01 - Part 2: {}", result);
    Ok(())
}

fn solve_puzzle_part_1(input: &str) -> Result<i32, String> {
    let result = input.split('\n').fold(Ok(0), |acc: Result<i32, String>, cur: &str| {
        let mut ret_val = acc?;
        if !cur.is_empty() {
            ret_val += cur.parse::<i32>().map_err(|err| {
                format!("Couldn't parse {}: {}", cur, err)
            })?;
        }
        Ok(ret_val)
    })?;
    Ok(result)
}

fn solve_puzzle_part_2(input: &str) -> Result<i32, String> {
    Ok(input.len() as i32)
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