use rust_2015::parse_file;

fn count(s: &str) {
    let floor = s.matches("(").count() - s.matches(")").count();
    println!("{}", floor);
}

fn first_basement(s: &str) {
    let mut floor = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }
        else if c == ')' {
            floor -= 1;
            if floor < 0 {
                println!("{}", i + 1);
                break;
            }
        }
        //println!("{}", floor);
    }
}

fn main() {
    //parse_file("puzzle-input/1.0.txt", count);
    parse_file("puzzle-input/1.0.txt", first_basement);
}