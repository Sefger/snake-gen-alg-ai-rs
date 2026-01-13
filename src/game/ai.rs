use rand::prelude::*;
use std::collections::LinkedList;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::config::*;
use crate::game::snake::Direction;

#[derive(Clone)]
pub struct Brain {
    // Матрица весов: ih(input-hidden), ho (hidden-output)
    // hh - h1-h2 промежуточный слой
    weight_ih: Vec<Vec<f32>>,
    weight_hh: Vec<Vec<f32>>,
    weight_ho: Vec<Vec<f32>>,
}

impl Brain {
    /// Создание мозга со случайными весами
    pub fn new(inputs: usize, hidden1: usize,hidden2:usize,  outputs: usize) -> Self {
        let mut rng = rand::rng();

        let weight_ih = (0..hidden1)
            .map(|_| (0..inputs).map(|_| rng.random_range(-1.0..1.0)).collect())
            .collect();
        let weight_hh = (0..hidden2)
            .map(|_| (0..hidden1).map(|_| rng.random_range(-1.0..1.0)).collect())
            .collect();
        let weight_ho = (0..outputs)
            .map(|_| (0..hidden2).map(|_| rng.random_range(-1.0..1.0)).collect())
            .collect();
        Brain { weight_ih, weight_hh,weight_ho }
    }

    /// Прогон данных через нейросеть
    pub fn forward(&self, inputs: &Vec<f32>) -> Vec<f32> {
        // Высисляем скрытый слой
        // Hidden = tanh(Weights_ih * Inputs)
        let mut h1 = vec![0.0; self.weight_ih.len()];

        for i in 0..self.weight_ih.len() {
            for j in 0..inputs.len() {
                h1[i] += inputs[j] * self.weight_ih[i][j];
            }
            h1[i] = h1[i].tanh();
        }

        let mut h2 = vec![0.0; self.weight_hh.len()];
        for i in 0..self.weight_hh.len() {
            for j in 0..inputs.len() {
                h2[i] += h1[j] * self.weight_hh[i][j];
            }
            h2[i] = h2[i].tanh();
        }

        // Вычисляем выходной слой: Output = Weights_ho * Hidden
        let mut outputs = vec![0.0; self.weight_ho.len()];
        for i in 0..self.weight_ho.len() {
            for j in 0..h2.len() {
                outputs[i] += h2[j] * self.weight_ho[i][j];
            }
        }
        outputs
    }

    /// Сбор данных из мира ("зрение")
    pub fn get_inputs(
        head: (i32, i32),
        apple: (i32, i32),
        body:&LinkedList<(i32, i32)>,
    ) -> Vec<f32> {
        let mut inputs = vec![0.0; 12];
        let max_coord = (GRID_SIZE - 1) as f32;
        // Расстояние до стен (нормализованные от 0 до 1)

        inputs[0] = head.1 as f32 / max_coord; // До верха
        inputs[1] = (max_coord - head.1 as f32) / max_coord; // До низа
        inputs[2] = head.0 as f32 / max_coord; //До левого края
        inputs[3] = (max_coord - head.0 as f32) / max_coord; // До нижнего края

        // Относительное положение яблока
        // ИИ должен понимать куда ему ползти
        inputs[4] = if apple.1 < head.1 { 1.0 } else { 0.0 };
        inputs[5] = 1.0 - inputs[4];
        inputs[6] = if apple.0 < head.0 { 1.0 } else { 0.0 };
        inputs[7] = 1.0 - inputs[6];
        inputs[8] = if Self::is_unsafe((head.0, head.1 - 1), body) { 1.0 } else { 0.0 };
        inputs[9] = if Self::is_unsafe((head.0, head.1 + 1), body) { 1.0 } else { 0.0 };
        inputs[10] = if Self::is_unsafe((head.0 - 1, head.1), body) { 1.0 } else { 0.0 };
        inputs[11] = if Self::is_unsafe((head.0 + 1, head.1), body) { 1.0 } else { 0.0 };
        inputs
    }
    fn is_unsafe(pos:(i32, i32), body:&LinkedList<(i32, i32)>)->bool{
        if pos.0<0|| pos.0>=GRID_SIZE||pos.1<0||pos.1>=GRID_SIZE{
            return true;
        }
        body.iter().any(|&p| p == pos)
    }

    pub fn decide(&self, inputs: &Vec<f32>) -> Direction {
        let out = self.forward(inputs);
        let max_idx = out.iter().enumerate().max_by(
            |a, b| a.1.partial_cmp(b.1).unwrap()
        ).unwrap().0;

        match max_idx {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right
        }
    }
    pub fn mutate(&mut self, rate: f32) {
        let mut rng = rand::rng();
        let mut m = |v: &mut f32| if rng.random_bool(rate as f64) {
            if rng.random_bool(0.1){
                *v = rng.random_range(-1.0..1.0);
            }
            *v += rng.random_range(-0.2..0.2);
        };
        for r in &mut self.weight_ih { r.iter_mut().for_each(&mut m) }
        for r in &mut self.weight_hh { r.iter_mut().for_each(&mut m) }
        for r in &mut self.weight_ho { r.iter_mut().for_each(&mut m) }
    }

    pub fn render_vis(
        &self,
        gl: &mut GlGraphics,
        args: &RenderArgs,
        inputs: &[f32],
        offset_x: f64, // Добавлен четвертый аргумент
    ) {

        use graphics::*;

        let start_x = offset_x + 50.0; // Используем offset_x для смещения

        let layer_gap = 150.0;
        let node_gap = 12.0;
        let layers = [INPUTS, H1,H2, OUTPUT];
        let max_layer_height = layers.iter().max().unwrap_or(&0);
        let total_max_height = (*max_layer_height as f64)*node_gap;

        // 1. Считаем активации
        let h1 = self.calculate_layer(inputs, &self.weight_ih);
        let h2 = self.calculate_layer(&h1, &self.weight_hh);
        let out = self.calculate_layer(&h2, &self.weight_ho);
        let acts = [inputs.to_vec(), h1, h2, out];

        // 2. Координаты узлов
        let mut positions = Vec::new();
        for (l_idx, &size) in layers.iter().enumerate() {
            let mut layer_pos = Vec::new();
            let layer_height = (size as f64)*node_gap;
            let v_offset = (total_max_height - layer_height)/2.0+10.0;
            for i in 0..size {
                layer_pos.push([
                    start_x + l_idx as f64 * layer_gap,
                    v_offset + i as f64 * node_gap
                ]);
            }
            positions.push(layer_pos);
        }

        gl.draw(args.viewport(), |c, gl| {
            // 3. Рисуем связи
            self.draw_layer_links(gl, &c, &positions[0], &positions[1], &self.weight_ih);
            self.draw_layer_links(gl, &c, &positions[1], &positions[2], &self.weight_hh);
            self.draw_layer_links(gl, &c, &positions[2], &positions[3], &self.weight_ho);

            // 4. Рисуем нейроны
            for (l, layer) in positions.iter().enumerate() {
                for (n, pos) in layer.iter().enumerate() {
                    let a = acts[l][n];
                    let intensity = 0.3 + (a.abs() * 0.7) as f32;
                    let color = if a > 0.0 { [0.0, intensity, 0.5, 1.0] } else { [intensity, 0.0, 0.2, 1.0] };
                    ellipse(color, ellipse::circle(pos[0], pos[1], 4.0), c.transform, gl);
                }
            }
        });
    }

    fn calculate_layer(&self, inputs: &[f32], weights: &Vec<Vec<f32>>) -> Vec<f32> {
        let mut output = vec![0.0; weights.len()];
        for i in 0..weights.len() {
            for j in 0..inputs.len() {
                output[i] += inputs[j] * weights[i][j];
            }
            output[i] = output[i].tanh();
        }
        output
    }

    fn draw_layer_links(
        &self,
        gl:&mut opengl_graphics::GlGraphics,
        c:&graphics::Context,
        p1: &[[f64;2]],
        p2: &[[f64;2]],
        weights:&Vec<Vec<f32>>
    ){
        use graphics::*;
        for (j, end) in p2.iter().enumerate(){
            for (i, start) in p1.iter().enumerate(){
                let w = weights[j][i];
                if w.abs()<0.1{
                    continue
                }
                let alpha = (w.abs()*0.2) as f32;
                let color = if w>0.0{
                    [0.2, 0.2, 1.0, alpha]
                }else{
                    [1.0, 0.2, 0.2, alpha]
                };

                line(color, 0.5, [start[0], start[1], end[0], end[1]], c.transform, gl)
            }
        }
    }
}