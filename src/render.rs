use macroquad::prelude::{draw_arc, draw_circle_lines, draw_line, draw_text, measure_text};

#[derive(Clone)]
pub struct RenderSettings {
    window_width: i32,
    window_height: i32,
    x_spacing: f32,
    y_spacing: f32,
    neuron_size: f32,
    line_width: f32,
    text_size: f32,
    line_length: f32,
    x_offset: f32,
    y_offset: f32,
    float_precision: usize,
}

impl Default for RenderSettings {
    fn default() -> RenderSettings {
        RenderSettings {
            window_height: 1600,
            window_width: 1200,
            x_offset: 800.,
            y_offset: 600.,
            x_spacing: 200.,
            y_spacing: 144.,
            neuron_size: 48.,
            line_width: 2.,
            text_size: 20.,
            line_length: 0.9,
            float_precision: 2,
        }
    }
}

impl RenderSettings {
    pub fn scale(self, factor: f32) -> Self {
        RenderSettings {
            x_spacing: self.x_spacing * factor,
            y_spacing: self.y_spacing * factor,
            neuron_size: self.neuron_size * factor,
            line_width: self.line_width * factor,
            text_size: self.text_size * factor,
            ..self
        }
    }

    pub fn translate(self, dx: f32, dy: f32) -> Self {
        RenderSettings {
            x_offset: self.x_offset + dx,
            y_offset: self.y_offset + dy,
            ..self
        }
    }

    pub fn draw_line(
        &self,
        x1: f32,
        y1: f32,
        x2: &f32,
        y2: &f32,
        colour: macroquad::prelude::Color,
        label: String,
    ) {
        let (x_min, y_min, x_max, y_max) = if x1 < *x2 {
            (x1, y1, *x2, *y2)
        } else {
            (*x2, *y2, x1, y1)
        };
        let slope = (y_max - y_min) / (x_max - x_min);
        let intercept = (self.neuron_size.powf(2.) / (1. + slope.powf(2.))).sqrt();

        let x1 = x_min + intercept / self.line_length;
        let x2 = x_max - intercept / self.line_length;
        let y1 = y_min + intercept * slope / self.line_length;
        let y2 = y_max - intercept * slope / self.line_length;

        draw_arrow(x1, y1, x2, y2, self.line_width, 5., colour);

        let td = measure_text(&label, None, self.text_size.round() as u16, 1.);

        let (x_label, y_label) = if slope.abs() <= 0.001 {
            ((x1 + x2) / 2.0 - td.width / 2., (y1 + y2) / 2. - 15.)
        } else if slope < 0. {
            (x2 - td.width, y2 - 5.)
        } else {
            (x2 - td.width, y2 - td.height - 20.)
        };

        draw_text(&label, x_label, y_label, self.text_size, colour);
    }

    pub fn draw_layer_node(
        &self,
        x: f32,
        y: f32,
        colour: macroquad::prelude::Color,
        bias: Option<String>,
        input: Option<String>,
    ) {
        draw_circle_lines(x, y, self.neuron_size, self.line_width, colour);

        if let Some(input_label) = input {
            let td = measure_text(&input_label, None, self.text_size.round() as u16, 1.);

            draw_text(
                &input_label,
                x - td.width / 2.,
                y + td.height / 2.,
                self.text_size,
                macroquad::prelude::WHITE,
            );
        }

        if let Some(bias_label) = bias {
            let td = measure_text(&bias_label, None, self.text_size.round() as u16, 1.);

            draw_text(
                &bias_label,
                x - td.width / 2.,
                y + self.neuron_size * 2. / 3. + td.height / 2.,
                self.text_size,
                colour,
            );
        }
    }

    pub fn screen_coords(&self, x: usize, y: usize, nodes: usize) -> (f32, f32) {
        let x1 = x as f32 * self.x_spacing + self.x_offset;
        let y_offset = if nodes % 2 == 0 {
            0.
        } else {
            self.y_spacing / 2.
        };
        let y1 = self.y_offset + y_offset + self.y_spacing * y as f32;
        (x1, y1)
    }

    pub fn format_float(&self, input: f32) -> String {
        format!("{:.1$}", input, self.float_precision).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn draw_arrow(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    width: f32,
    marker_size: f32,
    colour: macroquad::prelude::Color,
) {
    let slope = (y2 - y1) / (x2 - x1);

    draw_line(x1, y1, x2, y2, width, colour);

    draw_arc(
        x2 - marker_size,
        y2 - marker_size * slope,
        3,
        marker_size,
        180. / std::f32::consts::PI * (y2 - y1).atan2(x2 - x1) - 120.,
        marker_size,
        240.,
        colour,
    );
}
