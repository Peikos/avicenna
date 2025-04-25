use std::cell::RefCell;
use std::rc::Rc;

use crate::Vector;
use crate::layers::{Layer, LayerState};
use crate::render::RenderSettings;

pub enum ActivationFunction {
    Sigmoid,
    Tanh,
    Relu,
}

pub struct Activation<Next>
where
    Next: Layer,
{
    function: ActivationFunction,
    next: Rc<RefCell<Next>>,
    state: Rc<RefCell<LayerState>>,
}

impl<Next> Activation<Next>
where
    Next: Layer,
{
    pub fn sigmoid(next: Next) -> Self {
        Activation {
            function: ActivationFunction::Sigmoid,
            next: Rc::new(RefCell::new(next)),
            state: Rc::new(RefCell::new(LayerState::NeedsInput)),
        }
    }
    pub fn tanh(next: Next) -> Self {
        Activation {
            function: ActivationFunction::Tanh,
            next: Rc::new(RefCell::new(next)),
            state: Rc::new(RefCell::new(LayerState::NeedsInput)),
        }
    }
    pub fn relu(next: Next) -> Self {
        Activation {
            function: ActivationFunction::Relu,
            next: Rc::new(RefCell::new(next)),
            state: Rc::new(RefCell::new(LayerState::NeedsInput)),
        }
    }
}

impl<Next> Layer for Activation<Next>
where
    Next: Layer,
{
    const SIZE: usize = Next::SIZE;

    fn get_next(&self) -> Option<Rc<RefCell<impl Layer>>> {
        Some(self.next.clone())
    }

    fn get_state(&self) -> Rc<RefCell<LayerState>> {
        self.state.clone()
    }

    fn process_all(&self, input: &Vector) -> Vector {
        self.next.borrow().process_all(&self.process(input))
    }

    fn process(&self, input: &Vector) -> Vector {
        match self.function {
            ActivationFunction::Sigmoid => 1. / (1. + (-input).exp()),
            ActivationFunction::Relu => input.iter().map(|x| x.max(0.)).collect(),
            ActivationFunction::Tanh => input.iter().map(|x| x.tanh()).collect(),
        }
    }

    fn render(
        &self,
        x: usize,
        settings: &RenderSettings,
        bias: Option<&Vector>,
    ) -> Vec<(f32, f32)> {
        self.next.borrow().render(x, settings, bias)
    }
}

pub fn sigmoid<Next: Layer>(next: Next) -> Activation<Next> {
    Activation::sigmoid(next)
}

pub fn tanh<Next: Layer>(next: Next) -> Activation<Next> {
    Activation::tanh(next)
}

pub fn relu<Next: Layer>(next: Next) -> Activation<Next> {
    Activation::relu(next)
}
