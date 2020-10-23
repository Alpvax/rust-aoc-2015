use rust_2015::read_lines;
use std::collections::HashMap;
use fancy_regex::Regex;

type Map = HashMap<String, Reindeer>;

fn main() {
    let mut map = HashMap::new();
    let mut first = String::from("<NONE>");
    build_map(&mut map, &mut first);
    /*let mut seconds = 0;
    while seconds < 2503 {
        let next = map.get(first).unwrap().state.unwrap();
    }*/
    let mut max_score = 0;
    for _seconds in 0..2503 {
        first = map.values_mut().map(|r| r.tick(1)).max_by_key(|r| r.distance).unwrap().name.to_string();
        let f = map.get_mut(&first).unwrap();
        f.score += 1;
        max_score = f.score;
    }
    println!("Part 2: {:?}\n{:?}", max_score, map.values().collect::<Vec<_>>());
}

fn build_map(map: &mut Map, first: &mut String) {
    let pattern = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
    let mut max = 0;
    let mut top_speed = 0;
    for line in read_lines("puzzle-input/14.0.txt", |s| s) {
        let caps = pattern.captures(&line).expect("Regex parsing error").expect("No match found");
        let r = Reindeer::new(
            caps.get(1).unwrap().as_str().to_string(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        );
        let d = r.calculate_distance(2503);
        if d > max {
            max = d;
        }
        if r.speed > top_speed {
            top_speed = r.speed;
            *first = r.name.to_string();
        }
        map.insert(r.name.to_string(), r);
    }
    println!("Part 1: {}", max);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum State {
    Fly(u32),
    Rest(u32),
}
impl State {
    fn unwrap(&self) -> u32 {
        match self {
            Self::Fly(i) => *i,
            Self::Rest(i) => *i,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    score: u32,
    distance: u32,
    state: State,
}

impl Reindeer {
    const fn new(name: String, speed: u32, fly_time: u32, rest_time: u32) -> Reindeer {
        Reindeer {
            name, speed, fly_time, rest_time, score: 0, distance: 0, state: State::Fly(fly_time),
        }
    }
    fn calculate_distance(&self, seconds: u32) -> u32 {
        let sum_time = self.fly_time + self.rest_time;
        let fly_duration = self.fly_time * (seconds / sum_time) + std::cmp::min(self.fly_time, seconds % sum_time);
        self.speed * fly_duration
    }
    fn tick(&mut self, seconds: u32) -> &Self {
        let mut remaining = seconds;
        while remaining > 0 {
            let time = self.state.unwrap();
            if let State::Fly(_i) = self.state {
                self.distance += std::cmp::min(time, remaining) * self.speed;
            }
            if remaining >= time {
                self.state = match self.state {
                    State::Fly(_) => State::Rest(self.rest_time),
                    State::Rest(_) => State::Rest(self.fly_time),
                }
            }
            remaining -= std::cmp::min(remaining, time);
        }
        self
    }
    /*fn time_to(&self, distance: u32) -> u32 {
        let delta = distance - self.distance;

    }*/
}
