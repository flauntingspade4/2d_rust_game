use ggez::graphics::{self, DrawParam, Image};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::player::Player;

const STANDARD_SPEED: f32 = 5.;
const LEAPER_SPEED: f32 = 2.;

const ENEMY_RADIUS: f32 = 32.;

const LEAPER_MULT: f32 = 10.;

pub struct AiType {
    pub points: Vec<Point2<f32>>,
    radius: f32,
    function: fn(&mut Point2<f32>, &mut Player) -> Point2<f32>,
    pub image: Image,
    rotation_vec: Vec<f32>,
}

trait Point2Changing {
    fn equals(&self, other: &Self, allowance: f32) -> bool;
}

impl Point2Changing for Point2<f32> {
    fn equals(&self, other: &Point2<f32>, allowance: f32) -> bool {
        if self.x + allowance >= other.x
            && self.x - allowance <= other.x
            && self.y + allowance >= other.y
            && self.y - allowance <= other.y
        {
            return true;
        }
        return false;
    }
}

impl AiType {
    pub fn from_function(
        input_function: fn(&mut Point2<f32>, &mut Player) -> Point2<f32>,
        image: Image,
        radius: f32,
    ) -> Self {
        Self {
            points: Vec::new(),
            radius,
            function: input_function,
            image: image,
            rotation_vec: Vec::new(),
        }
    }
    pub fn update(&mut self, player: &mut Player) {
        for (index, point) in (&mut self.points).iter_mut().enumerate() {
            let change_by = (self.function)(point, player);
            point.coords[0] += change_by.x;
            point.coords[1] += change_by.y;
            self.rotation_vec[index] = -(player.dst.y - point.y).atan2(point.x - player.dst.x);
        }
    }
    pub fn draw(&self, ctx: &mut Context) {
        for (index, point) in (&self.points).iter().enumerate() {
            graphics::draw(
                ctx,
                &self.image,
                DrawParam::default()
                    .dest(Point2::new(point.x, point.y))
                    .offset(Point2::new(0.5, 0.5))
                    .rotation(self.rotation_vec[index]),
            )
            .expect("Failed to draw point");
        }
    }
    pub fn add_point(&mut self, new_point: Point2<f32>) {
        self.points.push(new_point);
        self.rotation_vec.push(0.);
    }
    pub fn check_shot(&mut self, a: f32, c: f32) {
        for point in self.points.iter_mut() {
            if (a * point.x + point.y + c).abs() / (1. + a * a).sqrt() <= self.radius {
                println!("Hit somthing");
            }
        }
    }
}

pub fn init_types(ctx: &mut Context) -> Vec<AiType> {
    let mut standard: AiType = AiType::from_function(
        |point, player| {
            let mut to_return = Point2::new(0., 0.);
            let dp = Point2::new(player.dst.x - point.x, player.dst.y - point.y);
            if player.dst.equals(point, ENEMY_RADIUS) {
                player.health -= 5;
                return Point2::new(-point.x, -point.y);
            }
            if dp.x > 0. {
                to_return.x += STANDARD_SPEED;
            } else if dp.x < 0. {
                to_return.x -= STANDARD_SPEED;
            }
            if dp.y > 0. {
                to_return.y += STANDARD_SPEED;
            } else if dp.y < 0. {
                to_return.y -= STANDARD_SPEED;
            }
            return to_return;
        },
        graphics::Image::new(ctx, "/standard.png").unwrap(),
        ENEMY_RADIUS,
    );
    let mut leaper = AiType::from_function(
        |point, player| {
            let dp = Point2::new(player.dst.x - point.x, player.dst.y - point.y);
            let mut to_return = Point2::new(0., 0.);
            if player.dst.equals(point, ENEMY_RADIUS) {
                player.health -= 5;
                return Point2::new(-point.x, -point.y);
            } else if dp.x > 300. {
                to_return.x += LEAPER_SPEED * LEAPER_MULT;
            } else if dp.x <= 300. && dp.x >= -300. && dp.x > 0. {
                to_return.x += LEAPER_SPEED;
            } else if dp.x <= 300. && dp.x >= -300. && dp.x < 0. {
                to_return.x -= LEAPER_SPEED;
            } else if dp.x < -300. {
                to_return.x -= LEAPER_SPEED * LEAPER_MULT;
            }
            if dp.y > 300. {
                to_return.y += LEAPER_SPEED * LEAPER_MULT;
            } else if dp.y <= 300. && dp.y >= -300. && dp.y > 0. {
                to_return.y += LEAPER_SPEED;
            } else if dp.y <= 300. && dp.y >= -300. && dp.y < 0. {
                to_return.y -= LEAPER_SPEED;
            } else if dp.y < -300. {
                to_return.y -= LEAPER_SPEED * LEAPER_MULT;
            }
            return to_return;
        },
        graphics::Image::new(ctx, "/leaper.png").unwrap(),
        ENEMY_RADIUS,
    );
    for x in 0..4 {
        standard.add_point(Point2::new(x as f32 * 50., x as f32 * 100.));
        leaper.add_point(Point2::new(x as f32, x as f32 * 100.));
    }
    let mut ai = Vec::with_capacity(2);
    ai.push(standard);
    ai.push(leaper);
    return ai;
}
