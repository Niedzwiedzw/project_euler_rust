//A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
//Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut largest = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            if is_palindrome(i*j) && largest < i*j {
                largest = i*j;
            }
        }
    }
    println!("The largest number is {}", largest);
}

fn is_palindrome(number: i32) -> bool {
    let num = number.to_string().into_bytes();
    for i in 0..((num.len() / 2) + 1) {
        if num[i] != num[num.len() - 1 - i] {
            return false
        }
    }
    return true
}
