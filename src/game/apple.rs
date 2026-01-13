use piston::input::*;
use opengl_graphics::{GlGraphics};
use rand::prelude::*;
use std::collections::LinkedList;
use crate::config::COLOR_APPLE;
use crate::traits::Drawable;

pub struct Apple {
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Apple {

    pub fn update_chord(&mut self, list: &LinkedList<(i32, i32)>) {
        let mut rnd = rand::rng();
        loop {
            let new_x = rnd.random_range(0..20);
            let new_y = rnd.random_range(0..20);

            if !list.iter().any(|&(x, y)| x == new_x && y == new_y) {
                self.pos_x = new_x;
                self.pos_y = new_y;
                break;
            }
        }
    }
}
impl Drawable for Apple{
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;


        let square = graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20f64);

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(COLOR_APPLE, square, transform, gl);
        })
    }
}