use std::time::Duration;
use crate::dls;

// Iterative Deepening Depth First Search
pub fn find_number(wanted: u32) -> Result<u32, &'static str> {
    let mut depth = 0;
    let mut total_duration = Duration::new(0, 0);
    loop {
        println!("Depth {depth}");
        let result = dls::find_number(wanted, depth);
        match result {
            Ok(duration) => {
                total_duration = total_duration.saturating_add(duration);
                print!("Really took {:?}s", total_duration);
                break Ok(wanted)
            },
            Err(duration) => {
                total_duration = total_duration.saturating_add(duration);
            }
        }
        depth+=1;
    }
}