use rust_2015::read_lines;
use itertools::Itertools;
use std::collections::HashMap;
use fancy_regex::Regex;

type Map = HashMap<String, HashMap<String, i32>>;

fn main() {
    let pattern = Regex::new(r"(?P<A>\w+) would (?:gain|(?P<l>lose)) (?P<h>\d+) happiness units by sitting next to (?P<B>\w+)\.").unwrap();
    let mut map = HashMap::new();
    for line in
    /*[
        "Alice would gain 54 happiness units by sitting next to Bob.",
        "Alice would lose 79 happiness units by sitting next to Carol.",
        "Alice would lose 2 happiness units by sitting next to David.",
        "Bob would gain 83 happiness units by sitting next to Alice.",
        "Bob would lose 7 happiness units by sitting next to Carol.",
        "Bob would lose 63 happiness units by sitting next to David.",
        "Carol would lose 62 happiness units by sitting next to Alice.",
        "Carol would gain 60 happiness units by sitting next to Bob.",
        "Carol would gain 55 happiness units by sitting next to David.",
        "David would gain 46 happiness units by sitting next to Alice.",
        "David would lose 7 happiness units by sitting next to Bob.",
        "David would gain 41 happiness units by sitting next to Carol."
    ].iter() {*/
    read_lines("puzzle-input/13.0.txt", |s| s) {
        let caps = pattern.captures(&line).expect("Regex parsing error").expect("No match found");
        let a = caps.name("A").unwrap().as_str();
        let b = caps.name("B").unwrap().as_str();
        let h = caps.name("h").unwrap().as_str().parse::<i32>().unwrap() * ( if let Some(_) = caps.name("l") { -1 } else { 1 } );
        let am = map.entry(a.to_string()).or_insert(HashMap::new());
        am.insert(b.to_string(), h);
    }
    println!("Part 1: {}", get_max_happiness(&map));
    let mut me = HashMap::new();
    for (name, vals) in map.iter_mut() {
        me.insert(name.to_string(), 0);
        vals.insert("_".to_string(), 0);
    }
    map.insert("_".to_string(), me);
    println!("Part 2: {}", get_max_happiness(&map));
}

fn get_max_happiness(map: &Map) -> i32 {
    map.keys().permutations(map.len()).map(|order| process(map, &order[..])).max().unwrap()
}

fn process(map: &Map, order: &[&String]) -> i32 {
    let l = order.len();
    let mut sum = 0;
    for i in 0..l {
        let left = (i + l - 1) % l;
        let right = (i + 1) % l;
        let person = map.get(order[i]).unwrap();
        //println!("{}: l={} ({}); r={}({})", order[i], order[left], person.get(order[left]).unwrap(), order[right], person.get(order[right]).unwrap());
        sum += person.get(order[left]).unwrap() + person.get(order[right]).unwrap();
    }
    sum
}
