use std::collections::HashMap;

fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    let result = solve_puzzle_part_1(input)?;
    println!("Day 03 - Part 1: {}", result);
    let result = solve_puzzle_part_2(input)?;
    println!("Day 03 - Part 2: {}", result);
    Ok(())
}

fn solve_puzzle_part_1(input: &str) -> Result<u32, String> {
    let list = input
        .lines()
        .filter(|label| !label.is_empty())
        .map(parse_line);
    let mut fabric = HashMap::new();
    for order in list {
        for x in order.x..(order.x + order.width) {
            for y in order.y..(order.y + order.height) {
                let coords = format!("{},{}", x, y);
                let count = fabric.entry(coords).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut count = 0;
    for sqin in fabric.values() {
        if sqin > &1 {
            count += 1;
        }
    }
    Ok(count)
}

fn solve_puzzle_part_2(input: &str) -> Result<u32, String> {
    let list = input
        .lines()
        .filter(|label| !label.is_empty())
        .map(parse_line);
    let mut fabric = HashMap::new();
    for order in list.clone() {
        for x in order.x..(order.x + order.width) {
            for y in order.y..(order.y + order.height) {
                let coords = format!("{},{}", x, y);
                let count = fabric.entry(coords).or_insert(0);
                *count += 1;
            }
        }
    }

    'order_loop: for order in list {
        for x in order.x..(order.x + order.width) {
            for y in order.y..(order.y + order.height) {
                let coords = format!("{},{}", x, y);
                let value = fabric.get(&coords).unwrap();
                if value > &1 {
                    continue 'order_loop;
                }
            }
        }
        return Ok(order.number);
    }
    Err("not found".into())
}

#[derive(Debug)]
struct Order {
    number: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
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
