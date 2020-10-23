use rust_2015::read_lines;
use std::collections::HashMap;
use strum::{IntoEnumIterator, EnumIter};

type Map = HashMap<Pos, u8>;

fn main() {
    let mut data = load(read_lines("puzzle-input/18.0.txt", |s| s));
    /*let mut data = load([
        ".#.#.#",
        "...##.",
        "#....#",
        "..#...",
        "#.#..#",
        "####..",
    ].iter().map(|&s| s.to_string()));*/
    for _ in 0..100 {
        //Part 2:
        data.turn_on_corners();
        //data.display(true);
        data.run_step();
    }
    //Part 2:
    data.turn_on_corners();
    data.display(false);
    println!("Grid: ({} x {})\nLights on: {}", data.xsize, data.ysize, data.lights.values().filter(|&l| *l != 0).count());
}

fn load(it: impl Iterator<Item=String>) -> Data {
    let mut map = HashMap::new();
    let mut xsize: u8 = 0;
    let mut ysize: u8 = 0;
    for (line_num, line) in it.enumerate() {
        let y = line_num as u8;
        for (char_num, c) in line.chars().enumerate() {
            let x = char_num as u8;
            map.insert(Pos { x, y }, match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Unknown character encountered"),
            });
            xsize = x;
        }
        ysize = y;
    }
    Data { xsize, ysize, lights: map }
}

fn neighbour_pos(x: u8, y: u8, d: &Direction) -> Option<Pos> {
    let dx = d.get_x();
    let dy = d.get_y();
    if (dx == -1 && x == 0) || (dx == 1 && x == 99) ||
        (dy == -1 && y == 0) || (dy == 1 && y == 99) {
        None
    } else {
        Some(Pos { x: (x as i8 + dx) as u8, y: (y as i8 + dy) as u8 })
    }
}

struct Data {
    xsize: u8,
    ysize: u8,
    lights: Map,
}
impl Data {
    fn display(&self, append_newline: bool) {
        for y in 0..self.ysize + 1 {
            println!("{}", (0..self.xsize + 1).map(|x| if self.get(x, y) != 0 { '#' } else { '.' }).collect::<String>());
        }
        if append_newline {
            println!("");
        }
    }
    fn get(&self, x: u8, y: u8) -> u8 {
        match self.lights.get(&Pos{x,y}) {
            Some(i) => *i,
            None => 0,
        }
    }
    fn run_step(&mut self) {
        let mut map: Map = HashMap::new();
        for (p, state) in self.lights.iter() {
            let num: u8 = Direction::iter().map(|d| neighbour_pos(p.x, p.y, &d).map_or(0, |p| self.get(p.x, p.y))).sum();
            let n = match num {
                3 => 1,
                2 => *state,
                _ => 0,
            };
            map.insert(*p, n);
        }
        self.lights = map;
    }
    fn turn_on_corners(&mut self) {
        self.lights.insert(Pos { x: 0, y: 0 }, 1);
        self.lights.insert(Pos { x: 0, y: self.ysize }, 1);
        self.lights.insert(Pos { x: self.xsize, y: 0 }, 1);
        self.lights.insert(Pos { x: self.xsize, y: self.ysize }, 1);
    }
}

#[derive(EnumIter, Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Direction { NW, N, NE, E, SE, S, SW, W }
impl Direction {
    fn get_x(&self) -> i8 {
        match self {
            Self::W | Self::NW | Self::SW => -1,
            Self::E | Self::NE | Self::SE => 1,
            _ => 0,
        }
    }
    fn get_y(&self) -> i8 {
        match self {
            Self::N | Self::NW | Self::NE => -1,
            Self::S | Self::SW | Self::SE => 1,
            _ => 0,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: u8,
    y: u8,
}
