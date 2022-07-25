use crate::prelude::*;

pub struct Assets {
    pub rustacean: Texture2D,
    pub enemy1: Texture2D,
    pub enemy2: Texture2D,
    pub explosion: Texture2D,
    pub laser: Texture2D,
}

impl Assets {
    pub async fn new() -> Result<Assets, macroquad::prelude::FileError> {
        let rustacean =
            Texture2D::from_file_with_format(include_bytes!("../assets/rustacean.png"), None);
        let enemy1 = Texture2D::from_file_with_format(include_bytes!("../assets/enemy1.png"), None);
        let enemy2 = Texture2D::from_file_with_format(include_bytes!("../assets/enemy2.png"), None);
        let explosion =
            Texture2D::from_file_with_format(include_bytes!("../assets/explosion.png"), None);
        let laser = Texture2D::from_file_with_format(include_bytes!("../assets/laser.png"), None);

        Ok(Assets {
            rustacean,
            enemy1,
            enemy2,
            explosion,
            laser,
        })
    }
}
