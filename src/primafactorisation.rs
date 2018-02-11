//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
use std::collections::HashMap;

fn main() {
    let max_num = 10;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 1..(max_num + 1) {
        prime_factors(i, &mut map);
    }
    let mut smallest_number = 1;
    for (key, value) in &map {
        smallest_number *= key.pow(*value as u32);
    }
    println!("smallest positive number that is evenly divisible by all of the numbers from 1 to {} is {}",max_num, smallest_number);
}

fn prime_factors(number: i32, map: &mut HashMap<i32, i32>) {
    let mut n = number;
    for i in 2..(n + 1) {
        let mut occurences = 0;
        loop {
            if n % i == 0 {
                occurences += 1;
                n /= i;
            } else {
                break
            }
        }
        if occurences > 0 {
            let val = map.entry(i).or_insert(0);
            if *val < occurences {
                *val = occurences
            }
        }
    }
}