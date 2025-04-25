use std::cell::RefCell;
use std::rc::Rc;

use crate::Vector;
use crate::layers::{Layer, LayerState};
use crate::render::RenderSettings;

pub struct OutputLayer<const SIZE: usize> {
    state: Rc<RefCell<LayerState>>,
}

impl<const SIZE: usize> Layer for OutputLayer<SIZE> {
    const SIZE: usize = SIZE;

    fn get_next(&self) -> Option<Rc<RefCell<impl Layer>>> {
        None::<Rc<RefCell<OutputLayer<SIZE>>>>
    }

    fn get_state(&self) -> Rc<RefCell<LayerState>> {
        self.state.clone()
    }

    fn process_all(&self, input: &Vector) -> Vector {
        input.clone() // Just pass through for the basic case
    }

    fn process(&self, input: &Vector) -> Vector {
        input.clone() // Just pass through for the basic case
    }

    fn render(
        &self,
        x: usize,
        settings: &RenderSettings,
        bias: Option<&Vector>,
    ) -> Vec<(f32, f32)> {
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

                (x1, y1)
            })
            .collect()
    }
}

pub fn output<const SIZE: usize>() -> OutputLayer<SIZE> {
    OutputLayer {
        state: Rc::new(RefCell::new(LayerState::NeedsInput)),
    }
}
