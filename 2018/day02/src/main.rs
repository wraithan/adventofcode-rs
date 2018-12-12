use std::collections::HashMap;

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let result = solve_puzzle_part_1(input)?;
    println!("Day 02 - Part 1: {}", result);
    let result = solve_puzzle_part_2(input)?;
    println!("Day 02 - Part 2: {}", result);
    Ok(())
}

struct HashInput {
    two: u32,
    three: u32,
}

fn solve_puzzle_part_1(input: &str) -> Result<u32, String> {
    let mut char_counts = HashMap::new();

    let hash_input = input
        .lines()
        .filter_map(|label| {
            if !label.is_empty() {
                char_counts.clear();
                label.chars().for_each(|label_char| {
                    let current_count = char_counts.entry(label_char).or_insert(0);
                    *current_count += 1;
                });
                let mut two_or_three = HashInput { two: 0, three: 0 };
                char_counts
                    .values()
                    .fold(&mut two_or_three, |mut acc, cur| {
                        match cur {
                            2 => acc.two = 1,
                            3 => acc.three = 1,
                            _ => (),
                        };
                        acc
                    });
                return Some(two_or_three);
            }
            None
        })
        .fold(HashInput { two: 0, three: 0 }, |mut acc, cur| {
            acc.two += cur.two;
            acc.three += cur.three;
            acc
        });
    Ok(hash_input.two * hash_input.three)
}

fn solve_puzzle_part_2(input: &str) -> Result<i32, String> {
    Ok(input.len() as i32)
}

#[cfg(test)]
mod test_part_1 {
    use super::solve_puzzle_part_1;
    #[test]
    fn example_01() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
        let expected = 12;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod test_part_2 {
    use super::solve_puzzle_part_2;
    #[test]
    fn example_01() {}
}
