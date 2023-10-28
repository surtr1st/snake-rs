use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Snake(i32, i32);

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Snake(x, y)
    }

    pub fn rect(&self, size: u32) -> Rect {
        Rect::new(self.0, self.1, size, size)
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn set_x(&mut self, x: i32) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.1 = y;
    }

    pub fn go_left(&mut self) {
        self.0 -= 25;
    }

    pub fn go_right(&mut self) {
        self.0 += 25;
    }

    pub fn go_up(&mut self) {
        self.1 -= 25;
    }

    pub fn go_down(&mut self) {
        self.1 += 25;
    }
}
