use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;
use crate::ai::{Brain, Evolution};
use crate::game::Game;
use crate::config::*;

pub struct Trainer{
    pub evolution: Evolution,
    pub current_game: Game,
    pub current_agent_idx: usize,
    pub scores:Vec<(usize,f32)>,
    pub best_brain:Option<Brain>,
    pub opengl: OpenGL
}

impl Trainer{
    pub fn new(opengl: OpenGL)->Self{
        let evolution = Evolution::new(POPULATION_SIZE);
        let current_game = Game::create_game(opengl, evolution.current_generation[0].clone());

        Self{
            evolution,
            current_game,
            current_agent_idx:0,
            scores:Vec::new(),
            best_brain:None,
            opengl,
        }
    }

    pub fn update(&mut self){
        if !self.current_game.is_game_over{
            self.current_game.update_ai();
        }else{
            self.next_agent()
        }
    }
    fn next_agent(&mut self){
        let fitness = Evolution::calculate_fitness(self.current_game.snake.score, self.current_game.snake.lifetime);
        self.scores.push((self.current_agent_idx, fitness));

        self.current_agent_idx+=1;
        if self.current_agent_idx<POPULATION_SIZE{
            self.current_game = Game::create_game(self.opengl, self.evolution.current_generation[self.current_agent_idx].clone());
        }
        else{
            self.evolve_population()
        }
    }

    fn evolve_population(&mut self){
        if let Some((best_idx, _)) = self.scores.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()){
            self.best_brain = Some(self.evolution.current_generation[*best_idx].clone());
        }

        self.print_stats();

        self.evolution.breed(self.scores.clone());
        self.scores.clear();
        self.current_agent_idx = 0;
        self.current_game = Game::create_game(self.opengl, self.evolution.current_generation[0].clone());
    }

    pub fn render(&mut self, r:&RenderArgs){
        self.current_game.render(r);

        //Рисуем мозр лучшего из прошлого поколения
        if let Some(ref brain) = self.best_brain{
            let head = *self.current_game.snake.body.front().unwrap();
            let inputs = Brain::get_inputs(head,(self.current_game.apple.pos_x, self.current_game.apple.pos_y), &self.current_game.snake.body);
            brain.render_vis(&mut self.current_game.gl, r, &inputs, (WINDOW_WIDTH/3) as f64);
        }
    }

    pub fn handle_input(&mut self, btn: &piston::input::Button){
        self.current_game.pressed(btn);
    }

    fn print_stats(&self){
        let best = self.scores.iter().map(|s| s.1).fold(f32::MIN, |a,b| a.max(b));
        let avg = self.scores.iter().map(|s| s.1).sum::<f32>()/(self.scores.len() as f32);

        println!("Поколение {}. Лучший: {:.2} \nСреднее: {:.2}\n", self.evolution.generation_number, best, avg);
    }
}