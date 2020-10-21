#[macro_use]extern crate lazy_static;

use rust_2015::read_lines;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;

const MATCH_DATA: Sue = Sue {
    number: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

fn main() {
    let sue = read_lines("puzzle-input/16.0.txt", |s| s.parse::<Sue>().expect("Unable to parse Sue"))
        .find(|s| s.matches(&MATCH_DATA)).unwrap();
    println!("{:?}", sue);
}

#[derive(Debug)]
struct Sue {
    number: u16,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}
impl Sue {
    fn matches(&self, other: &Sue) -> bool {
        macro_rules! optional_match {
            ($a:expr, < $b:expr) => {
                match ($a, $b) {
                    (Some(a), Some(b)) => a < b,
                    _ => true,
                }
            };
            ($a:expr, > $b:expr) => {
                match ($a, $b) {
                    (Some(a), Some(b)) => a > b,
                    _ => true,
                }
            };
            ($a:expr, $b:expr) => {
                match ($a, $b) {
                    (Some(a), Some(b)) => a == b,
                    _ => true,
                }
            };
        }
        return
            optional_match!(self.children, other.children)
            && optional_match!(self.cats, > other.cats)
            && optional_match!(self.samoyeds, other.samoyeds)
            && optional_match!(self.pomeranians, < other.pomeranians)
            && optional_match!(self.akitas, other.akitas)
            && optional_match!(self.vizslas, other.vizslas)
            && optional_match!(self.goldfish, < other.goldfish)
            && optional_match!(self.trees, > other.trees)
            && optional_match!(self.cars, other.cars)
            && optional_match!(self.perfumes, other.perfumes)
    }
}
impl FromStr for Sue {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(
                r"Sue (?P<number>\d+):(?:,? (?:children: (?P<children>\d+)|cats: (?P<cats>\d+)|samoyeds: (?P<samoyeds>\d+)|pomeranians: (?P<pomeranians>\d+)|akitas: (?P<akitas>\d+)|vizslas: (?P<vizslas>\d+)|goldfish: (?P<goldfish>\d+)|trees: (?P<trees>\d+)|cars: (?P<cars>\d+)|perfumes: (?P<perfumes>\d+)))+"
            ).unwrap();
        }
        let captures = PATTERN.captures(s).unwrap();
        Ok(Sue {
            number: captures.name("number").unwrap().as_str().parse().unwrap(),
            children: captures.name("children").map(|m| m.as_str().parse().unwrap()),
            cats: captures.name("cats").map(|m| m.as_str().parse().unwrap()),
            samoyeds: captures.name("samoyeds").map(|m| m.as_str().parse().unwrap()),
            pomeranians: captures.name("pomeranians").map(|m| m.as_str().parse().unwrap()),
            akitas: captures.name("akitas").map(|m| m.as_str().parse().unwrap()),
            vizslas: captures.name("vizslas").map(|m| m.as_str().parse().unwrap()),
            goldfish: captures.name("goldfish").map(|m| m.as_str().parse().unwrap()),
            trees: captures.name("trees").map(|m| m.as_str().parse().unwrap()),
            cars: captures.name("cars").map(|m| m.as_str().parse().unwrap()),
            perfumes: captures.name("perfumes").map(|m| m.as_str().parse().unwrap()),
        })
    }
}