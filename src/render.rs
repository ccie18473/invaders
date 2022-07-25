use crate::prelude::*;

pub fn render(frame: &Frame) {
    for (x, col) in frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            draw_texture(*s, 20.0 * x as f32, 20.0 * y as f32, WHITE);
        }
    }
}
