mod ai_type;
mod assets;
mod event_loop;
mod player;

use ggez;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::GameResult;

pub fn main() -> GameResult {
    let ready_window = WindowSetup {
        title: "A basic shooter game".to_string(),
        samples: ggez::conf::NumSamples::Zero,
        vsync: true,
        icon: "/icon.png".to_string(),
        srgb: true,
    };
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Starting", "flauntingspade4")
        .window_setup(ready_window)
        .build()
        .unwrap();
    let mut state = event_loop::MainState::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
