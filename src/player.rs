use ggez::nalgebra as na;

pub struct Player {
    pub coord: na::Point2<f32>,
}

impl Player {
    pub fn new() -> Self {
        let coord = na::Point2::new(400., 300.);
        return Player { coord }
    }
}