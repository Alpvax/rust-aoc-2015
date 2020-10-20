use rust_2015::read_lines;
//use std::collections::HashMap;
use fancy_regex::Regex;

fn main() {
    let pattern = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
    //let map = HashMap::new();
    let mut max = 0;
    for line in read_lines("puzzle-input/14.0.txt", |s| s) {
        let caps = pattern.captures(&line).expect("Regex parsing error").expect("No match found");
        let r = Reindeer {
            name: caps.get(1).unwrap().as_str().to_string(),
            speed: caps.get(2).unwrap().as_str().parse().unwrap(),
            fly_time: caps.get(3).unwrap().as_str().parse().unwrap(),
            rest_time: caps.get(4).unwrap().as_str().parse().unwrap(),
        };
        let d = r.move_for(2503);
        if d > max {
            max = d;
        }
        //map.insert(r.name, r);
    }
    println!("Part 1: {}", max);
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn move_for(&self, seconds: u32) -> u32 {
        let sum_time = self.fly_time + self.rest_time;
        let fly_duration = self.fly_time * (seconds / sum_time) + std::cmp::min(self.fly_time, seconds % sum_time);
        self.speed * fly_duration
    }
}
