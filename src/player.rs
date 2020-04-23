use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::nalgebra as na;
use ggez::Context;
use na::Point2;

const PLAYER_SPEED: f32 = 8.;
const PLAYER_WIDTH: f32 = 32.;
const PLAYER_HEIGHT: f32 = 32.;

pub struct Player {
    pub dst: Point2<f32>,
    pub rotation: f32, // In radians
    health: i8,
}

impl Player {
    pub fn new() -> Self {
        let dst = Point2::new(400., 300.);
        return Player {
            dst,
            rotation: 0.,
            health: 100,
        };
    }
    pub fn damage(&mut self, damage: i8) {
        self.health -= damage;
    }
    pub fn check_health(&self) -> i8 {
        self.health
    }
    pub fn update(&mut self, ctx: &Context) {
        let up_down: bool = is_key_pressed(ctx, KeyCode::W) || is_key_pressed(ctx, KeyCode::S);
        let left_right: bool = is_key_pressed(ctx, KeyCode::A) || is_key_pressed(ctx, KeyCode::D);

        let mut speed = PLAYER_SPEED;

        if up_down && left_right {
            speed = speed / 2.0_f32.sqrt();
        }

        if is_key_pressed(ctx, KeyCode::S) && self.dst.y < 600. - PLAYER_HEIGHT {
            self.dst.y = self.dst.y + 1. * speed;
        } else if is_key_pressed(ctx, KeyCode::W) && self.dst.y > PLAYER_HEIGHT {
            self.dst.y = self.dst.y - 1. * speed;
        }

        if is_key_pressed(ctx, KeyCode::D) && self.dst.x < 800. - PLAYER_WIDTH {
            self.dst.x = self.dst.x + 1. * speed;
        } else if is_key_pressed(ctx, KeyCode::A) && self.dst.x > PLAYER_WIDTH {
            self.dst.x = self.dst.x - 1. * speed;
        }
    }
}
