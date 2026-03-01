use raylib::color::Color;
use raylib::math::Vector2;
use raylib::prelude::RaylibDraw;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const THICKNESS: f32 = 15.0;
const DEFAULT_ANGLE: f32 = 30_f32.to_radians();

fn draw_fractal_tree(
    drw: &mut impl RaylibDraw,
    x1: f32,
    y1: f32,
    length: f32,
    angle: f32,
    thickness: f32,
) {
    if thickness < 1.0 || length < 2.0 {
        return;
    }

    let x2 = x1 + length * angle.sin();
    let y2 = y1 - length * angle.cos();

    drw.draw_line_ex(
        Vector2 { x: x1, y: y1 },
        Vector2 { x: x2, y: y2 },
        thickness,
        Color::WHITE,
    );

    let new_length = length * 0.7;
    let new_thickness = thickness * 0.7;

    draw_fractal_tree(
        drw,
        x2,
        y2,
        new_length,
        angle + DEFAULT_ANGLE,
        new_thickness,
    );
    draw_fractal_tree(
        drw,
        x2,
        y2,
        new_length,
        angle - DEFAULT_ANGLE,
        new_thickness,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Fractal Tree")
        .build();

    rl.set_target_fps(1);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        let x1 = WIDTH as f32 / 2.0;
        let y1 = HEIGHT as f32;
        let trunk_length = HEIGHT as f32 / 4.0;
        let trunk_angle = 0.0;

        draw_fractal_tree(&mut d, x1, y1, trunk_length, trunk_angle, THICKNESS);
    }
}
