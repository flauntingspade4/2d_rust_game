use ggez::nalgebra as na;

use rand::Rng;

#[derive(Clone, Copy, std::fmt::Debug)]
pub struct Pine {
    pub dst: na::Point2<f32>,
    pub alive: u32,
    pub rotation: f32,
}

impl Pine {
    pub fn from_coordsr(x: f32, y: f32) -> Self {
        let dst = na::Point2::new(x, y);
        let alive = 0;
        let rotation = rand::thread_rng().gen_range(0., 360.);
        return Self { dst, alive, rotation }
    }
    pub fn check_alive(&self) -> bool {
        if self.alive >= 600 {
            return false
        }
        return true
    }
}