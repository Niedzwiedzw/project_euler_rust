//A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//a^2 + b^2 = c^2
//
//For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
//There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//Find the product abc.


fn main() {
    for a in 1..1001 {
        for b in 1..(1001-a) {
            for c in 1..(1001 - a - b) {
                if a*a + b*b == c*c  && a + b + c == 1000 {
                    println!("{}, {}, {}\nproduct: {}", a, b, c, a*b*c)
                }
            }
        }
    }
}
