use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod calculations;
use calculations::BpmCalculator;

fn main() {
    let event_loop = EventLoop::new();
    let _window = Window::new(&event_loop).unwrap();

    let mut bpm_calc = BpmCalculator::new();

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
                bpm_calc.capture_beat();

                let bpm = bpm_calc.average_bpm(None);

                match bpm {
                    Some(bpm) => println!("BPM {:?}", bpm),
                    None => println!("Tap again to get a measurement"),
                }
            }
        }

        _ => *control_flow = ControlFlow::Wait,
    });
}
