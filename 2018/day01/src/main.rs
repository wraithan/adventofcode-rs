#![deny(clippy::all)]

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let result = solve_puzzle(input)?;
    println!("Result: {}", result);
    Ok(())
}

fn solve_puzzle(input: &str) -> Result<i32, String> {
    let result = input.split('\n').fold(0, |mut acc: i32, cur| {
        if !cur.is_empty() {
            acc += cur.parse::<i32>().unwrap();
        }
        acc
    });
    Ok(result)
}

#[test]
fn first_example() {
    let input = "+1\n-2\n+3\n+1\n";
    let expected = 3;
    let result = solve_puzzle(input).unwrap();
    assert_eq!(result, expected)
}

#[test]
fn second_example() {
    let input = "+1\n+1\n+1\n";
    let expected = 3;
    let result = solve_puzzle(input).unwrap();
    assert_eq!(result, expected)
}

#[test]
fn third_example() {
    let input = "+1\n+1\n-2\n";
    let expected = 0;
    let result = solve_puzzle(input).unwrap();
    assert_eq!(result, expected)
}

#[test]
fn fourth_example() {
    let input = "-1\n-2\n-3\n";
    let expected = -6;
    let result = solve_puzzle(input).unwrap();
    assert_eq!(result, expected)
}
