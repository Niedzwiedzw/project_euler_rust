//By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
//What is the 10 001st prime number?

fn main() {
    let prime_number_index = 10_001;
    let mut prime_number = 2;
    for _i in 1..prime_number_index {
        prime_number = next_prime(prime_number);
    }
    print!("{}th prime number is {}.",prime_number_index, prime_number);
}

fn is_prime(number: i32) -> bool {
    if number == 1 {
        return false
    }
    for i in 2..((number as f32).sqrt() as i32 + 1){
        if number % i == 0 {
            return false
        }
    }
    true
}

fn next_prime(number: i32) -> i32 {
    let mut n = number;
    loop {
        n += 1;
        if is_prime(n) {
            return n;
        }
    }
}