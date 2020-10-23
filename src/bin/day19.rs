#[macro_use]extern crate lazy_static;

use rust_2015::read_lines;
use std::collections::{HashMap, HashSet};
use regex::Regex;


fn main() {
    let mut map = HashMap::new();
    let input = load(&mut map);
    //println!("Input: {}\n{:?}", input, map);
    println!("Part 1: {}", part1(&input, &map));
    println!("Part 2: {}", part2(&input, &map));
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
    let target_len = target.len(); // ASCII, so len consistent
    let mut processing = HashSet::new();
    processing.insert(String::from("e"));
    let mut processed = HashSet::new();
    let mut count = 0;
    loop {
        count += 1;
        //println!("Iteration {}: Processing: {}\n{:?}", count, processing.len(), processing);
        let mut next = HashSet::new();
        for s in processing.iter().cloned() {
            processed.insert(s.to_owned());
            match replace1(&s, &map, target, target_len) {
                ReplResult::Found => return count,
                ReplResult::Produced(set) => {
                    for res in set {
                        next.insert(res);
                    }
                }
            }
        }
        processing.clear();
        for res in next.difference(&processed) {
            processing.insert(res.to_string());
        }
    }
}

fn replace1(input: &str, map: &HashMap<String, Vec<String>>, target: &str, target_len: usize) -> ReplResult {
    let mut results = HashSet::new();
    for (k, v) in map.iter() {
        for m in Regex::new(&k).unwrap().find_iter(input) {
            for r in v {
                let s = format!("{}{}{}", &input[..m.start()], r, &input[m.end()..]);
                if s == target {
                    return ReplResult::Found;
                } else if s.len() < target_len {
                    results.insert(s);
                }
            }
        }
    }
    ReplResult::Produced(results)
}

enum ReplResult {
    Found,
    Produced(HashSet<String>),
}
