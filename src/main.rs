extern crate sdl2;

mod camera;
mod game_controller;
mod map;
mod vector_2d;

use game_controller::GameController;

fn main() {
    let sdl_context = sdl2::init().expect("sdl2 init failed!");
    let mut game_controller = GameController::new(sdl_context);

    game_controller.run();
}
