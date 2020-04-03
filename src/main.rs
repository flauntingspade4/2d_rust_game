mod assets;
mod event_loop;
mod pine;
mod player;

use ggez;
use ggez::event;
use ggez::GameResult;
use ggez::conf::WindowSetup;

impl Init for WindowSetup {
    fn new(title: String, samples: ggez::conf::NumSamples, vsync: bool, icon: String, srgb: bool) -> Self {
        return WindowSetup { title: title, samples: samples, vsync: vsync, icon: icon, srgb: srgb}
    }
}

pub trait Init {
    fn new(title: String, samples: ggez::conf::NumSamples, vsync: bool, icon: String, srgb: bool) -> Self;
}

pub fn main() -> GameResult {
    let ready_window = WindowSetup::new("A basic shooter game".to_string(), ggez::conf::NumSamples::Zero, true, "/icon.png".to_owned(), true);
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Starting", "flauntingspade4")
        .window_setup(ready_window)
        .build()
        .unwrap();
    let mut state = event_loop::MainState::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}