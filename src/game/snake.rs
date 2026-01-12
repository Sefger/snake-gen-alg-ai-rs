use std::collections::LinkedList;
use graphics::color::grey;
use piston::input::*;
use opengl_graphics::GlGraphics;
use crate::game::Brain;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}
pub struct Snake {
    pub(crate) body: LinkedList<(i32, i32)>,
    pub(crate) dir: Direction,
    pub(crate) score: i32,
    pub(crate) dir_locked: bool,
    pub lifetime: u32,
    pub brain: Brain
}

impl Snake {
    pub(crate) fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let grey:[f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let squares: Vec<_> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64,
                    20_f64)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for square in squares {
                // 1. Рисуем основное тело сегмента
                graphics::rectangle(blue, square, transform, gl);

                // 2. Рисуем серую обводку по краям этого сегмента
                // Параметр 1.0 — это толщина линии края
                graphics::Rectangle::new_border(grey, 1.0)
                    .draw(square, &c.draw_state, transform, gl);
            }
        });
    }
    pub fn update(&mut self, apple_pos: (i32, i32)) -> bool {
        self.lifetime+=1;
        self.dir_locked = false;

        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }
        self.body.push_front(new_head);
        if apple_pos.0 == new_head.0 && apple_pos.1 == new_head.1 {

            self.score += 1;
            return true;
        }
        self.body.pop_back();
        return false;
    }
}