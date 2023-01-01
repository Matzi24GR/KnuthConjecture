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