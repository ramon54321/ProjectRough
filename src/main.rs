use ggez::{
    conf::{WindowSetup, WindowMode},
    event::run,
    event::EventHandler,
    graphics::{clear, present},
    Context, ContextBuilder, GameError, GameResult,
};

mod game;
mod logic;
mod render;

use game::Game;

impl EventHandler<GameError> for Game {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        render::render(ctx, self)?;
        present(ctx)?;
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        logic::update(ctx, self);
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ContextBuilder::new("my_game", "ramon").window_setup(WindowSetup {
        title: String::from("Project Rough"),
        vsync: true,
        ..Default::default()
    }).window_mode(WindowMode {
        width: 1280.0 * 2.0,
        height: 720.0 * 2.0,
        ..Default::default()
    });
    let (ctx, event_loop) = context_builder.build()?;
    let game = Game::new()?;
    run(ctx, event_loop, game)
}
