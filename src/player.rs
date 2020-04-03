use ggez::nalgebra as na;

pub struct Player {
    pub coord: na::Vector2<f32>,
    pub rotation: f32, // In radians
}

impl Player {
    pub fn new() -> Self {
        let coord = na::Vector2::new(400., 300.);
        return Player { coord, rotation: 0. }
    }
}