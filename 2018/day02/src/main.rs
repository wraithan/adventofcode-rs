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

    let label_list = input.lines().filter(|label| !label.is_empty());

    let mut hash_inputs = HashInput { two: 0, three: 0 };
    for label in label_list {
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
        hash_inputs.two += two_or_three.two;
        hash_inputs.three += two_or_three.three;
    }
    Ok(hash_inputs.two * hash_inputs.three)
}

fn solve_puzzle_part_2(input: &str) -> Result<String, String> {
    let label_list = input.lines().filter(|label| !label.is_empty());
    for (index, label) in label_list.clone().enumerate() {
        for other_label in label_list.clone().skip(index + 1) {
            let mut bad_chars = label.chars().zip(other_label.chars()).filter_map(|chars| {
                if chars.0 == chars.1 {
                    None
                } else {
                    Some(chars.0)
                }
            });
            if bad_chars.clone().count() == 1 {
                let char_to_remove = bad_chars.next().unwrap();
                return Ok(label.chars().filter(|c| c != &char_to_remove).collect());
            }
        }
    }
    Ok(input.into())
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
    fn example_01() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        let expected = "fgij";
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }
}
