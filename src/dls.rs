use std::time::{Duration, Instant};
use std::collections::{HashSet, VecDeque};
use rug::{Integer, Complete};

use crate::node::{Node, NodeType, print_result};
use crate::*;

// Depth Limited Search
pub fn find_number(wanted: u32, max_depth: usize) -> Result<Duration, Duration> {

    // Declarations
    let mut stack: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<Integer> = HashSet::new();

    // Push Initial Node
    let node = Node{value: Integer::from(4), operations: vec![NodeType::InitialNumber]};
    visited.insert(node.value.clone());
    stack.push_front(node);

    // Debug
    let total_start = Instant::now();
    if SHOW_QUEUE_STACK_EVERY_STEP {println!("{:?}", stack);}
    let mut node_count = 0;

    loop {

        // Debug
        node_count+=1;
        let start = Instant::now();

        // Get next node to check from stack
        let result = stack.pop_front();
        let v: Node;
        match result {
            Some(node) => v = node,
            None => break Err(total_start.elapsed())
        }

        // Check node
        if v.value == wanted {
            let total_duration = total_start.elapsed();
            print_result(v, wanted, total_duration);
            break Ok(total_start.elapsed());
        }

        if v.operations.len()-1 < max_depth {
            // Insert child nodes to stack
            insert_factorial(&v, &mut visited, &mut stack);
            insert_square_root(&v, &mut visited, &mut stack);
        }

        // Debug
        if SHOW_QUEUE_STACK_EVERY_STEP {println!("{:?}", stack);}
        let duration = start.elapsed();
        if SHOW_TIME_EVERY_STEP { println!("Passed Node {node_count}, took {:?}", duration); }
    }
}

fn insert_factorial(v: &Node, visited: &mut HashSet<Integer>, stack: &mut VecDeque<Node>) {
    if v.value < FACTORIAL_LIMIT {
        let x = v.value.to_u32().expect("Number to big for factorial");
        let value = Integer::factorial(x).complete();
        if !visited.contains(&value) {
            //visited.insert(value.clone());
            push_node(v, value, NodeType::Factorial, stack)
        }

    }
}

fn insert_square_root(v: &Node, visited: &mut HashSet<Integer>, stack: &mut VecDeque<Node>) {
    let value = v.value.clone().sqrt();
    if !visited.contains(&value) {
        //visited.insert(value.clone());
        push_node(v, value, NodeType::SquareRoot, stack)
    }
}

fn push_node(parent_node: &Node, value: Integer, operation: NodeType, stack: &mut VecDeque<Node>) {
    let mut operations = parent_node.operations.clone();
    operations.push(operation);
    stack.push_front(Node{value, operations});
}