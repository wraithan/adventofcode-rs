fn main() {
    let mut las = LookAndSay::new("1113122113".to_owned());
    println!("Part A: {} chars", las.nth(39).unwrap().len());
    println!("Part B: {} chars", las.nth(9).unwrap().len());
}

struct LookAndSay {
    value: String
}

impl LookAndSay {
    fn new(input: String) -> LookAndSay {
        LookAndSay{
            value: input
        }
    }
}

impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut next_value = String::new();
        let mut last = None;
        let mut count = 0;
        for c in self.value.chars() {
            match last {
                Some(last_char) => {
                    if last_char == c {
                        count += 1;
                    } else {
                        next_value = next_value + &format!("{}{}", count, last_char);
                        last = Some(c);
                        count = 1;
                    }
                }
                None => {
                    last = Some(c);
                    count += 1;
                }
            }
        }
        next_value = next_value + &format!("{}{}", count, last.unwrap());
        self.value = next_value.clone();
        Some(next_value)
    }
}

#[cfg(test)]
mod tests {
    use super::LookAndSay;

    fn test(iter: &mut LookAndSay, expected: &'static str) {
        assert_eq!(iter.next(), Some(expected.to_owned()));
    }

    #[test]
    fn example_a() {
        let mut las = LookAndSay::new("1".to_owned());
        test(&mut las, "11");
        test(&mut las, "21");
        test(&mut las, "1211");
        test(&mut las, "111221");
        test(&mut las, "312211");
    }
}
