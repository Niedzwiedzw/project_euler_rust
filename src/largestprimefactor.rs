//The prime factors of 13195 are 5, 7, 13 and 29.
//What is the largest prime factor of the number 600851475143 ?
fn main() {
    let number:i64 = 600851475143;
    let mut largest_prime = -1;
    for i in 1..((number as f64).sqrt() as i64 + 1) {
        if is_prime(i) && (number % i == 0) {
            largest_prime = i;
        }
    }
    println!("");
    match largest_prime {
        -1 => println!("The number is 1 or is a prime number."),
        _ => println!("{} is the largest prime factor of {}.", largest_prime, number)
    }
}

fn is_prime(number: i64) -> bool {
    match number {
        1 => return false,
        _ => {
            for i in 2..(((number as f64).sqrt() as i64) + 1) {
                if number % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}
