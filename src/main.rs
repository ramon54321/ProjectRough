use ggez::{
    event::run,
    event::EventHandler,
    graphics::{clear, present},
    Context, ContextBuilder, GameError, GameResult,
};

mod game;
mod render;
mod logic;

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
    let context_builder = ContextBuilder::new("my_game", "ramon");
    let (ctx, event_loop) = context_builder.build()?;
    let game = Game::new()?;
    run(ctx, event_loop, game)
}
