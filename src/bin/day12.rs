extern crate serde_json;

use rust_2015::parse_file;
use regex::Regex;
use serde_json::Value;

fn main() {
    parse_file("puzzle-input/12.0.json", part1);
    parse_file("puzzle-input/12.0.json", part2);
}

fn part1(s: &str) {
    println!("Part 1: {}", Regex::new(r"-?\d+").unwrap().find_iter(s).map(|m| m.as_str().parse::<isize>().unwrap()).sum::<isize>());
}

fn part2(s: &str) {
    let json: Value = serde_json::from_str(s).unwrap();
    println!("Part 2: {}", sum_json(&json));
}

fn sum_json(json: &Value) -> i64 {
    return if json.is_array() {
        json.as_array().unwrap().iter().map(|j| sum_json(j)).sum()
    } else if json.is_object() {
        let o = json.as_object().unwrap();
        if o.values().any(|v| v.is_string() && v.as_str().unwrap() == "red") {
            0
        } else {
            o.values().map(|j| sum_json(j)).sum()
        }
    } else if json.is_number() {
        json.as_i64().unwrap()
    } else {
        0
    }
}