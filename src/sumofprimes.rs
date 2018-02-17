//The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
//Find the sum of all the primes below two million.


fn main() {
    let mut sum_of_primes: i64 = 0;
    let mut prime: i64 = next_prime(1);
    loop {
        if prime >= 2_000_000 {
            break
        }
        sum_of_primes += prime;
        prime = next_prime(prime);
    }
    println!("Sum of all the primes below two million is {}.", sum_of_primes);
}

fn is_prime(number: i64) -> bool {
    if number == 1 {
        return false
    }
    for i in 2..((number as f64).sqrt() as i64 + 1){
        if number % i == 0 {
            return false
        }
    }
    true
}

fn next_prime(number: i64) -> i64 {
    let mut n = number;
    loop {
        n += 1;
        if is_prime(n) {
            return n;
        }
    }
}
