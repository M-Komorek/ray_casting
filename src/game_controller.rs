use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};

use crate::camera::Camera;

pub struct GameController {
    camera: Camera,
    event_pump: EventPump,
    quit: bool,
}

impl GameController {
    pub fn new(sdl_context: Sdl) -> GameController {
        let camera = Camera::new(&sdl_context);

        GameController {
            camera,
            event_pump: sdl_context.event_pump().expect("event_pump init failed!"),
            quit: false,
        }
    }

    pub fn run(&mut self) {
        while !self.quit {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        self.quit = true;
                    }
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(Keycode::Left) => {
                            self.camera.rotate(7.0 * 0.016);
                        }
                        Some(Keycode::Right) => {
                            self.camera.rotate(-7.0 * 0.016);
                        }
                        Some(Keycode::Up) => {
                            self.camera.move_forward(5.0 * 0.016);
                        }
                        Some(Keycode::Down) => {
                            self.camera.move_backward(5.0 * 0.016);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            self.camera.render();
        }
    }
}
