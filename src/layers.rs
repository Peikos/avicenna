pub mod activation;
pub mod output;
pub mod perceptron;

use std::cell::RefCell;
use std::rc::Rc;

use crate::Vector;
use crate::render::RenderSettings;

pub trait Layer {
    const SIZE: usize;

    fn get_state(&self) -> Rc<RefCell<LayerState>>;

    fn get_next(&self) -> Option<Rc<RefCell<impl Layer>>>;

    fn feed(&mut self, input: Vector) {
        self.get_state().replace(LayerState::HasInput(input));
    }

    fn reset(&mut self) {
        self.get_state().replace(LayerState::NeedsInput);
        if let Some(next) = self.get_next() {
            next.borrow_mut().reset();
        }
    }

    fn process_all(&self, input: &Vector) -> Vector {
        if let Some(next) = self.get_next() {
            next.borrow().process_all(&self.process(input))
        } else if let Some(input) = self.get_state().borrow().get_input() {
            // OutputLayer; return input
            input.clone()
        } else {
            panic!("No function and no input available")
        }
    }

    fn process(&self, input: &Vector) -> Vector;

    fn visualise(&self, settings: RenderSettings) {
        self.render(0, &settings, None);
    }

    fn render(&self, x: usize, settings: &RenderSettings, bias: Option<&Vector>)
    -> Vec<(f32, f32)>;

    fn advance(&mut self) {
        let rc = self.get_state();
        let mut state = rc.borrow_mut();
        if let Some(next) = self.get_next() {
            let mut next = next.borrow_mut();
            if state.is_done() {
                next.advance()
            } else if state.have_input() {
                let res = self.process(state.get_input().unwrap());
                state.set_output(res);
                next.feed(self.process(state.get_input().unwrap()))
            }
        }
    }
}

pub enum LayerState {
    NeedsInput,
    HasInput(Vector),
    Forward { input: Vector, output: Vector },
}

impl LayerState {
    pub fn have_input(&self) -> bool {
        !matches!(&self, Self::NeedsInput)
    }

    pub fn set_output(&mut self, o: Vector) {
        match self {
            Self::NeedsInput => panic!(),
            Self::HasInput(i) => {
                *self = Self::Forward {
                    input: i.clone(),
                    output: o,
                }
            }
            Self::Forward { .. } => {}
        }
    }

    pub fn get_input(&self) -> Option<&Vector> {
        match self {
            Self::NeedsInput => None,
            Self::HasInput(v) => Some(v),
            Self::Forward { input: v, .. } => Some(v),
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(
            &self,
            Self::Forward {
                input: _,
                output: _
            }
        )
    }
}
