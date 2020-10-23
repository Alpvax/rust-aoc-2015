use std::collections::HashMap;

fn main() {
    //println!("Part 1: {}", part1(29000000));
    println!("Part 2: {}", part2(29000000));
}

fn part1(target: u32) -> u32 {
    let l = target / 10;
    let mut data: HashMap<u32, u32> = HashMap::new(); // Eat RAM for performance
    for n in 1..l/2 {
        let e = data.entry(n).or_insert(0);
        *e += n * 10;
        if *e >= target {
            return n;
        }
        for h in (n*2..l).step_by(n as usize) {
            *data.entry(h).or_insert(0) += n * 10;
        }
    }
    l
}

fn part2(target: u32) -> u32 {
    let l = target / 10;
    let mut data: HashMap<u32, u32> = HashMap::new(); // Eat RAM for performance
    for n in 1..l {
        let e = data.entry(n).or_insert(0);
        *e += n * 11;
        if *e >= target {
            return n;
        }
        for h in 2..=50 {
            if h < l {
                *data.entry(h * n).or_insert(0) += n * 11;
            }
        }
    }
    l
}