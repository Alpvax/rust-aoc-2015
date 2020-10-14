use rust_2015::parse_file;
use std::collections::{HashMap, VecDeque};


fn main() {
    let map = parse_file("puzzle-input/3.0.txt", make_map);
    println!("{}", map.len());
}

fn make_map(s: &str) -> HashMap<Point, u32> {
    let mut curr = VecDeque::with_capacity(2);
    curr.push_back(Point::new(0, 0)); //Santa
    curr.push_back(Point::new(0, 0)); //robo
    let mut map = HashMap::new();
    for c in s.chars() {
        let mut current = curr.front_mut().expect("Where is my Santa?!");
        let p = Point::of(&current);
        let count = map.entry(p).or_insert(1);
        *count += 1;
        match c {
            '^' => current.up(),
            'v' => current.down(),
            '<' => current.left(),
            '>' => current.right(),
            _ => println!("{}", c),
        }
        curr.rotate_right(1);
    }
    map
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn of(p: &Point) -> Point {
        Point { x: p.x, y: p.y }
    }
    fn up(&mut self) {
        self.y += 1;
    }
    fn down(&mut self) {
        self.y -= 1;
    }
    fn left(&mut self) {
        self.x -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }
}