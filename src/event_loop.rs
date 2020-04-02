use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::input::keyboard::{ KeyCode, KeyMods, is_key_pressed };

use rand::Rng;
use std::f32;

use crate::assets::Assets;
use crate::player::Player;

const PLAYER_SPEED: f32 = 16.;
const PLAYER_WIDTH: f32 = 32.;
const PLAYER_HEIGHT: f32 = 32.;

pub struct MainState {
    player: Player,
    circ_pos_x: f32,
    circ_pos_y: f32,
    assets: Assets,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let player = Player::new();
        let circ_pos_x = 0.;
        let circ_pos_y = rand::thread_rng().gen_range(0., 800.);
        let assets = Assets::new(ctx).unwrap();

        let s = MainState { player, circ_pos_x, circ_pos_y, assets };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.circ_pos_x = self.circ_pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb_u32(0x4E4132));

        let dst = na::Point2::new(self.player.coord.x, self.player.coord.y);
        graphics::draw(ctx, &self.assets.player_image, (dst,))?;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(self.circ_pos_x, self.circ_pos_y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        let up_down: bool = is_key_pressed(ctx, KeyCode::W) || is_key_pressed(ctx, KeyCode::S);
        let left_right: bool = is_key_pressed(ctx, KeyCode::A) || is_key_pressed(ctx, KeyCode::D);

        let mut speed = PLAYER_SPEED;

        if up_down && left_right {
            speed = speed / 2.0_f32.sqrt();
        }

        if is_key_pressed(ctx, KeyCode::S) && self.player.coord.y < 600. - PLAYER_HEIGHT {
            self.player.coord.y = self.player.coord.y + 1. * speed;
        }
        else if is_key_pressed(ctx, KeyCode::W) && self.player.coord.y > 0. {
            self.player.coord.y = self.player.coord.y - 1. * speed;
        }

        if is_key_pressed(ctx, KeyCode::D) && self.player.coord.x < 800. - PLAYER_WIDTH {
            self.player.coord.x = self.player.coord.x + 1. * speed;
        }
        else if is_key_pressed(ctx, KeyCode::A) && self.player.coord.x > 0. {
            self.player.coord.x = self.player.coord.x - 1. * speed;
        }
    }
}