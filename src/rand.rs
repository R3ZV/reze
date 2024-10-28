use std::time::{SystemTime, UNIX_EPOCH};

/// This shouldn't be used for anything serious where the random
/// distribution & security matters.
pub fn rand(end: usize) -> usize {
    let mut steps = 0;

    // Keep generating if you keep getting SystemTime errors
    loop {
        if steps == 100 {
            break 0;
        }

        let curr_time = SystemTime::now().duration_since(UNIX_EPOCH);
        if let Ok(rand) = curr_time {
            break (rand.subsec_nanos() % end as u32) as usize;
        }

        steps += 1;
    }
}
