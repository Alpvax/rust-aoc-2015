use std::collections::HashMap;

fn main() {
    let mut containers = [33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42];
    containers.sort();
    containers.reverse();
    //let containers = [20, 15, 10, 5, 5];
    let mut count = 0;
    let mut counts = HashMap::new();
    store_largest(150, &containers, &mut count, &mut counts, Vec::new());
    println!("Part 1: {}\nPart 2: {}", count, counts.iter().min_by_key(|e| e.0).unwrap().1);
}

fn store_largest(remaining: u8, containers: &[u8], count: &mut u32, counts: &mut HashMap<usize, u32>, used: Vec<u8>) {
    for i in 0..containers.len() {
        let c = containers[i];
        if c > remaining {
            continue;
        }
        //println!("Filling: {:?} with {}. Currently testing: {}; remaining: {}", &containers[i..], remaining, c, remaining as i16 - c as i16);
        let con_sum = &containers[i..].iter().fold(0, |acc, x| acc + *x as u16);
        if con_sum < &(remaining as u16) {
            //println!("Skipping remaining containers: {:?} (Can store: {} required: {})", &containers[i..], con_sum, remaining);
            break;
        } else if c < remaining {
            //println!("Used: {:?} (= {}); Current: {}; Remaining: {:?}; Remaining fluid: {} - {};", used, used.iter().sum::<u8>(), c, &containers[i+1..], remaining, c);
            store_largest(remaining - c, &containers[i+1..], count, counts, used.iter().chain([c].iter()).cloned().collect());
        } else if c == remaining {
            //println!("SUCCESS!! {:?}", used.iter().chain([c].iter()).cloned().collect::<Vec<_>>());
            *count += 1;
            *counts.entry(used.len()).or_insert(0) += 1;
        }
    }
}