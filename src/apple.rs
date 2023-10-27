use rand::Rng;
use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Apple(i32, i32);

impl Apple {
    pub fn spawn() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=25) * 25;
        let y = rng.gen_range(0..=25) * 25;
        Apple(x, y)
    }

    pub fn rect(&self, size: u32) -> Rect {
        Rect::new(self.0, self.1, size, size)
    }
}
