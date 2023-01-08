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
    pub fn print(self) {
        let string = match self {
            NodeType::InitialNumber => 4.to_string(),
            NodeType::Factorial => "Factorial".to_string(),
            NodeType::SquareRoot => "Square Root".to_string(),
            NodeType::Floor => "Floor".to_string(),
        };
        print!("{string}");
    }
    pub fn is_floor(&self) -> bool {
        match *self {
            NodeType::Floor => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub value: Integer,
    pub operations: Vec<NodeType>,
    pub is_integer: bool,
}

impl Node {
    pub fn new(value: u32) -> Self {
        Node{value: Integer::from(value), operations: vec![NodeType::InitialNumber], is_integer:true}
    }
    pub fn new_child(&self, value: Integer, operation: NodeType) -> Self {
        let mut is_integer = self.is_integer;
        let mut operations = self.operations.clone();
        match operation {
            NodeType::Factorial => {
                if !is_integer { operations.push(NodeType::Floor) }
                is_integer = true
            },
            NodeType::SquareRoot => {
                is_integer = self.value == value.clone().pow(2)
            },
            _ => {},
        }
        operations.push(operation);
        Node{value, operations, is_integer }
    }
    pub fn print(self, wanted: u32, total_duration: Duration) {
        print!("Found {wanted:<5} after {:>10.2?} at depth {:<5}", total_duration, self.operations.len()-1);
        if SHOW_STEPS {
            print!(" | ");
            for operation in self.operations {
                operation.print();
                print!(" -> ");
            }
            print!("{wanted}");
        }
        println!();
    }
}