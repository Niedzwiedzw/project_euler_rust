//Sum square difference

//The sum of the squares of the first ten natural numbers is,
//12 + 22 + ... + 102 = 385
//
//The square of the sum of the first ten natural numbers is,
//(1 + 2 + ... + 10)2 = 552 = 3025
//
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let max_num:i32 = 100;
    let mut sum_of_squares = 0;
    for i in 1..(max_num+1) {
        sum_of_squares += i.pow(2 as u32);
    }

    let square_of_sum = ((max_num*(max_num+1))/2).pow(2 as u32);
    print!("difference is {}", square_of_sum -sum_of_squares);
}