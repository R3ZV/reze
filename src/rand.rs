use std::time::{SystemTime, UNIX_EPOCH};

/// This shouldn't be used for anything serious where the random
/// distribution & security matters.
///
/// `rand` generates a random number in the interval [0, end)
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

/// Fisher-Yates shuffle algorithm
pub fn shuffle<T>(v: &mut Vec<T>) {
    for i in (1..=v.len() - 1).rev() {
        let j = rand(i);
        v.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_test() {
        for n in 2..10 {
            let mut v = (1..=n).collect();
            shuffle(&mut v);

            assert!(!v.is_sorted());
        }
    }
}
