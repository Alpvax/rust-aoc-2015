#[macro_use]extern crate lazy_static;

use fancy_regex::Regex;

const DISALLOWED: [u8; 3] = ['i' as u8, 'o' as u8, 'l' as u8];

fn increment_char(c: &mut u8) -> bool {
    if *c == ('z' as u8) {
        *c = 'a' as u8;
        return true;
    }
    *c += 1;
    if DISALLOWED.contains(c) {
        *c += 1;
    }
    false
}

fn increment<'a>(bytes: &'a mut [u8]) -> &mut [u8] {
    bytes.reverse();
    for c in &mut bytes[..] {
        if !increment_char(c) {
            break;
        }
    }
    bytes.reverse();
    bytes
}

fn check_adjacent(bytes: &[u8]) -> bool {
    bytes[0] + 1 == bytes[1] && bytes[1] + 1 == bytes[2]
}

fn check_bytes(bytes: &[u8]) -> bool {
    let mut flag = false;
    for i in 0..5 {
        if check_adjacent(&bytes[i..(i + 3)]) {
            flag = true;
            break;
        }
    }
    flag && {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"([a-z])\1").unwrap();
        }
        let s = String::from_utf8(bytes.to_vec()).unwrap();
        match PATTERN.find(&s).unwrap() {
            Some(m) => {
                let c = m.as_str();
                let i = m.end();
                loop {
                    if let Some(n) = PATTERN.find(&s[i..]).unwrap() {
                        if n.as_str() != c {
                            return true
                        }
                    } else {
                        return false;
                    }
                }
            },
            None => false,
        }
    }
}

fn main() {
    let s = &mut String::from("hxbxwxbz");
    let mut bytes = unsafe { s.as_bytes_mut() };
    let mut c = 0;
    loop {
        increment(&mut bytes);
        if check_bytes(&bytes) {
            c += 1;
            println!("Valid ({}): {}", c, String::from_utf8(bytes.to_vec()).unwrap());
            if c >= 2 {
                break;
            }
        }
    }
}