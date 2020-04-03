use ggez::nalgebra as na;

pub struct Player {
    pub dst: na::Point2<f32>,
    pub rotation: f32, // In radians
}

impl Player {
    pub fn new() -> Self {
        let dst = na::Point2::new(400., 300.);
        return Player { dst, rotation: 0. }
    }
}