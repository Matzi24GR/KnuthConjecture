use std::time::Duration;
use crate::{dls, SHOW_DEPTH};
use crate::node::Node;

// Iterative Deepening Depth First Search
pub fn find_number(wanted: u32) -> Result<(Node, Duration), &'static str> {
    let mut depth = 0;
    let mut total_duration = Duration::new(0, 0);
    loop {
        if SHOW_DEPTH {println!("Depth {depth}")};
        let result = dls::find_number(wanted, depth);
        match result {
            Ok(value) => {
                total_duration = total_duration.saturating_add(value.1);
                break Ok((value.0, total_duration))
            },
            Err(duration) => {
                total_duration = total_duration.saturating_add(duration);
            }
        }
        depth+=1;
    }
}

#[cfg(test)]
mod tests {
    use rug::{Complete, Integer};
    use crate::iddfs;
    use crate::node::NodeType;
    use super::*;
    #[test]
    fn verifyIDDFS() {
        let wanted = 7;
        let (result_node, _) = iddfs::find_number(wanted).expect("Couldn't find number");
        let operations = result_node.operations;
        let mut number: Integer = Default::default();
        for operation in operations {
            number = match operation {
                NodeType::Factorial => Integer::factorial(number.to_u32().expect("Error")).complete(),
                NodeType::SquareRoot => number.sqrt(),
                NodeType::Floor => number,
                _ => Integer::from(4)
            };
            //operation.print();
            println!("{:.90}", number.to_string());
        }
    }
}