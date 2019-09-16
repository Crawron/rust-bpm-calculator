use std::time::Instant;

use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod calculations;
use calculations::{get_average_duration, get_instant_deltas, to_bpm};

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

                let d = get_instant_deltas(&beat_times);
                if let Some(durations) = d {
                    println!("BPM {:?}", to_bpm(get_average_duration(durations)));
                } else {
                    println!("Tap again to get a measurement");
                }
            }
        }

        _ => *control_flow = ControlFlow::Wait,
    });
}
