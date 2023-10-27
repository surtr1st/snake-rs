pub mod apple;
pub mod snake;
use apple::Apple;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use snake::Snake;
use std::thread;
use std::time::Duration;

const TITLE: &str = "Snake";
const GRID_CELL: i32 = 25;
const GRID_WIDTH: i32 = 32;
const GRID_HEIGHT: i32 = 24;
const SCREEN_SIZE: (u32, u32) = (800, 600);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (width, height) = SCREEN_SIZE;
    let window = video_subsystem
        .window(TITLE, width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut snake = Snake::new(0, 0);
    let mut apple = Apple::spawn();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        draw_grid(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Q => break 'running,
                    Keycode::W => snake.go_up(),
                    Keycode::S => snake.go_down(),
                    Keycode::A => snake.go_left(),
                    Keycode::D => snake.go_right(),
                    _ => {}
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here

        let size = GRID_CELL as u32;
        let rsnake = snake.rect(size);
        let rapple = apple.rect(size);

        // Collision detection
        if detect_collision(&rsnake, &rapple) {
            apple = Apple::spawn();
        }

        // Snake color
        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rsnake).unwrap();

        // Apple color
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(rapple).unwrap();

        // Background color
        canvas.set_draw_color(Color::BLACK);

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
        canvas.set_draw_color(Color::GREY);
        canvas
            .draw_line(Point::new(x, 0), Point::new(x, window_height))
            .unwrap();
    }
    for _ in 0..cols {
        y += GRID_CELL;
        canvas.set_draw_color(Color::GREY);
        canvas
            .draw_line(Point::new(0, y), Point::new(window_width, y))
            .unwrap();
    }
}

fn detect_collision(target: &Rect, opponent: &Rect) -> bool {
    let collided = target.has_intersection(*opponent);
    if collided {
        true
    } else {
        false
    }
}
