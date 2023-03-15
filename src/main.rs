use std::ops::{Mul, Add};

use rug::{Float, ops::Pow};

// https://en.wikipedia.org/wiki/Chudnovsky_algorithm
fn chudnovsky(n: u32) -> Float {
    let C = Float::with_val(n, 426880).mul(Float::with_val(n, 10005).sqrt());

    let mut summation = Float::with_val(n, 0);

    let mut q = Float::with_val(n, 0);
    let mut L = Float::with_val(n, 13591409);
    let mut X = Float::with_val(n, 1);
    let mut M = Float::with_val(n, 1);
    let mut K = Float::with_val(n, -6);

    while q < Float::with_val(n, n) {
        summation += M.clone() * L.clone() / X.clone();
        // post work
        L = L.add(545140134);
        X = X * Float::with_val(n, -262537412640768000i128);
        K = K.add(12);
        q += 1;
        M = M * ((K.clone().pow(3) - 16 * K.clone()) / q.clone().pow(3));
    }

    C * summation.pow(-1)
}

fn main() {
    let pi = chudnovsky(2049);
    println!("{}", pi);
}

// use std::ops::{Mul, Add};
// use num_bigint::{BigInt, Sign};
// use num_traits::One;

// // https://en.wikipedia.org/wiki/Chudnovsky_algorithm
// fn chudnovsky(n: u32) -> BigInt {
//     // 426880
//     // 10005
//     let C = BigInt::new(Sign::NoSign, vec![0, 8, 8, 6, 2, 4]).mul(BigInt::new(Sign::NoSign, vec![5, 0, 0 , 0, 1]).sqrt());

//     let mut summation = BigInt::new(Sign::NoSign, vec![0]);

//     let mut q = 0;
//     // 13591409
//     let mut L = BigInt::new(Sign::NoSign, vec![9, 0, 4, 1, 9, 5, 3, 1]);
//     let mut X: BigInt = One::one();
//     let mut M: BigInt = One::one();
//     // let mut X = BigInt::new(Sign::NoSign, vec![1]);
//     // let mut M = BigInt::new(Sign::NoSign, vec![1]);
//     let mut K = BigInt::new(Sign::Minus, vec![6]);

//     while q < n {
//         // println!("{:?}", X);
//         // println!("get here 1 M: {:.2}, L: {:.2}, X: {:.2}", &M, &L, &X);
//         println!("summation: {} result: {}", summation, X.clone());
//         summation += M.clone() * L.clone() / X.clone();
//         // post work
//         L = L.add(545140134);
//         // -262537412640768000i128
//         X = X * BigInt::new(Sign::Minus, vec![0, 0, 0, 8, 6, 7, 0, 4, 6, 2, 1, 4, 7, 3, 5, 2, 6, 2]);
//         K = K.add(12);
//         println!("get here 2");
//         M = M * ((K.clone().pow(3) - 3 * K.clone()) / (q + 1).pow(3));
//         println!("get here 3");
//         q += 1;
//     }
//     println!("exits");
//     println!("{}", summation);

//     // use of negative exponent power identity
//     C * (1 / summation.pow(1))
// }

// fn main() {
//     println!("{}", chudnovsky(50));
// }
