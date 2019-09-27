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
                let last_ten = bpm_calc.average_bpm(Some(10));
                let last_one = bpm_calc.average_bpm(Some(1));

                match (bpm, last_ten, last_one) {
                    (Some(bpm), Some(last_ten), Some(last_one)) => println!(
                        "\nBPM\nTotal:\t\t{:.1}\nLast 10:\t{:.1}\nLast one:\t{:.1}",
                        bpm, last_ten, last_one
                    ),
                    _ => println!("Tap again to get a measurement"),
                }
            }
        }

        _ => *control_flow = ControlFlow::Wait,
    });
}
