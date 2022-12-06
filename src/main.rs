use std::io;
use std::io::Write;
use std::env;
use std::time::Instant;

mod bfs;
mod node;
mod iddfs;
mod dls;

pub static SHOW_STEPS: bool = true;
pub static SHOW_QUEUE_STACK_EVERY_STEP: bool = false; // Warning: Large Numbers!
pub static SHOW_TIME_EVERY_STEP: bool = false;
pub static FACTORIAL_LIMIT: u32 = 100_000;

fn main() {

    let args: Vec<String> = env::args().collect();

    print!("Enter Number to search: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.contains('-') {
        let split: Vec<&str> = input.split('-').collect();
        let start_time = Instant::now();
        let start: u32 = split[0].trim().parse().expect("Failed to parse range");
        let end: u32 = split[1].trim().parse().expect("Failed to parse range");
        for i in start..end {
            iddfs::find_number(i);
        }
        let duration = start_time.elapsed();
        println!("Took {:?}", duration);
    } else {
        let input: u32 = input.trim().parse().expect("Failed to parse range");
        // bfs::find_number(input);
        iddfs::find_number(input);
    }

}
