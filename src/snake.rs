use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Snake(i32, i32);

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Snake(x, y)
    }

    pub fn rect(&self, width: u32, height: u32) -> Rect {
        Rect::new(self.0, self.1, width, height)
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
