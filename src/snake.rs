use sdl2::rect::Rect;

use crate::constants::GRID_CELL;

#[derive(PartialEq, Eq, Clone)]
pub enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}

pub struct Snake {
    coordinates: Vec<(i32, i32)>,
    body: Vec<Rect>,
    direction: SnakeDirection,
}

impl Snake {
    pub fn new() -> Self {
        let parts = 3;
        let mut coordinates = vec![];
        let mut body = vec![];
        for i in 0..parts {
            coordinates.insert(i, (0, 0));
        }

        let mut index = 0;
        let size = GRID_CELL as u32;
        for (x, y) in &coordinates {
            body.insert(index, Rect::new(*x, *y, size, size));
            index += 1;
        }

        Snake {
            coordinates,
            body,
            direction: SnakeDirection::Right,
        }
    }

    pub fn body(&self) -> &Vec<Rect> {
        &self.body
    }

    pub fn add_coordinate(&mut self, coordinate: (i32, i32)) {
        self.coordinates.insert(0, coordinate);
    }

    pub fn position(&self) -> (i32, i32) {
        self.coordinates[0]
    }

    pub fn direction(&self) -> SnakeDirection {
        self.direction.clone()
    }

    pub fn go_left(&mut self) {
        self.direction = SnakeDirection::Left;
    }

    pub fn go_right(&mut self) {
        self.direction = SnakeDirection::Right;
    }

    pub fn go_up(&mut self) {
        self.direction = SnakeDirection::Up;
    }

    pub fn go_down(&mut self) {
        self.direction = SnakeDirection::Down;
    }

    pub fn wriggle(&mut self) {
        let (mut x, mut y) = self.coordinates[0];

        match self.direction {
            SnakeDirection::Up => y -= GRID_CELL,
            SnakeDirection::Down => y += GRID_CELL,
            SnakeDirection::Left => x -= GRID_CELL,
            SnakeDirection::Right => x += GRID_CELL,
        }

        self.coordinates.insert(0, (x, y));

        let size = GRID_CELL as u32;
        let body_part = Rect::new(x, y, size, size);

        self.body.insert(0, body_part);

        // Delete the last element to prevent old drawing
        let coordinate_length = self.coordinates.len();
        self.coordinates.remove(coordinate_length - 1);
        let body_length = self.body.len();
        self.body.remove(body_length - 1);
    }
}
