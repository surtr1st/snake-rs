use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread;
use std::time::Duration;

const GRID_CELL: i32 = 25;
const GRID_WIDTH: i32 = 32;
const GRID_HEIGHT: i32 = 24;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Snake", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let snake = Rect::new(25, 25, 25, 25);

    draw_grid(&mut canvas);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here
        canvas.fill_rect(snake).unwrap();
        canvas.set_draw_color(Color::GREEN);

        //
        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_grid(canvas: &mut Canvas<Window>) {
    let window_width = (GRID_WIDTH * GRID_CELL) + 1;
    let window_height = (GRID_HEIGHT * GRID_CELL) + 1;
    let rows = GRID_WIDTH * GRID_CELL;
    let cols = GRID_HEIGHT * GRID_CELL;

    let mut x = 0;
    let mut y = 0;

    for _ in 0..rows {
        x += GRID_CELL;
        canvas.set_draw_color(Color::WHITE);
        canvas
            .draw_line(Point::new(x, 0), Point::new(x, window_height))
            .unwrap();
    }
    for _ in 0..cols {
        y += GRID_CELL;
        canvas.set_draw_color(Color::WHITE);
        canvas
            .draw_line(Point::new(0, y), Point::new(window_width, y))
            .unwrap();
    }
}
