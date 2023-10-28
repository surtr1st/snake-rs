use sdl2::rect::Rect;

enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}

pub struct Snake {
    x: i32,
    y: i32,
    direction: SnakeDirection,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Snake {
            x,
            y,
            direction: SnakeDirection::Right,
        }
    }

    pub fn rect(&self, size: u32) -> Rect {
        Rect::new(self.x, self.y, size, size)
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
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
        match self.direction {
            SnakeDirection::Up => self.y -= 25,
            SnakeDirection::Down => self.y += 25,
            SnakeDirection::Left => self.x -= 25,
            SnakeDirection::Right => self.x += 25,
        }
    }
}
