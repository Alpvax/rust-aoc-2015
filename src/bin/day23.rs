use rust_2015::read_lines;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
enum Instruction {
    Hlf(char), //hlf r sets register r to half its current value, then continues with the next instruction.
    Tpl(char), //tpl r sets register r to triple its current value, then continues with the next instruction.
    Inc(char), //inc r increments register r, adding 1 to it, then continues with the next instruction.
    Jmp(i16), //jmp offset is a jump; it continues with the instruction offset away relative to itself.
    Jie(char, i16), //jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
    Jio(char, i16), //jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
}
impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().skip(4);
        match &s[..3] {
            "hlf" => Ok(Self::Hlf(chars.next().unwrap())),
            "tpl" => Ok(Self::Tpl(chars.next().unwrap())),
            "inc" => Ok(Self::Inc(chars.next().unwrap())),
            "jmp" => Ok(Self::Jmp(chars.collect::<String>().parse()?)),
            "jie" => Ok(Self::Jie(chars.next().unwrap(), chars.skip(2).collect::<String>().parse()?)),
            "jio" => Ok(Self::Jio(chars.next().unwrap(), chars.skip(2).collect::<String>().parse()?)),
            _ => panic!("Invalid instruction"),
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> =
        /*[
            "inc a",
            "jio a, +2",
            "tpl a",
            "inc a"
        ].iter()*/
        read_lines("puzzle-input/23.0.txt", |s| s)
        .map(|s| {
            match s.parse::<Instruction>() {
                Ok(i) => i,
                Err(e) => {
                    println!("Error parsing \"{}\": {:?}", s, e);
                    panic!();
                },
            }
        })
        .collect();
    println!("Part 1: {}\nPart 2: {}", run_program(0, &instructions).1, run_program(1, &instructions).1);
}

fn run_program(initial_a: u32, instructions: &Vec<Instruction>) -> (u32, u32) {
    //println!("Running program with {} instructions", instructions.len());
    let mut a: u32 = initial_a;
    let mut b: u32 = 0;
    let mut index: usize = 0;
    loop {
        //println!("a: {}; b: {}; index: {}", a, b, index);
        match instructions.get(index) {
            Some(inst) => {
                //println!("Current instruction: {:?}", inst);
                match inst {
                    Instruction::Hlf(r) => {
                        update_register(&mut a, &mut b, r, |i| i / 2);
                        index += 1;
                    },
                    Instruction::Tpl(r) => {
                        update_register(&mut a, &mut b, r, |i| i * 3);
                        index += 1;
                    },
                    Instruction::Inc(r) => {
                        update_register(&mut a, &mut b, r, |i| i + 1);
                        index += 1;
                    },
                    Instruction::Jmp(o) => {
                        index = (index as i16 + o) as usize;
                    },
                    Instruction::Jie(r, o) => {
                        if (if let 'a' = r { a } else { b }) % 2 == 0 {
                            index = (index as i16 + o) as usize;
                        } else {
                            index += 1;
                        }
                    },
                    Instruction::Jio(r, o) => { //JUMP IF ONE, not JUMP IF ODD!!
                        if (if let 'a' = r { a } else { b }) == 1 {
                            index = (index as i16 + o) as usize;
                        } else {
                            index += 1;
                        }
                    },
                }
            },
            None => return (a, b),
        }
    }
}

fn update_register(a: &mut u32, b: &mut u32, r: &char, f: fn(u32) -> u32) {
    match r {
        'a' => *a = f(*a),
        'b' => *b = f(*b),
        _ => (), //Unrecognised register!
    };
}