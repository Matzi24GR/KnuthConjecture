use std::time::{Duration, Instant};
use std::collections::{HashSet, VecDeque};
use rug::{Integer, Complete};
use rug::ops::Pow;

use crate::node::{Node, NodeType};
use crate::*;

// Breadth First Search
pub fn find_number(wanted: u32) -> Result<(Node, Duration), &'static str> {

    // Declarations
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<Integer> = HashSet::new();

    // Push Initial Node
    let root_node = Node::new(4);
    visited.insert(root_node.value.clone());
    queue.push_back(root_node);

    // Debug
    let total_start = Instant::now();
    if SHOW_QUEUE_STACK_EVERY_STEP {println!("{:?}", queue);}
    let mut node_count = 0; 

    loop {
        // Debug
        node_count+=1;
        let start = Instant::now();

        // Get next node to check from queue
        let result = queue.pop_front();
        let mut current_node: Node;
        match result {
            Some(node) => current_node = node,
            None => break Err("NOT FOUND")
        }

        // Check node
        if current_node.value == wanted {
            if !current_node.is_integer { current_node.operations.push(NodeType::Floor) }
            break Ok((current_node, total_start.elapsed()));
        }

        // Insert child nodes to queue
        insert_square_root(&current_node, &mut visited, &mut queue);
        insert_factorial(&current_node, &mut visited, &mut queue);
        
        // Debug
        if SHOW_QUEUE_STACK_EVERY_STEP {println!("{:?}", queue);}
        let duration = start.elapsed();
        if SHOW_TIME_EVERY_STEP { println!("Passed Node {node_count}, took {:?}", duration); }
    }
}

fn insert_factorial(node: &Node, visited: &mut HashSet<Integer>, queue: &mut VecDeque<Node>) {
    if node.value < FACTORIAL_LIMIT {
        let x = node.value.to_u32().expect("Number too big for factorial");
        let value = Integer::factorial(x).complete();
        if !visited.contains(&value) {
            visited.insert(value.clone());
            queue.push_back(node.new_child(value, NodeType::Factorial))
        }
        
    }
}

fn insert_square_root(node: &Node, visited: &mut HashSet<Integer>, queue: &mut VecDeque<Node>) {
    let value = node.value.clone().sqrt();
    if !visited.contains(&value) {
        visited.insert(value.clone());
        queue.push_back(node.new_child(value, NodeType::SquareRoot))
    }
}