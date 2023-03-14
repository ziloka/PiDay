mod calc;

use crate::calc::pi_calculator::{Task, DEFAULT_PRECISION};
use rug::Float;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("multithreaded-pi-calculations")
        .version("0.0.1")
        .arg(Arg::new("threads").short('t').long("threads").value_parser(clap::value_parser!(usize)))
        .arg(Arg::new("elements").short('e').long("elements").value_parser(clap::value_parser!(usize)))
        .get_matches();

    static DEFAULT_THREADS: usize = 2;
    static DEFAULT_ELEMENTS: usize = 10000;

    let thread_count: usize = match matches.get_one::<usize>("threads") {
        Some(value) => *value,
        None => DEFAULT_THREADS,
    };

    let amount_of_elements: usize = match matches.get_one::<usize>("elements") {
        Some(value) => *value,
        None => DEFAULT_ELEMENTS,
    };

    println!(
        "Starting the program with {} elements to be calculated on {} threads",
        amount_of_elements, thread_count
    );

    let mut result = Float::with_val(DEFAULT_PRECISION, 0);
    let mut task_tuples: VecDeque<(u32, u32)> = VecDeque::new();
    let amount_divisor: usize = 2;
    let amount_of_work_per_thread = (amount_of_elements / thread_count) / amount_divisor;

    for start_task in (0..amount_of_elements).step_by(amount_of_work_per_thread) {
        let end_index: u32 = {
            if start_task + amount_of_work_per_thread < amount_of_elements {
                start_task + amount_of_work_per_thread - 1
            } else {
                amount_of_elements - 1
            }
        } as u32;

        task_tuples.push_back((start_task as u32, end_index))
    }
    let mut receivers: Vec<Receiver<Float>> = vec![];
    let mut calculated_elements: u32 = 0;
    for current_thread in 0..thread_count {
        let (sender, receiver) = mpsc::channel();
        let (start, end) = task_tuples.pop_front().unwrap();
        let mut current_task = Task::new(start, end, sender);
        thread::spawn(move || {
            println!("Thread-{} has started working", current_thread);
            current_task.work();
        });
        receivers.push(receiver);
    }
    while calculated_elements < (amount_of_elements as u32) {
        for current_thread in 0..thread_count {
            match receivers[current_thread].try_recv() {
                Ok(result_from_thread) => {
                    println!("Thread-{} has stoped working", current_thread);
                    result += result_from_thread.clone();
                    calculated_elements += amount_of_work_per_thread as u32;
                    let (sender, receiver) = mpsc::channel();
                    match task_tuples.pop_front() {
                        Some((start, end)) => {
                            let mut current_task = Task::new(start, end, sender);
                            thread::spawn(move || {
                                println!("Thread-{} has started working", current_thread);
                                current_task.work();
                            });
                            receivers[current_thread] = receiver;
                        }
                        None => {
                            break;
                        }
                    };
                }
                Err(_) => continue,
            };
        }
    }
    let final_result: Float = Float::with_val(DEFAULT_PRECISION, 1) / result;

    println!("Pi is {}", final_result);
}

// use rug::{Float, Assign, ops::Pow};

// trait Factorial {
//     fn factorial(&self) -> Float;
// }

// impl Factorial for Float {
//     // fn factorial(&self) -> Float {
//     //     let num = self.to_integer().unwrap();
//     //     match num {
//     //         Integer(0) => Float::with_val(1, 1),
//     //         1 => Float::with_val(1, 1),
//     //         _ => (num - 1).factorial() * num,
//     //     }
//     //     // (1..=self.to_i32_saturating().unwrap()).product()
//     // }
// }

// fn chudnovsky(n: u32) -> Float {
//     let mut pi = Float::with_val(n, 0);
//     let mut a = Float::with_val(n, 13591409);
//     let mut b = Float::with_val(n, 545140134);
//     let mut c = Float::with_val(n, -640320);
//     let mut s = Float::with_val(n, 0);
//     let mut k = Float::with_val(n, 0);

//     while k < n {
//         let f1 = Float::with_val(n, (-1).pow(k));
//         let f2 = Float::with_val(n, (6*k).factorial());
//         let f3 = a + (b * k);
//         let numerator = f1 * f2 * f3;
//         let f4 = Float::with_val(n, (3*k).factorial());
//         let f5 = Float::with_val(n, k.factorial().pow(3));
//         let f6 = &c.pow(3*(k)+1.5);
//         let denominator = f4 * f5 * f6;
//         s += &numerator / &denominator;
//         k += 1;
//     }

//     let f7 = &c * Float::with_val(n, 10005).sqrt();
//     pi = &f7 / (Float::with_val(n, 12) * &s);
//     pi.assign(n);

//     pi
// }

// fn main() {
//     println!("{}", chudnovsky(50));
// }
