use rug::{Float, ops::Pow};

// https://en.wikipedia.org/wiki/Chudnovsky_algorithm
// The time complexity of this algorithm is O(n(logn)^3)
// this algorithm is good for computing large numbers of digits of pi
fn chudnovsky(n: u32) -> Float {
    let constant = Float::with_val(n, 426_880) * Float::with_val(n, 10_005).sqrt();

    let mut summation = Float::with_val(n, 0);

    let mut q = Float::with_val(n, 0);
    let mut linear = Float::with_val(n, 13_591_409);
    let mut exponential = Float::with_val(n, 1);
    let mut multinomial = Float::with_val(n, 1);
    let mut K = Float::with_val(n, -6);

    while q < Float::with_val(n, n) {
        summation += multinomial.clone() * linear.clone() / exponential.clone();
        // post work
        linear += 545_140_134;
        exponential *= Float::with_val(n, -262_537_412_640_768_000i128);
        K += 12;
        q += 1;
        multinomial *= (K.clone().pow(3) - 16 * K.clone()) / q.clone().pow(3);
    }

    constant * summation.pow(-1)
}

fn main() {
    let pi = chudnovsky(20000);
    println!("{}", pi);
}
