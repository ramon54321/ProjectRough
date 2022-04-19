use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::{graphics, graphics::Color, graphics::Mesh, GameResult};
use glam::Vec2;
use state::State;

pub fn render(ctx: &mut Context, state: &State) -> GameResult {
    let circle = Mesh::new_circle(
        ctx,
        ggez::graphics::DrawMode::fill(),
        Vec2::new(state.x, state.y),
        100.0,
        0.2,
        Color::WHITE,
    )?;
    graphics::draw(ctx, &circle, DrawParam::default())?;
    Ok(())
}
