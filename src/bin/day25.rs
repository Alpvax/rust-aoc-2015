fn main() {
    let num = get_num(3010, 3019);
    println!("Calculating value {}.", num);
    let mut val = 20151125u64;
    for n in 2..=num {
        val = (val * 252533) % 33554393;
    }
    println!("Part 1: {}", val);
}

fn get_num(row: u64, col: u64) -> u64 {
    let line = col + row;
    (line - 1) * (line - 2) / 2 + col
}