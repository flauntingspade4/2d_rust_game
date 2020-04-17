use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use na::Point2;

use std::f32;

use core::f32::consts::PI;

use crate::ai_type::{init_types, AiType};
use crate::assets::Assets;
use crate::player::Player;

pub struct MainState {
    player: Player,
    assets: Assets,
    types: Vec<AiType>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let player = Player::new();
        let assets = Assets::new(ctx).unwrap();
        let types = init_types(ctx);

        let s = MainState {
            player,
            assets,
            types,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for types in self.types.iter_mut() {
            types.update(&mut self.player);
        }
        self.player.update(ctx);
        if self.player.check_health() <= 0 {
            ctx.continuing = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //graphics::clear(ctx, graphics::Color::from_rgb_u32(0x4E4132));
        graphics::clear(ctx, graphics::Color::from_rgb_u32(0xffffff));

        graphics::draw(
            ctx,
            &self.assets.player_image,
            DrawParam::default()
                .dest(self.player.dst)
                .offset(Point2::new(32. as f32 / 64.0, 32. as f32 / 64.0))
                .rotation(self.player.rotation),
        )?;

        for types in self.types.iter_mut() {
            types.draw(ctx);
        }

        /*for x in 0..=10 {
            for y in 0..=10 {
                let x = x as f32;
                let y = y as f32;
                let p = DrawParam::new()
                    .dest(Point2::new(x * 10.0, y * 10.0))
                    .scale(Vector2::new(2.0, 2.0))
                    .rotation(time as f32 / 1000.);
                self.all_pines.add(p);
            }
        }
        graphics::draw(ctx, &self.all_pines, DrawParam::default())?;
        self.all_pines.clear();*/

        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.player.rotation = -(y - self.player.dst.y).atan2(self.player.dst.x - x) - PI / 2.;
    }
}
