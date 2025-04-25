#![feature(generic_const_exprs)]

pub mod layers;
pub mod render;

use itertools::Itertools;
use ndarray::{Array, Array1, Array2};
//use ndarray_rand::RandomExt;
//use rand::distributions::Uniform;

use crate::layers::{Layer, activation::*, output::*, perceptron::*};
use crate::render::RenderSettings;

use macroquad::prelude::{Conf, KeyCode, is_key_pressed, next_frame};

type Vector = Array1<f32>;
type Matrix = Array2<f32>;

fn window_conf() -> Conf {
    Conf {
        window_title: "Avicenna".to_owned(),
        window_width: 1600,
        window_height: 1200,
        fullscreen: false,
        sample_count: 16,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //let test: Perceptron<40, 10, Activation<10, Perceptron<10, 3, OutputLayer<3>>>> = todo!();
    //let should_fail: Perceptron<40, 9, Activation<10, Perceptron<10, 3, OutputLayer<3>>>> = todo!();

    let test = perceptron::<2, _>([1., 2., 3., 4.], [0., 0.], output::<2>());
    println!("{}", test.process(&Array::from_vec(vec![1., 1.])));

    println!();

    let sig = sigmoid(output::<1>());
    println!(
        "σ(-1000) = {}",
        sig.process(&Vector::from_vec(vec![-1000.]))[0]
    );
    println!("σ(0)     = {}", sig.process(&Vector::from_vec(vec![0.]))[0]);
    println!(
        "σ(1000)  = {}",
        sig.process(&Vector::from_vec(vec![1000.]))[0]
    );

    println!();

    let mut xor = perceptron::<2, _>(
        [100., 100., -100., -100.],
        [-50., 150.],
        sigmoid(perceptron::<2, _>(
            [100., 100.],
            [-150.],
            sigmoid(output::<1>()),
        )),
    );
    //let xor: Perceptron<2,2, Activation<2, OutputLayer<2>>> = Perceptron::from_array([1.,1.,1.,1.], [0.5, -1.5], Activation::sigmoid(OutputLayer));

    (0..=1)
        .cartesian_product(0..=1)
        .for_each(|(a, b): (u8, u8)| {
            println!(
                "{} ⊕ {} = {}",
                a,
                b,
                xor.process_all(&Vector::from_vec(vec![a.into(), b.into()]))[0].round()
            );
        });

    let mut frame_count: u32 = 0;
    let mut scale = 2.5;
    let mut x_pos = -500.;
    let mut y_pos = -200.;

    let mut demo = (0..=1)
        .cartesian_product(0..=1)
        .map(|(a, b): (u8, u8)| Vector::from_vec(vec![a.into(), b.into()]))
        .cycle();

    loop {
        if is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }

        if is_key_pressed(KeyCode::Equal) {
            scale += 0.5;
        }

        if is_key_pressed(KeyCode::Minus) {
            scale -= 0.5;
        }

        if is_key_pressed(KeyCode::Left) {
            x_pos -= 50.;
        }

        if is_key_pressed(KeyCode::Right) {
            x_pos += 50.;
        }

        if is_key_pressed(KeyCode::Up) {
            y_pos -= 50.;
        }

        if is_key_pressed(KeyCode::Down) {
            y_pos += 50.;
        }

        if frame_count == 100 {
            xor.feed(demo.next().unwrap());
        } else if frame_count == 0 {
            xor.reset();
        } else if frame_count % 100 == 0 {
            xor.advance();
        }

        xor.visualise(
            //Array::from_vec(vec![0.0, 1.0]),
            RenderSettings::default().scale(scale).translate(x_pos, y_pos),
        );

        frame_count += 1;
        frame_count %= 1000;

        next_frame().await
    }
}
