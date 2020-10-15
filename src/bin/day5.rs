use rust_2015::read_lines;
use fancy_regex::Regex;

#[derive(Debug)]
enum ElfString {
    Nice,
    Naughty,
}
impl ElfString {
    fn is_nice(&self) -> bool {
        match self {
            ElfString::Nice => true,
            _ => false,
        }
    }
}

fn main() {
    let patterns1 = vec!(
        Regex::new(r"ab|cd|pq|xy").unwrap(),
        Regex::new(r"([[:alpha:]])\1").unwrap(),
        Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap(),
    );
    let patterns2 = vec!(
        Regex::new(r"([[:alpha:]]{2}).*\1").unwrap(),
        Regex::new(r"([[:alpha:]]).\1").unwrap(),
    );
    let result: Vec<_> = /*
    [
        "ugknbfddgicrmopn", // is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
        "aaa", // is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
        "jchzalrnumimnmhp", // is naughty because it has no double letter.
        "haegwjzuvuyypxyu", // is naughty because it contains the string xy.
        "dvszwmarrgswjxmb", // is naughty because it contains only one vowel.
    ].iter()*/
    read_lines("puzzle-input/5.0.txt", |s| s)
        //.map(|s| check_nice_1(&patterns1, &s))
        .map(|s| check_nice_2(&patterns2, &s))
        .filter(|e| e.is_nice())
        .collect();
    println!("{}", result.len());
}

fn check_nice_1(patterns: &Vec<Regex>, s: &str) -> ElfString {
    if !patterns[0].is_match(s).unwrap() {
        if patterns[1].is_match(s).unwrap() && patterns[2].is_match(s).unwrap() {
            return ElfString::Nice;
        }
    }
    ElfString::Naughty
}

fn check_nice_2(patterns: &Vec<Regex>, s: &str) -> ElfString {
    if patterns[0].is_match(s).unwrap() && patterns[1].is_match(s).unwrap() {
        return ElfString::Nice;
    }
    ElfString::Naughty
}