use std::time::{Duration, Instant};

pub fn get_instant_deltas(times: &Vec<Instant>) -> Option<Vec<Duration>> {
    if times.len() < 2 {
        return None;
    }

    let mut durations: Vec<Duration> = Vec::new();

    for i in 1..times.len() {
        durations.push(times[i].duration_since(times[i - 1]))
    }

    return Some(durations);
}

pub fn get_average_duration(durations: Vec<Duration>) -> Duration {
    let mut total: Duration = Duration::new(0, 0);

    for duration in &durations {
        total += *duration;
    }

    return total / durations.len() as u32;
}

pub fn to_bpm(duration: Duration) -> f64 {
    let secs = duration.as_secs() as f64;
    let nanos = duration.subsec_nanos() as f64;
    let duration_float: f64 = secs + nanos / 1e+9;

    return 60f64 / duration_float;
}
