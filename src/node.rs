use std::time::Duration;
use rug::Integer;

use crate::SHOW_STEPS;

#[derive(Debug)]
#[derive(Clone)]
pub enum NodeType {
    Factorial,
    SquareRoot,
    InitialNumber,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub value: Integer,
    pub operations: Vec<NodeType>, 
}

pub fn print_result(v: Node, wanted: u32, total_duration: Duration) {
    print!("Found {wanted} after {:?} at depth {}", total_duration, v.operations.len()-1);
    if SHOW_STEPS {
        print!(" | ");
        for operation in v.operations {
            print_node(&operation);
            print!(" -> ");
        }
        print!("{wanted}");
    }
    println!();
}

fn print_node(operation: &NodeType) {
    let string = match operation {
        NodeType::InitialNumber => 4.to_string(),
        NodeType::Factorial => "Factorial".to_string(),
        NodeType::SquareRoot => "Square Root".to_string(),
    };
    print!("{string}");
}