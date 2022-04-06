use crate::game::Game;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::{graphics, graphics::Color, graphics::Mesh, GameResult};
use glam::Vec2;

pub fn render(ctx: &mut Context, game: &Game) -> GameResult {
    let circle = Mesh::new_circle(
        ctx,
        ggez::graphics::DrawMode::fill(),
        Vec2::new(game.x, game.y),
        100.0,
        0.2,
        Color::WHITE,
    )?;
    graphics::draw(ctx, &circle, DrawParam::default())?;
    Ok(())
}
