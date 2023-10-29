mod game;
mod render;
mod game_logic;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use std::env;
use game::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let event_loop = glutin::event_loop::EventLoop::new();
    let mut application = Game::new(&event_loop);
    
    event_loop.run(move |event, _, control_flow| {
        application.handle_event(&event);
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..}
            => {
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });
}