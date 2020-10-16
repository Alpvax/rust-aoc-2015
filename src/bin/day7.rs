use rust_2015::read_lines;
use fancy_regex::{Captures, Match, Regex};
use std::collections::HashMap;

type Map = HashMap<String, Operation>;

fn main() {
    let pattern = Regex::new(r"(?:(?:(?P<sig1>\d+|[[:alpha:]]+) )?(?P<op>AND|OR|LSHIFT|RSHIFT|NOT) )?(?P<sig2>\d+|[[:alpha:]]+) -> (?P<wire>[[:alpha:]]+)").unwrap();
    let mut map: Map = HashMap::new();
    for line in read_lines("puzzle-input/7.0.txt", |s| s) {
        let captures = pattern.captures(&line).expect("Regex parsing error").expect("No match found");
        let wire = captures.name("wire").unwrap().as_str();
        let op = Operation::parse(&captures, &map);
        //println!("{}: {} = {:?}", line, wire, op);
        map.insert(wire.to_string(), op);
    }
    let mut results = HashMap::new();
    println!("{:?}", evaluate_wire("a", &map, &mut results));
    println!("{:?}", results);
}

fn evaluate_wire(s: &str, map: &Map, values: &mut HashMap<String, u16>) -> u16 {
    match values.get(s) {
        Some(val) => *val,
        None => {
            let op = map.get(s).expect("Wire not present in map");
            let val = match op {
                Operation::Value(i) => *i,
                Operation::EQ(w) => evaluate_wire(w, map, values),
                Operation::NOT(w) => !evaluate_wire(w, map, values),
                Operation::AND(l, r) => {
                    let (a, b) = eval_pair(l, r, map, values);
                    a & b
                },
                Operation::OR(l, r) => {
                    let (a, b) = eval_pair(l, r, map, values);
                    a | b
                },
                Operation::LSHIFT(a, b) => match a {
                    Operand::Value(i) => i << b,
                    Operand::Wire(w) => evaluate_wire(w, map, values) << b,
                },
                Operation::RSHIFT(a, b) => match a {
                    Operand::Value(i) => i >> b,
                    Operand::Wire(w) => evaluate_wire(w, map, values) >> b,
                },
            };
            values.insert(s.to_string(), val);
            val
        },
    }
}

fn eval_pair(l: &Operand, r: &Operand, map: &Map, values: &mut HashMap<String, u16>) -> (u16, u16) {
    (
        match l {
            Operand::Value(i) => *i,
            Operand::Wire(w) => evaluate_wire(w, map, values),
        },
        match r {
            Operand::Value(i) => *i,
            Operand::Wire(w) => evaluate_wire(w, map, values),
        },
    )
}

fn parse_signal(m: Match, map: &Map) -> Operand {
    let name = m.as_str();
    match name.parse::<u16>() {
        Ok(i) => Operand::Value(i),
        Err(_e) => if let Some(op) = map.get(name) {
            match op {
                Operation::Value(i) => Operand::Value(*i),
                _ => Operand::Wire(name.to_string()),
            }
        } else { Operand::Wire(name.to_string()) },
    }
}

#[derive(Debug)]
enum Operand {
    Value(u16),
    Wire(String),
}
impl Operand {
    fn as_int(self) -> u16 {
        match self {
            Self::Value(i) => i,
            Self::Wire(s) => panic!("Attempted to get wire {} as signal", s),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Value(u16),
    EQ(String),
    NOT(String),
    AND(Operand, Operand),
    OR(Operand, Operand),
    LSHIFT(Operand, u16),
    RSHIFT(Operand, u16),
}

impl Operation {
    fn parse(captures: &Captures, map: &Map) -> Self {
        let r = parse_signal(captures.name("sig2").unwrap(), map);
        if let Some(op) = captures.name("op") {
            if op.as_str() == "NOT" {
                match r {
                    Operand::Value(i) => Self::Value(!i),
                    Operand::Wire(w) => Self::NOT(w),
                }
            } else {
                let l = captures.name("sig1").map(|m| parse_signal(m, map)).unwrap();
                if let Operand::Value(a) = l {
                    if let Operand::Value(b) = r {
                        return match op.as_str() {
                            "AND" => Self::Value(a & b),
                            "OR" => Self::Value(a | b),
                            "LSHIFT" => Self::Value(a << b),
                            "RSHIFT" => Self::Value(a >> b),
                            _ => panic!("Invalid operation! Should not be possible"),
                        }
                    }
                }
                match op.as_str() {
                    "AND" => Self::AND(l, r),
                    "OR" => Self::OR(l, r),
                    "LSHIFT" => Self::LSHIFT(l, r.as_int()),
                    "RSHIFT" => Self::RSHIFT(l, r.as_int()),
                    _ => panic!("Invalid operation! Should not be possible"),
                }
            }
        } else {
            match r {
                Operand::Value(i) => Self::Value(i),
                Operand::Wire(w) => Self::EQ(w),
            }
        }
    }
}