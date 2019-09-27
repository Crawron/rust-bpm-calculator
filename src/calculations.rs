use std::cmp::min;
use std::time::{Duration, Instant};

pub struct BpmCalculator {
    last_beat: Option<Instant>,
    beat_deltas: Vec<f64>,
}

impl BpmCalculator {
    pub fn new() -> BpmCalculator {
        BpmCalculator {
            last_beat: None,
            beat_deltas: Vec::new(),
        }
    }

    pub fn capture_beat(&mut self) {
        let now = Instant::now();
        if let Some(last_beat) = self.last_beat {
            let last_beat_delta = now.duration_since(last_beat);
            self.beat_deltas.push(to_f64(last_beat_delta));
        }

        self.last_beat = Some(now);
    }

    pub fn average_bpm(&self, last: Option<usize>) -> Option<f64> {
        if self.beat_deltas.len() < 1 {
            return None;
        }

        let average = match last {
            Some(last) => {
                let safe_last = min(last, self.beat_deltas.len());
                let start = self.beat_deltas.len() - safe_last;
                get_average(&self.beat_deltas[start..])
            }
            None => get_average(&self.beat_deltas),
        };
        Some(60.0 / average)
    }
}

fn get_average(ns: &[f64]) -> f64 {
    let mut total = 0.0;

    for n in ns {
        total += *n;
    }

    return total / ns.len() as f64;
}

fn to_f64(duration: Duration) -> f64 {
    let secs = duration.as_secs() as f64;
    let nanos = duration.subsec_nanos() as f64 / 1e9;

    secs + nanos
}
