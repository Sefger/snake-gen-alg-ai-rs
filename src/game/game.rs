use std::collections::LinkedList;
use piston::input::*;

use opengl_graphics::{GlGraphics};
use crate::config::COLOR_BLACK;
use crate::game::{Direction, Snake};
use crate::game::Apple;
use crate::traits::Drawable;

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub apple: Apple,
    pub is_game_over: bool,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        use graphics;


        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(COLOR_BLACK, gl);
        });
        self.snake.render(&mut self.gl, arg);

        self.apple.render(&mut self.gl, arg);
    }
    #[allow(dead_code)]
    pub fn update(&mut self, apple_pos: (i32, i32)) {
        if self.check_collision() {
            self.is_game_over = true;
            return;
        }
        if self.is_game_over {
            return;
        }

        if self.snake.update(apple_pos) {
            self.apple.update_chord(&self.snake.body)
        }
    }
    pub fn pressed(&mut self, btn: &Button) {
        if let &Button::Keyboard(Key::G)= btn{
            if self.is_game_over{
                self.restart();
                return;
            }
        }
        if self.is_game_over|| self.snake.dir_locked{
            return;
        }
        let last_direction = self.snake.dir.clone();

        let new_dir = match btn {
            &Button::Keyboard(Key::Up)
            if last_direction != Direction::Down => Some(Direction::Up),

            &Button::Keyboard(Key::Down)
            if last_direction != Direction::Up => Some(Direction::Down),

            &Button::Keyboard(Key::Left)
            if last_direction != Direction::Right => Some(Direction::Left),

            &Button::Keyboard(Key::Right)
            if last_direction != Direction::Left => Some(Direction::Right),
            _ =>None
        };
        if let Some(dir) = new_dir{
            if dir != last_direction{
                self.snake.dir = dir;
                self.snake.dir_locked = true;
            }

        }
    }
    pub fn check_collision(&mut self) -> bool {
        let head = *self.snake.body.front().unwrap();
        if head.0 < 0 || head.0 >= 20 || head.1 < 0 || head.1 >= 20 {
            return true;
        }
        let body_len = self.snake.body.len();
        if body_len>3 {
            return self.snake.body.iter()
                .skip(1)
                .any(|&node| node.0 == head.0 && node.1 == head.1);
        }
        false
    }
    fn restart(&mut self){
        self.snake.body = LinkedList::from_iter(vec![(2, 0), (1, 0), (0, 0)]);
        self.snake.dir = Direction::Right;
        self.snake.score = 0;
        self.is_game_over = false;
        self.apple.update_chord(&self.snake.body);
    }

    // Для ии
    pub fn update_ai(&mut self){
        if self.is_game_over{
            return;
        }

        // 1. Изучаем мир
        let head = *self.snake.body.front().unwrap();
        let inputs = crate::game::ai::Brain::get_inputs(
            head,
            (self.apple.pos_x, self.apple.pos_y),
            &self.snake.body,

        );

        // 2. ИИ принимает решение
        let suggested_dir = self.snake.brain.decide(&inputs);
        // 3. Защита
        let is_opposite = match (&self.snake.dir, &suggested_dir) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        };
        if !is_opposite{
            self.snake.dir = suggested_dir;
        }

        // 4. Двигаемся
        let ate_apple = self.snake.update((self.apple.pos_x, self.apple.pos_y));

        // 5. Проверяем коллизии
        if self.check_collision(){
            self.is_game_over = true;
            return;
        }
        if ate_apple{
            self.apple.update_chord(&self.snake.body);
        }

        if self.snake.lifetime > (self.snake.score as u32 + 1) * 150 {
            self.is_game_over = true;
        }


    }
}