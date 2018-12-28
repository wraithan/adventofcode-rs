#![deny(clippy::all)]

use std::collections::HashMap;
use std::cmp::{self, Ordering};

pub fn solve_puzzle_part_1(input: &str) -> Result<u32, String> {
    let mut list: Vec<Order> = input
        .lines()
        .filter(|label| !label.is_empty())
        .map(parse_line)
        .collect();

    let (top, bottom, left, right) = list.iter().fold((std::u32::MAX, 0, std::u32::MAX, 0), |mut acc, cur| {
        acc.0 = cmp::min(acc.0, cur.y);
        acc.1 = cmp::max(acc.1, cur.y + cur.height);
        acc.2 = cmp::min(acc.2, cur.x);
        acc.3 = cmp::max(acc.3, cur.x + cur.width);
        acc
    });

    list.sort_unstable();

    let mut double_claim = 0;
    for y in top..(bottom-top) {
        for x in left..(right-left) {
            let mut claims = 0;
            for order in &list {
                if y >= order.y && y < (order.y + order.height) && 
                    x >= order.x && x < (order.x + order.width) {
                        claims += 1;
                        if claims > 1 {
                            double_claim += 1;
                            break;
                        }
                    }
            }
        }
    }

    let height = bottom - top;
    let width = right - left;
    println!("top: {}, bottom: {}, left: {}, right: {}, height: {}, width: {}, area: {}", top, bottom, left, right, height, width, height * width);
    
    Ok(double_claim)
}

pub fn solve_puzzle_part_2(input: &str) -> Result<u32, String> {
    let list = input
        .lines()
        .filter(|label| !label.is_empty())
        .map(parse_line);
    let mut fabric = HashMap::new();
    for order in list.clone() {
        for x in order.x..(order.x + order.width) {
            for y in order.y..(order.y + order.height) {
                let coords = (x, y);
                let count = fabric.entry(coords).or_insert(0);
                *count += 1;
            }
        }
    }

    'order_loop: for order in list {
        for x in order.x..(order.x + order.width) {
            for y in order.y..(order.y + order.height) {
                let coords = (x, y);
                let value = fabric[&coords];
                if value > 1 {
                    continue 'order_loop;
                }
            }
        }
        return Ok(order.number);
    }
    Err("not found".into())
}

#[derive(Debug, Eq)]
struct Order {
    number: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Ord for Order {
    fn cmp(&self, other: &Order) -> Ordering {
        let y_cmp = other.y.cmp(&self.y);
        if y_cmp == Ordering::Equal {
            other.x.cmp(&self.x)
        } else {
            y_cmp
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Order) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.y == other.y && self.x == other.x
    }
}

fn parse_line(line: &str) -> Order {
    let mut parts = line.split(' ');
    let number = parts
        .next()
        .unwrap()
        .trim_start_matches('#')
        .parse()
        .unwrap();
    let _at = parts.next().unwrap();
    let pos = parts.next().unwrap();
    let size = parts.next().unwrap();

    let mut pos_parts = pos.split(',');
    let x = pos_parts.next().unwrap().parse().unwrap();
    let y = pos_parts
        .next()
        .unwrap()
        .trim_end_matches(':')
        .parse()
        .unwrap();

    let mut size_parts = size.split('x');
    let width = size_parts.next().unwrap().parse().unwrap();
    let height = size_parts.next().unwrap().parse().unwrap();
    Order {
        number,
        x,
        y,
        width,
        height,
    }
}

#[cfg(test)]
mod test_part_1 {
    use super::solve_puzzle_part_1;
    #[test]
    fn example_01() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        let expected = 4;
        let result = solve_puzzle_part_1(input).unwrap();
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod test_part_2 {
    use super::solve_puzzle_part_2;
    #[test]
    fn example_01() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        let expected = 3;
        let result = solve_puzzle_part_2(input).unwrap();
        assert_eq!(result, expected)
    }
}
