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
        println!("{}: {} = {:?}", line, wire, op);
        map.insert(wire.to_string(), op);
    }
    println!("{:?}", eval_wire("a", &mut map));
}

fn parse_signal(s: Option<Match>, map: &Map) -> Operand {
    let name = s.unwrap().as_str();
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

fn eval_wire(s: &str, map: &mut Map) -> u16 {
    let op = map.get(s).expect("Wire not present in map");
    match op {
        Operation::Value(i) => *i,
        Operation::EQ(w) => eval_wire(w, map),

        Operation::NOT(o) => !o.get(map),
        Operation::AND(a, b) => a.get(map) & b.get(map),
        Operation::OR(a, b) => a.get(map) | b.get(map),
        Operation::LSHIFT(a, b) => a.get(map) << b.get(map),
        Operation::RSHIFT(a, b) => a.get(map) >> b.get(map),
        _ => {
            let val = op.eval(map);
            map.insert(s.to_string(), Operation::Value(val));
            val
        }
    }
}

#[derive(Debug)]
enum Operand {
    Value(u16),
    Wire(String),
}

impl Operand {
    fn get(&self, map: &mut Map) -> u16 {
        match self {
            Self::Value(i) => *i,
            Self::Wire(s) => {
                println!("Evaluating wire: {}", s);
                eval_wire(s, map)
            },
        }
    }
}

#[derive(Debug)]
enum Operation {
    NOT(Operand),
    AND(Operand, Operand),
    OR(Operand, Operand),
    LSHIFT(Operand, Operand),
    RSHIFT(Operand, Operand),
    EQ(String),
    Value(u16),
}

impl Operation {
    fn parse(captures: &Captures, map: &Map) -> Self {
        if let Some(op) = captures.name("op") {
            match op.as_str() {
                "NOT" => Self::NOT(parse_signal(captures.name("sig2"), map)),
                "AND" => Self::AND(parse_signal(captures.name("sig1"), map), parse_signal(captures.name("sig2"), map)),
                "OR" => Self::OR(parse_signal(captures.name("sig1"), map), parse_signal(captures.name("sig2"), map)),
                "LSHIFT" => Self::LSHIFT(parse_signal(captures.name("sig1"), map), parse_signal(captures.name("sig2"), map)),
                "RSHIFT" => Self::RSHIFT(parse_signal(captures.name("sig1"), map), parse_signal(captures.name("sig2"), map)),
                _ => panic!("Invalid operation! Should not be possible"),
            }.to_value()
        } else {
            match parse_signal(captures.name("sig2"), map) {
                Operand::Value(i) => Self::Value(i),
                Operand::Wire(name) => Self::EQ(name),
            }
        }
    }
    fn eval(&self, map: &mut Map) -> u16 {
        match self {
            Self::NOT(o) => !o.get(map),
            Self::AND(a, b) => a.get(map) & b.get(map),
            Self::OR(a, b) => a.get(map) | b.get(map),
            Self::LSHIFT(a, b) => a.get(map) << b.get(map),
            Self::RSHIFT(a, b) => a.get(map) >> b.get(map),
            Self::EQ(w) => eval_wire(w, map),
            Self::Value(i) => *i,
        }
    }
    fn to_value(self) -> Self {
        match self {
            Self::NOT(o) => if let Operand::Value(i) = o { Self::Value(!i) } else { Self::NOT(o) }
            Self::AND(a, b) => match a {
                Operand::Value(i) => if let Operand::Value(j) = b { Self::Value(i & j) } else { Self::AND(a, b) },
                _ => Self::AND(a, b),
            },
            Self::OR(a, b) => match a {
                Operand::Value(i) => if let Operand::Value(j) = b { Self::Value(i & j) } else { Self::OR(a, b) },
                _ => Self::OR(a, b),
            },
            Self::LSHIFT(a, b) => match a {
                Operand::Value(i) => if let Operand::Value(j) = b { Self::Value(i & j) } else { Self::LSHIFT(a, b) },
                _ => Self::LSHIFT(a, b),
            },
            Self::RSHIFT(a, b) => match a {
                Operand::Value(i) => if let Operand::Value(j) = b { Self::Value(i & j) } else { Self::RSHIFT(a, b) },
                _ => Self::RSHIFT(a, b),
            },
            _ => self,
        }
    }
}