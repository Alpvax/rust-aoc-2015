use rust_2015::read_lines;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let presents = read_lines("puzzle-input/2.0.txt", |s| s.parse::<Present>().expect("Unable to parse present"));
    //presents_area(presents);
    ribbon_qty(presents);
}

fn _presents_area(it: Box<dyn Iterator<Item=Present>>) {
    println!("{}", it.map(|p| _parse_present_area(&p)).sum::<u32>());
}

fn _parse_present_area(p: &Present) -> u32 {
    let areas = [p.width * p.height, p.width * p.length, p.height * p.length];
    // Presents already property-sorted. areas.sort();
    3 * areas[0] + 2 * (areas[1] + areas[2])
}

fn ribbon_qty(it: Box<dyn Iterator<Item=Present>>) {
    println!("{}", it.map(|p| parse_ribbon_length(&p)).sum::<u32>());
}

fn parse_ribbon_length(p: &Present) -> u32 {
    2 * (p.width + p.height) + (p.width * p.height * p.length)
}

#[derive(Debug)]
struct Present {
    width: u32, //Smallest
    height: u32,
    length: u32, //Largest
}

impl FromStr for Present {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums: Vec<u32> = s.split('x')
                                  .map(|s| s.parse::<u32>()
                                            .expect("Unable to parse int")
                                  )
                                  .collect();
        nums.sort();
        Ok(Present { width: nums[0], height: nums[1], length: nums[2] })
    }
}