use std::io;
use std::io::Write;
use std::env;
use std::ops::Div;
use std::time::{Duration, Instant};
use crate::AlgorithmOption::{BFS, BOTH, IDDFS};

mod bfs;
mod node;
mod iddfs;
mod dls;

pub static SHOW_STEPS: bool = true;
pub static SHOW_DEPTH: bool = false;
pub static SHOW_QUEUE_STACK_EVERY_STEP: bool = false; // Warning: Large Numbers!
pub static SHOW_TIME_EVERY_STEP: bool = false;
pub static FACTORIAL_LIMIT: u32 = 100_000;

fn main() {

    let algorithm = choose_algorithm();
    let (start, end) = choose_number();

    let start_time = Instant::now();
    for i in start..end+1 {
        match algorithm {
            BFS => {
                let (wanted_node, total_duration) = bfs::find_number(i).expect("Couldn't find number");
                wanted_node.print(i, total_duration)
            }
            IDDFS => {
                let (wanted_node, total_duration) = bfs::find_number(i).expect("Couldn't find number");
                wanted_node.print(i, total_duration)
            }
            BOTH => {
                print!("-Using BFS:   ");
                let (wanted_node, total_duration1) = bfs::find_number(i).expect("Couldn't find number");
                wanted_node.print(i, total_duration1);
                print!("-Using IDDFS: ");
                let (wanted_node, total_duration2) = iddfs::find_number(i).expect("Couldn't find number");
                wanted_node.print(i, total_duration2);

                let difference = total_duration2.as_micros() as i128 - total_duration1.as_micros() as i128;
                let percentage = difference as f64 / total_duration1.as_micros() as f64 * 100.0;
                println!("Result: IDDFS took {:+.2}% time", percentage)
            },
        }
    }
    let duration = start_time.elapsed();
    if start!=end {println!("Program took {:.2?}", duration);}
}

enum AlgorithmOption {
    BFS,
    IDDFS,
    BOTH
}

fn choose_algorithm() -> AlgorithmOption {
    println!("1. Breadth First Search");
    println!("2. Iterative Deepening Depth First Search");
    println!("3. Both");
    print!("Enter Choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Failed to parse number");

    return match input {
        1 => BFS,
        2 => IDDFS,
        3 => BOTH,
        _ => {choose_algorithm()}
    }
}

fn choose_number() -> (u32, u32) {
    print!("Enter Number to search: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return if input.contains('-') {
        let split: Vec<&str> = input.split('-').collect();
        let start: u32 = split[0].trim().parse().expect("Failed to parse range");
        let end: u32 = split[1].trim().parse().expect("Failed to parse range");
        (start, end)
    } else {
        let input: u32 = input.trim().parse().expect("Failed to parse number");
        (input, input)
    }
}


