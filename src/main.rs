// The time complexity of this algorithm is O(n(logn)^3)

use rug::{Float, ops::Pow};

// https://en.wikipedia.org/wiki/Chudnovsky_algorithm
fn chudnovsky(n: u32) -> Float {
    let constant = Float::with_val(n, 426880) * Float::with_val(n, 10005).sqrt();

    let mut summation = Float::with_val(n, 0);

    let mut q = Float::with_val(n, 0);
    let mut linear = Float::with_val(n, 13591409);
    let mut exponential = Float::with_val(n, 1);
    let mut multinomial = Float::with_val(n, 1);
    let mut K = Float::with_val(n, -6);

    while q < Float::with_val(n, n) {
        summation += multinomial.clone() * linear.clone() / exponential.clone();
        // post work
        linear += 545140134;
        exponential *= Float::with_val(n, -262537412640768000i128);
        K += 12;
        q += 1;
        multinomial *= (K.clone().pow(3) - 16 * K.clone()) / q.clone().pow(3);
    }

    constant * summation.pow(-1)
}

fn main() {
    let pi = chudnovsky(500000);
    println!("{}", pi);
}
