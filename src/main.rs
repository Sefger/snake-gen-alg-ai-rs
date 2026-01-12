mod game;

use crate::game::{Game, Snake, Apple, Direction, Evolution, Brain};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::OpenGLWindow;
use std::collections::LinkedList;


// Для создания игры
fn create_game(opengl: OpenGL, brain: Brain) -> Game {
    Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            // Начальная позиция подальше от стен
            body: LinkedList::from_iter(vec![(5, 5), (4, 5), (3, 5)]),
            dir: Direction::Right,
            score: 0,
            lifetime: 0,
            dir_locked: false,
            brain,
        },
        apple: Apple { pos_x: 10, pos_y: 10 },
        is_game_over: false,
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake AI Training",
        [1200, 400],
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    let mut evolution = Evolution::new(350);
    let mut current_agent_idx = 0;
    let mut scores = Vec::new();
    let mut game = create_game(opengl, evolution.current_generation[0].clone());

    let mut best_brain_of_prev_gen:Option<Brain> = None;

    let mut events = Events::new(EventSettings::new()).ups(200);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
            if let Some(ref best_brain) = best_brain_of_prev_gen {
                let head = *game.snake.body.front().unwrap();
                let inputs = Brain::get_inputs(head, (game.apple.pos_x, game.apple.pos_y), &game.snake.body, 20);
                best_brain.render_vis(&mut game.gl, &r, &inputs, 400.0);
            }
        }
        if let Some(_) = e.update_args() {
            if !game.is_game_over {
                game.update_ai();
            }else{
                let fitness = calculate_fitness(game.snake.score, game.snake.lifetime);
                scores.push((current_agent_idx, fitness));

                current_agent_idx += 1;

                if current_agent_idx<evolution.population_size{
                    game = create_game(opengl, evolution.current_generation[current_agent_idx].clone());
                }
                else{
                    if let Some((best_idx, _)) = scores.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
                        // Сохраняем мозг именно этого чемпиона
                        best_brain_of_prev_gen = Some(evolution.current_generation[*best_idx].clone());
                    }

                    let best_fitness = scores.iter().map(|s| s.1).fold(f32::MIN, |a, b| a.max(b));
                    let worst_fitness = scores.iter().map(|s| s.1).fold(f32::MAX, |a, b| a.min(b));
                    println!("Поколение {}. Лучший: {:.2}\nХудший: {:.2}",
                             evolution.generation_number, best_fitness, worst_fitness);
                    evolution.breed(scores.clone());

                    scores.clear();
                    current_agent_idx = 0;
                    game = create_game(opengl, evolution.current_generation[0].clone())
                }
            }
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}

pub fn calculate_fitness(score: i32, lifetime: u32) -> f32 {
    // 1. Очки за выживание
    let survival_bonus = lifetime as f32 * 0.01;

    // 2. Большой бонус за яблоко
    let apple_bonus = (score * score * 500) as f32;

    // 3. Штраф за быструю смерть
    let death_penalty = if score == 0 && lifetime < 20 { -100.0 } else { 0.0 };

    survival_bonus + apple_bonus + death_penalty
}
