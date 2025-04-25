use std::cell::RefCell;
use std::rc::Rc;

use crate::layers::{Layer, LayerState};
use crate::render::RenderSettings;
use crate::{Matrix, Vector};

pub struct Perceptron<const SIZE: usize, Next>
where
    Next: Layer,
{
    weights: Matrix,
    bias: Vector,
    next: Rc<RefCell<Next>>,
    state: Rc<RefCell<LayerState>>,
}

impl<const SIZE: usize, Next> Perceptron<SIZE, Next>
where
    Next: Layer,
{
    pub fn from_array(
        weights_array: [f32; SIZE * Next::SIZE],
        bias_array: [f32; Next::SIZE],
        next: Next,
    ) -> Self {
        let weights = Matrix::from_shape_vec((Next::SIZE, SIZE), weights_array.to_vec()).unwrap();
        let bias = Vector::from_vec(bias_array.to_vec());

        Perceptron {
            weights,
            bias,
            next: Rc::new(RefCell::new(next)),
            state: Rc::new(RefCell::new(LayerState::NeedsInput)),
        }
    }
}

impl<const SIZE: usize, Next> Layer for Perceptron<SIZE, Next>
where
    Next: Layer,
{
    const SIZE: usize = SIZE;

    fn get_next(&self) -> Option<Rc<RefCell<impl Layer>>> {
        Some(self.next.clone())
    }

    fn get_state(&self) -> Rc<RefCell<LayerState>> {
        self.state.clone()
    }

    fn process(&self, input: &Vector) -> Vector {
        self.weights.dot(input) + self.bias.clone()
    }

    fn render(
        &self,
        x: usize,
        settings: &RenderSettings,
        bias: Option<&Vector>,
    ) -> Vec<(f32, f32)> {
        let targets = self.next.borrow().render(x + 1, settings, Some(&self.bias));

        let mut colours = [
            macroquad::prelude::RED,
            macroquad::prelude::GREEN,
            macroquad::prelude::BLUE,
            macroquad::prelude::MAGENTA,
            macroquad::prelude::YELLOW,
            macroquad::prelude::SKYBLUE,
            macroquad::prelude::WHITE,
            macroquad::prelude::BLACK,
        ]
        .into_iter()
        .cycle();

        (0..SIZE)
            .map(|y| {
                let (x1, y1) = settings.screen_coords(x, y, SIZE);

                settings.draw_layer_node(
                    x1,
                    y1,
                    macroquad::prelude::BLUE,
                    bias.map(|bias| settings.format_float(bias[y])),
                    self.state
                        .borrow()
                        .get_input()
                        .map(|input| settings.format_float(input[y])),
                );

                targets
                    .iter()
                    .enumerate()
                    .for_each(|(target_idx, (x2, y2))| {
                        let colour = colours.next().unwrap();
                        settings.draw_line(
                            x1,
                            y1,
                            x2,
                            y2,
                            colour,
                            settings.format_float(self.weights[(target_idx, x)]),
                        );
                    });

                (x1, y1)
            })
            .collect()
    }
}

pub fn perceptron<const SIZE: usize, Next: Layer>(
    array: [f32; SIZE * Next::SIZE],
    a2: [f32; Next::SIZE],
    next: Next,
) -> Perceptron<SIZE, Next> {
    Perceptron::from_array(array, a2, next)
}
