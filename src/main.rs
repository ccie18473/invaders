pub mod assets;
pub mod frame;
pub mod invaders;
pub mod player;
pub mod render;
pub mod shot;

pub mod prelude {
    pub use macroquad::prelude::*;

    pub use crate::assets::*;
    pub use crate::frame::*;
    pub use crate::invaders::*;
    pub use crate::player::*;
    pub use crate::render::*;
    pub use crate::shot::*;

    pub const NUM_ROWS: usize = 20;
    pub const NUM_COLS: usize = 40;
}
use prelude::*;

#[macroquad::main("Invaders v1.0.0 2022")]
async fn main() {
    let assets = Assets::new().await.unwrap();

    let mut player = Player::new();
    let mut invaders = Invaders::new();

    loop {
        clear_background(BLUE);
        let delta = get_frame_time();

        // Per-frame init
        let mut frame = new_frame();

        // Input
        if is_key_down(KeyCode::Left) {
            player.move_left();
        }
        if is_key_down(KeyCode::Right) {
            player.move_right()
        }
        if is_key_down(KeyCode::Space) | is_key_down(KeyCode::Enter) {
            player.shoot();
        }
        if is_key_down(KeyCode::Escape) | is_key_down(KeyCode::Q) {
            break;
        }

        // Updates
        player.update(delta);
        invaders.update(delta);
        player.detect_hits(&mut invaders);

        // Draw & render

        let fps = 1.0 / delta;
        let fps_str = format!("{}{:.1}", "FPS: ", fps);
        draw_text(&fps_str, 10.0, 20.0, 20.0, WHITE);

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut frame, &assets);
        }

        // Win or lose?
        if invaders.all_killed() {
            break;
        }
        if invaders.reached_bottom() {
            break;
        }

        // render
        render::render(&frame);

        next_frame().await
    }
}
