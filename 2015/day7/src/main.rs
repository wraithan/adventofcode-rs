use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Memory = HashMap<String, Instruction>;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let key = "a".to_owned();
    let mut machine_a = process_a(input.clone());
    machine_a.warm();
    println!("Part A: {} is the value of wire 'a'", machine_a.get(&key));
    let mut machine_b = process_a(input + "\n3176 -> b");
    machine_b.warm();
    println!("Part B: {} is the value of wire 'a'", machine_b.get(&key));
}

fn process_a(input: String) -> Machine {
    let mut machine = Machine::new();
    machine.setup(input.lines().filter_map(parse_line).collect());
    machine
}

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

fn parse_line(line: &str) -> Option<(Instruction, String)> {
    let mut words = line.split_whitespace();
    let first = get!(words.next());
    if first == "NOT" {
        let lhs = Value::from_str(get!(words.next()));
        if get!(words.next()) != "->" {
            return None;
        }
        let rhs = get!(words.next());
        return Some((Instruction::Not(lhs), rhs.to_owned()));
    }
    let lhs = Value::from_str(first);
    let command = get!(words.next());
    let rhs_raw = get!(words.next());
    if command == "->" {
        return Some((Instruction::Assign(lhs), rhs_raw.to_owned()));
    }
    let rhs = Value::from_str(rhs_raw);
    let arrow = get!(words.next());
    if arrow != "->" {
        return None;
    }
    let target = get!(words.next());
    match command {
        "AND" => Some((Instruction::And(lhs, rhs), target.to_owned())),
        "OR" => Some((Instruction::Or(lhs, rhs), target.to_owned())),
        "LSHIFT" => Some((Instruction::LShift(lhs, rhs), target.to_owned())),
        "RSHIFT" => Some((Instruction::RShift(lhs, rhs), target.to_owned())),
        _ => {
            println!("{}", line);
            None
        }
    }
}

#[derive(Default)]
struct Machine {
    mem: Memory
}

impl Machine {
    fn new() -> Machine {
        Default::default()
    }

    fn setup(&mut self, instructions: Vec<(Instruction, String)>) {
        for (i, target) in instructions.into_iter() {
            self.mem.insert(target, i);
        }
    }

    fn get(&self, wire: &String) -> u16 {
        self._get_limit(wire, None).unwrap()
    }

    fn _get_limit(&self, wire: &String, limit: Option<usize>) -> Option<u16> {
        let next = match limit {
            Some(num) => {
                if num == 0 {
                    return None
                }
                Some(num - 1)
            },
            None => None
        };
        let result = match self.mem.get(wire).unwrap_or(&Instruction::Assign(Value::Number(0))) {
            &Instruction::Assign(ref val) => {
                get!(self.resolve_value(val, next))
            }
            &Instruction::Not(ref val) => {
                !get!(self.resolve_value(val, next))
            }
            &Instruction::And(ref lhs, ref rhs) => {
                let real_lhs = get!(self.resolve_value(lhs, next));
                let real_rhs = get!(self.resolve_value(rhs, next));
                real_lhs & real_rhs
            }
            &Instruction::Or(ref lhs, ref rhs) => {
                let real_lhs = get!(self.resolve_value(lhs, next));
                let real_rhs = get!(self.resolve_value(rhs, next));
                real_lhs | real_rhs
            }
            &Instruction::LShift(ref lhs, ref rhs) => {
                let real_lhs = get!(self.resolve_value(lhs, next));
                let real_rhs = get!(self.resolve_value(rhs, next));
                real_lhs << real_rhs
            }
            &Instruction::RShift(ref lhs, ref rhs) => {
                let real_lhs = get!(self.resolve_value(lhs, next));
                let real_rhs = get!(self.resolve_value(rhs, next));
                real_lhs >> real_rhs
            }
        };
        Some(result)
    }

    fn warm(&mut self) {
        for _ in 0..17 {
            let keys: Vec<String> = self.mem.keys().map(|x| x.clone()).collect();
            for key in keys {
                match self._get_limit(&key, Some(5)) {
                    Some(value) => {
                        self.mem.insert(key, Instruction::Assign(Value::Number(value)));
                    }
                    None => {}
                }
            }
        }
    }

    fn resolve_value(&self, val: &Value, limit: Option<usize>) -> Option<u16> {
        match val {
            &Value::Number(number) => Some(number),
            &Value::Pointer(ref name) => self._get_limit(name, limit)
        }
    }
}

#[derive(Debug)]
enum Value {
    Pointer(String),
    Number(u16)
}

impl Value {
    fn from_str(input: &str) -> Value {
        match u16::from_str_radix(input, 10) {
            Ok(num) => Value::Number(num),
            Err(_) => Value::Pointer(input.to_owned())
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Not(Value),
    Assign(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Value),
    RShift(Value, Value)
}

#[cfg(test)]
mod tests {
    use super::process_a;
    use std::collections::HashMap;
    type ExpectedState = HashMap<String, u16>;

    fn run (input: &'static str, expected: ExpectedState) {
        let machine = process_a(input.to_owned());
        for (key, val) in expected.into_iter() {
            assert_eq!(machine.get(&key), val);
        }
    }

    #[test]
    fn example_a_1() {
        let mut expected = ExpectedState::new();
        expected.insert("x".to_owned(), 123);
        run("123 -> x", expected)
    }

    #[test]
    fn example_a_2() {
        let mut expected = ExpectedState::new();
        expected.insert("x".to_owned(), 2);
        expected.insert("y".to_owned(), 3);
        expected.insert("z".to_owned(), 2 & 3);
        run("2 -> x\n3 -> y\nx AND y -> z", expected)
    }

    #[test]
    fn example_a_3() {
        let mut expected = ExpectedState::new();
        expected.insert("p".to_owned(), 123);
        expected.insert("q".to_owned(), 492);
        run("123 -> p\np LSHIFT 2 -> q", expected)
    }

    #[test]
    fn example_a_4() {
        let mut expected = ExpectedState::new();
        expected.insert("e".to_owned(), 123);
        expected.insert("f".to_owned(), u16::max_value() - 123);
        run("123 -> e\nNOT e -> f", expected)
    }

    #[test]
    fn example_a_5() {
        let input = "123 -> x\n\
                     456 -> y\n\
                     x AND y -> d\n\
                     x OR y -> e\n\
                     x LSHIFT 2 -> f\n\
                     y RSHIFT 2 -> g\n\
                     NOT x -> h\n\
                     NOT y -> i";
        let mut expected = ExpectedState::new();
        expected.insert("d".to_owned(), 72);
        expected.insert("e".to_owned(), 507);
        expected.insert("f".to_owned(), 492);
        expected.insert("g".to_owned(), 114);
        expected.insert("h".to_owned(), 65412);
        expected.insert("i".to_owned(), 65079);
        expected.insert("x".to_owned(), 123);
        expected.insert("y".to_owned(), 456);
        run(input, expected)
    }
}
