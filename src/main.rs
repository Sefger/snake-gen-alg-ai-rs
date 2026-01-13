mod game;
mod traits;
mod config;
mod ai;
mod trainer;

use crate::game::Game;
use crate::ai::{Brain, Evolution};
use crate::config::*;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL};
use piston::OpenGLWindow;
use rand::Rng;
use crate::trainer::Trainer;


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake AI Training",
        [WINDOW_WIDTH, WINDOW_HEIGHT],
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    let mut trainer = Trainer::new(opengl);
    let mut events = Events::new(EventSettings::new()).ups(UPS);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            trainer.render(&r);
        }
        if let Some(u) = e.update_args() {
            trainer.update();
        }
        // if let Some(k) = e.button_args() {
        //     if k.state == ButtonState::Press {
        //         trainer.handle_input(&k.button);
        //     }
        // }
    }


}


