use crate::prelude::*;
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: f32,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: 0.05,
        }
    }
    pub fn update(&mut self, delta: f32) {
        self.timer -= delta;

        if self.timer < 0.0 && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer = 0.05;
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = 0.25;
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer < 0.0) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame, assets: &Assets) {
        frame[self.x][self.y] = if self.exploding {
            assets.explosion
        } else {
            assets.laser
        };
    }
}
