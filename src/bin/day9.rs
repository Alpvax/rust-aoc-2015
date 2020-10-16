use rust_2015::read_lines;
use itertools::Itertools;
use std::collections::HashMap;
use fancy_regex::Regex;

fn main() {
    let pattern = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut routemap/*: HashMap<String, HashMap<String, u32>>*/ = HashMap::new();
    for line in
    //["London to Dublin = 464", "London to Belfast = 518", "Dublin to Belfast = 141"].iter() {
    read_lines("puzzle-input/9.0.txt", |s| s) {
        let caps = pattern.captures(&line).expect("Regex parsing error").expect("No match found");
        let f = caps.get(1).unwrap().as_str();
        let t = caps.get(2).unwrap().as_str();
        let d: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let a = routemap.entry(f.to_string()).or_insert(HashMap::new());
        a.insert(t.to_string(), d);
        let b = routemap.entry(t.to_string()).or_insert(HashMap::new());
        b.insert(f.to_string(), d);
    }
    let mut shortest = u32::MAX;
    let mut longest = 0;
    for route in routemap.keys().permutations(routemap.len()) {
        let mut length = 0;
        let mut current = route[0];
        for node in &route[1..] {
            let d = routemap.get(current).unwrap().get::<str>(node).unwrap();
            length += d;
            current = node;
            //println!("{} -> {} = {}; Total = {}", current, node, d, length)
        }
        if length < shortest {
            shortest = length;
        }
        if length > longest {
            longest = length;
        }
        //println!("{:?} -> {}", route, length);
    }
    println!("Shortest: {}\n Longest: {}", shortest, longest);
}