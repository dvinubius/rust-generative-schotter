use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.04;
const DARK: bool = true;
const CONTRAST: bool = false;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    random_seed: u64,
    dark_mode: bool,
    contrast_mode: bool,
    disp_adj: f32,
    rot_adj: f32,
    shots: u32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let random_seed = random_range(0, 1000000);
    let disp_adj = 1.0;
    let rot_adj = 1.0;
    Model {
        random_seed,
        dark_mode: DARK,
        contrast_mode: CONTRAST,
        disp_adj,
        rot_adj,
        shots: 0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut rng = StdRng::seed_from_u64(model.random_seed);

    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    gdraw
        .background()
        .color(if model.dark_mode { BLACK } else { SNOW });

    for y in 0..ROWS {
        for x in 0..COLS {
            let cdraw = gdraw.x_y(x as f32, y as f32);
            let factor = y as f32 / ROWS as f32;
            let disp_factor = factor * model.disp_adj;
            let rot_factor = factor * model.rot_adj;
            let x_offset = disp_factor * rng.gen_range(-0.5..0.5);
            let y_offset = disp_factor * rng.gen_range(-0.5..0.5);
            let rotation = rot_factor * rng.gen_range(-PI / 4.0..PI / 4.0);
            let start_hue = 0.35;
            let diff_hue_max = 0.7 - start_hue;
            let hue = start_hue + map_range(factor, 0.0, 1.0, 0.0, diff_hue_max);
            let (sat, lum) = match (model.contrast_mode, model.dark_mode) {
                (true, true) => (0.8, 0.75),
                (true, false) => (0.4, 0.4),
                (false, true) => (0.4, 0.4),
                (false, false) => (0.8, 0.75),
            };
            cdraw
                .rect()
                .color(hsla(hue, sat, lum, 0.8))
                .stroke(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x_offset, y_offset)
                .rotate(rotation);
        }
    }

    gdraw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::D => model.dark_mode = !model.dark_mode,
        Key::C => model.contrast_mode = !model.contrast_mode,
        Key::S => {
            model.shots += 1;
            app.main_window().capture_frame(format!(
                "{}_{}.png",
                app.exe_name().unwrap(),
                &model.shots
            ));
        }
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }
}
