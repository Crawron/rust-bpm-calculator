use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use std::time::{Duration, Instant};

fn main() {
    let event_loop = EventLoop::new();

    let _window = Window::new(&event_loop).unwrap();

    let mut beat_times: Vec<Instant> = Vec::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,

        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => {
            if input.state == ElementState::Pressed {
                let now = Instant::now();
                beat_times.push(now);

                let durations = get_duration_values(&beat_times);
                if let Some(durations) = durations {
                    println!("BPM {:?}", to_bpm(get_average_duration(durations)));
                } else {
                    println!("Tap again to get a measurement");
                }
            }
        }

        _ => *control_flow = ControlFlow::Wait,
    });
}

fn get_duration_values(times: &Vec<Instant>) -> Option<Vec<Duration>> {
    if times.len() < 2 {
        return None;
    }

    let mut durations: Vec<Duration> = Vec::new();

    for i in 1..times.len() {
        durations.push(times[i].duration_since(times[i - 1]))
    }

    return Some(durations);
}

fn get_average_duration(durations: Vec<Duration>) -> Duration {
    let mut total: Duration = Duration::new(0, 0);

    for duration in &durations {
        total += *duration;
    }

    return total / durations.len() as u32;
}

fn to_bpm(duration: Duration) -> f64 {
    let secs = duration.as_secs() as f64;
    let millis = duration.subsec_millis() as f64;
    let duration_float: f64 = secs + millis / 1000f64;

    return 60f64 / duration_float;
}
