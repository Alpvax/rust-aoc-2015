#[macro_use]extern crate lazy_static;

use rust_2015::read_lines;
use std::collections::{HashMap, HashSet};
use regex::Regex;


fn main() {
    let mut map = HashMap::new();
    let input = load(&mut map);
    //println!("Input: {}\n{:?}", input, map);
    println!("Part 1: {:?}", part1(&input, &map));
}

fn load(map: &mut HashMap<String, Vec<String>>) -> String {
    lazy_static!{
        static ref PATTERN: Regex = Regex::new("([A-Za-z]+) => ([A-za-z]+)").unwrap();
    }
    for line in read_lines("puzzle-input/19.0.txt", |s| s) {
        match PATTERN.captures(&line) {
            Some(m) => {
                map.entry(m.get(1).unwrap().as_str().to_string()).or_insert(Vec::new())
                    .push(m.get(2).unwrap().as_str().to_string());
            },
            None => {
                if line.len() > 0 {
                    return line;
                }
            }
        }
    }
    panic!("No input line found!");
}

fn part1(input: &str, map: &HashMap<String, Vec<String>>) -> usize {
    let mut results = HashSet::new();
    for (k, v) in map.iter() {
        for m in Regex::new(&k).unwrap().find_iter(input) {
            for r in v {
                results.insert(format!("{}{}{}", &input[..m.start()], r, &input[m.end()..]));
            }
        }
    }
    results.len()
}

fn part2(target: &str, map: &HashMap<String, Vec<String>>) -> u32 {
    let target_len = target.chars().size();
    let mut min_steps = u32::MAX;

}
