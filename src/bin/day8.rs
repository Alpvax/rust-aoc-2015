use rust_2015::read_lines;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let pattern = Regex::new(r#"\\\\|\\"|\\x[0-9a-f]{2}"#).unwrap();
    println!("{}", read_lines("puzzle-input/8.0.txt", |s| s).map(|line| {
        line.chars().count() - (pattern.replace_all(&line, "|").chars().count() + 2)
    }).sum::<usize>());
}

fn part2() {
    let pattern = Regex::new(r#"("|\\)"#).unwrap();
    let count: usize = read_lines("puzzle-input/8.0.txt", |s| s).map(|line| {
        pattern.replace_all(&line, "\\$1").chars().count() + 2 - line.chars().count()
    }).sum();
    println!("{}", count);
}