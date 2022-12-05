use std::time::Instant;
use std::collections::{HashSet, VecDeque};
use rug::{Integer, Complete};

use crate::common::{Node, NodeType, print_result};
use crate::*;

pub fn find_number(wanted: u32) -> Result<u32, &'static str> {

    // Declarations
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<Integer> = HashSet::new();

    // Push Initial Node
    let node = Node{value: Integer::from(4), operations: vec![NodeType::InitialNumber]};
    visited.insert(node.value.clone());
    queue.push_back(node);

    // Debug
    let total_start = Instant::now();
    if SHOW_QUEUE_EVERY_STEP {println!("{:?}", queue);}
    let mut node_count = 0; 

    loop {
        // Debug
        node_count+=1;
        let start = Instant::now();

        // Get next node to check from queue
        let result = queue.pop_front();
        let v: Node;
        match result {
            Some(node) => v = node,
            None => break Err("NOT FOUND")
        }

        // Check node
        if v.value == wanted {
            let total_duration = total_start.elapsed();
            print_result(v, wanted, total_duration);
            break Ok(wanted);
        }

        // Insert child nodes to queue
        insert_square_root(&v, &mut visited, &mut queue);
        insert_factorial(&v, &mut visited, &mut queue);
        
        // Debug
        if SHOW_QUEUE_EVERY_STEP {println!("{:?}", queue);} 
        let duration = start.elapsed();
        if SHOW_TIME_EVERY_STEP { println!("Passed Node {node_count}, took {:?}", duration); }
    }
}

fn insert_factorial(v: &Node, visited: &mut HashSet<Integer>, queue: &mut VecDeque<Node>) {
    if v.value < FACTORIAL_LIMIT {
        let x = v.value.to_u32().expect("Number to big for factorial");
        let value = Integer::factorial(x).complete();
        if !visited.contains(&value) {
            visited.insert(value.clone());
            push_node(v, value, NodeType::Factorial, queue)
        }
        
    }
}

fn insert_square_root(v: &Node, visited: &mut HashSet<Integer>, queue: &mut VecDeque<Node>) {
    let value = v.value.clone().sqrt();
    if !visited.contains(&value) {
        visited.insert(value.clone());
        push_node(v, value, NodeType::SquareRoot, queue)
    }
}

fn push_node(parent_node: &Node, value: Integer, operation: NodeType, queue: &mut VecDeque<Node>) {
    let mut operations = parent_node.operations.clone();
    operations.push(operation);
    queue.push_back(Node{value, operations});
}