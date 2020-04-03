use ggez::graphics;
use ggez::{Context, GameResult};

#[derive(Clone)]
pub struct Assets {
    pub player_image: graphics::Image,
    pub shot_image: graphics::Image,
    pub pine_image: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let pine_image = graphics::Image::new(ctx, "/pine.png")?;

        Ok(Assets {
            player_image,
            shot_image,
            pine_image,
        })
    }
}