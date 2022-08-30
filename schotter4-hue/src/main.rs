use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou_egui::{self, egui, Egui};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.04;
const DARK: bool = true;
const CONTRAST: bool = false;
const HUE_START: f32 = 0.35;
const HUE_RANGE: f32 = 0.5; // overflows

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

struct Model {
    ui: Egui,
    main_window: WindowId,
    random_seed: u64,
    dark_mode: bool,
    contrast_mode: bool,
    disp_adj: f32,
    rot_adj: f32,
    hue_start: f32,
    hue_range: f32,
    gravel: Vec<Stone>,
    shots: u32,
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
    hue: f32,
    sat: f32,
    lum: f32,
    x_velocity: f32,
    y_velocity: f32,
    rot_velocity: f32,
    cycles: u32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let rotation = 0.0;
        let hue = 0.0;
        let sat = 0.0;
        let lum = 0.0;
        let x_velocity = 0.0;
        let y_velocity = 0.0;
        let rot_velocity = 0.0;
        let cycles = 0;
        Stone {
            x,
            y,
            x_offset,
            y_offset,
            rotation,
            hue,
            sat,
            lum,
            x_velocity,
            y_velocity,
            rot_velocity,
            cycles,
        }
    }
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(280, 200)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    let random_seed = random_range(0, 1000000);
    let disp_adj = 1.0;
    let rot_adj = 1.0;
    let hue_start = HUE_START;
    let hue_range = HUE_RANGE;

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            let stone = Stone::new(x as f32, y as f32);
            gravel.push(stone);
        }
    }

    Model {
        ui,
        main_window,
        random_seed,
        dark_mode: DARK,
        contrast_mode: CONTRAST,
        disp_adj,
        rot_adj,
        hue_start,
        hue_range,
        gravel,
        shots: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    let mut rng = StdRng::seed_from_u64(model.random_seed);
    for stone in &mut model.gravel {
        let factor = stone.y / ROWS as f32;
        if stone.cycles == 0 {
            let disp_factor = factor * model.disp_adj;
            let rot_factor = factor * model.rot_adj;
            let new_x = disp_factor * rng.gen_range(-0.5..0.5);
            let new_y = disp_factor * rng.gen_range(-0.5..0.5);
            let new_rot = rot_factor * rng.gen_range(-PI / 4.0..PI / 4.0);
            let new_cycles = rng.gen_range(50..300);
            stone.x_velocity = (new_x - stone.x_offset) / new_cycles as f32;
            stone.y_velocity = (new_y - stone.y_offset) / new_cycles as f32;
            stone.rot_velocity = (new_rot - stone.rotation) / new_cycles as f32;
            stone.cycles = new_cycles;
        } else {
            stone.x_offset += stone.x_velocity;
            stone.y_offset += stone.y_velocity;
            stone.rotation += stone.rot_velocity;
            stone.cycles -= 1;
        }

        let hue_end = model.hue_start + model.hue_range;
        let hue = map_range(factor, 0.0, 1.0, model.hue_start, hue_end);
        stone.hue = if hue > 1.0 { hue - 1.0 } else { hue };
        let (sat, lum) = match (model.contrast_mode, model.dark_mode) {
            (true, true) => (0.8, 0.75),
            (true, false) => (0.4, 0.4),
            (false, true) => (0.4, 0.4),
            (false, false) => (0.8, 0.75),
        };
        stone.sat = sat;
        stone.lum = lum;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    gdraw
        .background()
        .color(if model.dark_mode { BLACK } else { SNOW });

    for stone in &model.gravel {
        let cdraw = gdraw.x_y(stone.x, stone.y);
        cdraw
            .rect()
            .color(hsla(stone.hue, stone.sat, stone.lum, 0.8))
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.rotation);
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
            match app.window(model.main_window) {
                Some(window) => {
                    window.capture_frame(format!(
                        "{}_{}.png",
                        app.exe_name().unwrap(),
                        &model.shots
                    ));
                }
                None => {}
            }
        }
        Key::Up => {
            if model.disp_adj < 5.0 {
                model.disp_adj += 0.1;
            }
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            if model.rot_adj < 5.0 {
                model.rot_adj += 0.1;
            }
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.hue_start, 0.0..=1.0).text("Hue"));
            ui.add(egui::Slider::new(&mut model.hue_range, 0.0..=1.0).text("Hue Range"));
            ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(30.0);
                if ui.add(egui::Button::new("Randomize")).clicked() {
                    model.random_seed = random_range(0, 1000000);
                }
                ui.add(egui::DragValue::new(&mut model.random_seed));
                ui.label("Seed");
            });
        });
}
