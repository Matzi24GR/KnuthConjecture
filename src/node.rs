use std::collections::HashSet;
use std::time::Duration;
use rug::Integer;
use rug::ops::Pow;

use crate::SHOW_STEPS;

#[derive(Debug)]
#[derive(Clone)]
pub enum NodeType {
    Factorial,
    SquareRoot,
    Floor,
    InitialNumber,
}

impl NodeType {
    fn print(self) {
        let string = match self {
            NodeType::InitialNumber => 4.to_string(),
            NodeType::Factorial => "Factorial".to_string(),
            NodeType::SquareRoot => "Square Root".to_string(),
            NodeType::Floor => "Floor".to_string(),
        };
        print!("{string}");
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub value: Integer,
    pub operations: Vec<NodeType>,
    pub isInteger: bool,
}

impl Node {
    pub fn new(value: u32) -> Self {
        Node{value: Integer::from(value), operations: vec![NodeType::InitialNumber], isInteger:true}
    }
    pub fn new_child(&self, value: Integer, operation: NodeType) -> Self {
        let mut isInteger = self.isInteger;
        let mut operations = self.operations.clone();
        match operation {
            NodeType::Factorial => {
                if !isInteger { operations.push(NodeType::Floor) }
                isInteger = true
            },
            NodeType::SquareRoot => {
                isInteger = self.value == value.clone().pow(2)
            },
            _ => {},
        }
        operations.push(operation);
        Node{value, operations, isInteger}
    }
}

pub fn print_result(node: Node, wanted: u32, total_duration: Duration) {
    print!("Found {wanted} after {:?} at depth {}", total_duration, node.operations.len()-1);
    if SHOW_STEPS {
        print!(" | ");
        for operation in node.operations {
            operation.print();
            print!(" -> ");
        }
        print!("{wanted}");
    }
    println!();
}