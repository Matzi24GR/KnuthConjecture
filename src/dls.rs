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
    let root_node = Node::new(4);
    visited.insert(root_node.value.clone());
    stack.push_front(root_node);

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
        let mut current_node: Node;
        match result {
            Some(node) => current_node = node,
            None => break Err(total_start.elapsed())
        }

        // Check node
        if current_node.value == wanted {
            if !current_node.isInteger  { current_node.operations.push(NodeType::Floor) }
            let total_duration = total_start.elapsed();
            print_result(current_node, wanted, total_duration);
            break Ok(total_start.elapsed());
        }

        if current_node.operations.len()-1 < max_depth {
            // Insert child nodes to stack
            insert_factorial(&current_node, &mut visited, &mut stack);
            insert_square_root(&current_node, &mut visited, &mut stack);
        }

        // Debug
        if SHOW_QUEUE_STACK_EVERY_STEP {println!("{:?}", stack);}
        let duration = start.elapsed();
        if SHOW_TIME_EVERY_STEP { println!("Passed Node {node_count}, took {:?}", duration); }
    }
}

fn insert_factorial(node: &Node, visited: &mut HashSet<Integer>, stack: &mut VecDeque<Node>) {
    if node.value < FACTORIAL_LIMIT {
        let x = node.value.to_u32().expect("Number too big for factorial");
        let value = Integer::factorial(x).complete();
        if !visited.contains(&value) {
            visited.insert(value.clone());
            stack.push_front(node.new_child(value, NodeType::Factorial))
        }

    }
}

fn insert_square_root(node: &Node, visited: &mut HashSet<Integer>, stack: &mut VecDeque<Node>) {
    let value = node.value.clone().sqrt();
    if !visited.contains(&value) {
        visited.insert(value.clone());
        stack.push_front(node.new_child(value, NodeType::SquareRoot))
    }
}