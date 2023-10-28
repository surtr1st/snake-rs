pub mod apple;
pub mod constants;
pub mod snake;
use apple::Apple;
use constants::{GRID_HEIGHT, GRID_WIDTH, SCREEN_SIZE, TITLE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::FontStyle;
use sdl2::video::{Window, WindowContext};
use snake::{Snake, SnakeDirection};
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::constants::GRID_CELL;

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
    let mut texture_creator = canvas.texture_creator();

    canvas.clear();
    canvas.present();

    let mut score = 0;
    let mut snake = Snake::new();
    let mut apple = Apple::spawn();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        let score_indicate = format!("Score: {}", score);
        draw_text(&mut canvas, &mut texture_creator, score_indicate);
        draw_grid(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Q => break 'running,
                    Keycode::W => match snake.direction() {
                        SnakeDirection::Down => {}
                        _ => snake.go_up(),
                    },
                    Keycode::S => match snake.direction() {
                        SnakeDirection::Up => {}
                        _ => snake.go_down(),
                    },
                    Keycode::A => match snake.direction() {
                        SnakeDirection::Right => {}
                        _ => snake.go_left(),
                    },
                    Keycode::D => match snake.direction() {
                        SnakeDirection::Left => {}
                        _ => snake.go_right(),
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here

        let size = GRID_CELL as u32;
        snake.wriggle();

        resolve_overflow(&mut snake);
        let rsnake = snake.body().clone();
        let rapple = apple.rect(size);

        if detect_collision(&rsnake[0], &rapple) {
            apple = Apple::spawn();
            snake.grow();
            score += 100;
        }

        for i in 1..rsnake.len() {
            if detect_collision(&rsnake[0], &rsnake[i]) {
                break 'running;
            }
        }

        // Snake color
        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rects(&rsnake).unwrap();

        // Apple color
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(rapple).unwrap();

        // Background color
        canvas.set_draw_color(Color::BLACK);

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));
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

fn resolve_overflow(target: &mut Snake) {
    let (x, y) = target.position();
    let size = SCREEN_SIZE;
    let width = size.0 as i32;
    let height = size.1 as i32;

    if x >= width {
        target.add_coordinate((-25, y));
    }

    if x < 0 {
        target.add_coordinate((width, y));
    }

    if y >= height {
        target.add_coordinate((x, -25));
    }

    if y < 0 {
        target.add_coordinate((x, height));
    }
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &mut TextureCreator<WindowContext>,
    content: String,
) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("fonts/Inter-V.ttf");
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(FontStyle::BOLD);

    let surface = font.render(&content).blended(Color::WHITE).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    let target = Rect::new((SCREEN_SIZE.0 - 400) as i32, 0, 170, 50);

    canvas.copy(&texture, None, Some(target)).unwrap();
}
