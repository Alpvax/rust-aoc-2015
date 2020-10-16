

fn main() {
    let input = "3113322113";
    let mut val = String::from(input);
    for i in 0..50 {
        val = run_for(&val);
        if i == 39 {
            println!("After 40: {}", val.chars().count());
        }
    }
    println!("After 50: {}", val.chars().count());
    /*let mut result = String::new();
    let mut c = input.chars().next().unwrap();
    let mut count = 0;
    for n in input.chars() {
        if n == c {
                count += 1;
        } else {
            result = format!("{}{}{}", result, count, c);
            c = n;
            count = 1
        }
    }
    println!("{}", result);*/
}

fn run_for(s: &str) -> String {
    let mut chars = s.chars();
    let mut result = String::new();
    let mut c = chars.next().unwrap();
    let mut count = 1;
    loop {
        match chars.next() {
            Some(n) => {
                if n == c {
                    count += 1;
                } else {
                    result = format!("{}{}{}", result, count, c);
                    c = n;
                    count = 1
                }
            },
            None => {
                result = format!("{}{}{}", result, count, c);
                break;
            },
        }
    }
    result
}