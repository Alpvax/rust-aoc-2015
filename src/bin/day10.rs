fn main() {
    let input = "3113322113";
    let mut val = String::from(input);
    for i in 0..50 {
        val = run_for(&val);
        if i == 39 {
            println!("After 40: {}", val.len());
        }
    }
    println!("After 50: {}", val.len());
}

fn run_for(s: &str) -> String {
    let mut chars = s.chars();
    let mut result = String::new();
    let mut c = chars.next().unwrap();
    let mut count = 1;
    for n in chars {
        if n == c {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(c);
            c = n;
            count = 1
        }
    }
    format!("{}{}{}", result, count, c)
}