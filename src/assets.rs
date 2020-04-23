use ggez::graphics;
use ggez::{Context, GameResult};

//#[derive(Copy, Clone)]
pub struct Assets {
    pub player_image: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;

        Ok(Assets { player_image })
    }
}
