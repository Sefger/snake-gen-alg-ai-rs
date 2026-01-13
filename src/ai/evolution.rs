use rand::Rng;
use crate::Brain;
use crate::config::*;
pub struct Evolution {
    pub current_generation: Vec<Brain>,
    pub generation_number: u32,
    pub population_size: usize,
}

impl Evolution {
    pub fn new(size: usize) -> Self {
        let mut brains = Vec::new();
        for _ in 0..size {
            //12 входов, 16 скрытых, 4 выхода
            brains.push(Brain::new(INPUTS, H1, H2, OUTPUT));
        }
        Evolution {
            current_generation: brains,
            generation_number: 1,
            population_size: size,
        }
    }

    /// Создаём след поколение на основе "фитнеса" (очков) предыдущего
    pub fn breed(&mut self, scores: Vec<(usize, f32)>) {
        let mut scores = scores;
        //Сортируем: лучшие в начале списка
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut next_gen = Vec::new();

        // 1. "Элитизм" Копируем 2 лучших без изменений
        for i in 0..15 {
            next_gen.push(self.current_generation[scores[i].0].clone());
        }

        // 2. Заполняем остальную популяцию потомками лучших
        while next_gen.len() < self.population_size {
            // Берём случайную из пяти лучших
            let parent_idx = scores[rand::rng().random_range(0..30)].0;
            let mut child = self.current_generation[parent_idx].clone();

            //Мутируем её гены
            child.mutate(MUTATION_RATE);
            next_gen.push(child)
        }
        self.current_generation = next_gen;
        self.generation_number += 1;
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
}