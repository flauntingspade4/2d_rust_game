use ggez::nalgebra as na;

pub struct Pine {
    pub coord: na::Point2<f32>,
    pub alive: f32,
    pub rotation: f32,
}

impl Pine {
    pub fn check_alive(&self) -> bool {
        if self.alive >= 5.0 {
            return false
        }
        return true
    }
}