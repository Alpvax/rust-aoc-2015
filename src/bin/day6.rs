use rust_2015::read_lines;
use fancy_regex::Regex;
use std::collections::HashMap;

fn main() {
    let pattern = Regex::new(r"(?:(?P<toggle>toggle)|turn (?:(?P<on>on)|(?P<off>off))) (?P<fx>\d+),(?P<fy>\d+) through (?P<tx>\d+),(?P<ty>\d+)").unwrap();
    //let mut map: HashMap<CoOrd, bool> = HashMap::new();
    let mut map: HashMap<CoOrd, i32> = HashMap::new();
    for ranged_op in read_lines("puzzle-input/6.0.txt", |s| s).map(|s| RangedOperation::new(&pattern, &s)) {
        ranged_op.apply_2(&mut map);
    }
    //1: println!("{}", map.values().filter(|b| **b).count());
    println!("{}", map.values().sum::<i32>());
    //println!("{:?}", CoOrdRange::new(2, 3, 5, 4).iter().collect::<Vec<_>>());
    /*println!("{:?}", RangedOperation::new(&pattern, "turn on 0,0 through 999,999"));
    println!("{:?}", RangedOperation::new(&pattern, "toggle 0,0 through 999,0"));
    println!("{:?}", RangedOperation::new(&pattern, "turn off 499,499 through 500,500"));*/
    /*RangedOperation { op: Op::On, range: CoOrdRange::new(0, 0, 2, 2)}.apply(&mut map);
    println!("{:?}", map);
    RangedOperation { op: Op::Toggle, range: CoOrdRange::new(1, 1, 3, 2)}.apply(&mut map);
    println!("{:?}", map);*/
}

#[derive(Debug)]
struct RangedOperation {
    op: Op,
    range: CoOrdRange,
}

impl RangedOperation {
    fn new(pattern: &Regex, s: &str) -> RangedOperation {
        let captures = pattern.captures(s).expect("Regex parsing error").expect("No match found");
        let op = if let Some(_) = captures.name("toggle") {
            Op::Toggle
        } else if let Some(_) = captures.name("on") {
            Op::On
        } else {
            Op::Off
        };
        let range = CoOrdRange::new(
            captures.name("fx").unwrap().as_str().parse().unwrap(),
            captures.name("fy").unwrap().as_str().parse().unwrap(),
            captures.name("tx").unwrap().as_str().parse().unwrap(),
            captures.name("ty").unwrap().as_str().parse().unwrap(),
        );
        RangedOperation { op, range }
    }
    fn apply_1(&self, map: &mut HashMap<CoOrd, bool>) {
        for c in self.range.iter() {
            let e = map.entry(c).or_insert(false);
            *e = match self.op {
                Op::On => true,
                Op::Off => false,
                Op::Toggle => !*e
            }
        }
    }
    fn apply_2(&self, map: &mut HashMap<CoOrd, i32>) {
        for c in self.range.iter() {
            let e = map.entry(c).or_insert(0);
            *e = std::cmp::max(0, *e + match self.op {
                Op::On => 1,
                Op::Off => -1,
                Op::Toggle => 2
            });
        }
    }
}

#[derive(Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct CoOrd {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct CoOrdRange {
    from: CoOrd,
    to: CoOrd,
}

impl CoOrdRange {
    fn new(fx: u32, fy: u32, tx: u32, ty: u32) -> CoOrdRange {
        CoOrdRange {
            from: CoOrd { x: fx, y: fy },
            to: CoOrd { x: tx, y: ty },
        }
    }
    fn iter(&self) -> RangeIter {
        RangeIter {
            range: self,
            x: self.from.x,
            y: self.from.y,
        }
    }
}

struct RangeIter<'a> {
    range: &'a CoOrdRange,
    x: u32,
    y: u32,
}

impl<'a> Iterator for RangeIter<'a> {
    type Item = CoOrd;
    fn next(&mut self) -> Option<CoOrd> {
        let x = self.x;
        let y = self.y;
        if x < self.range.to.x {
            self.x += 1;
        } else {
            self.x = self.range.from.x;
            self.y += 1;
        }
        return if y > self.range.to.y { None } else { Some(CoOrd { x, y }) };
    }
}