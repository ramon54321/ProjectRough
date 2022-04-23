use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::{graphics, graphics::Color, graphics::Mesh, GameResult};
use glam::Vec2;
use state::State;

pub fn render(ctx: &mut Context, state: &State) -> GameResult {
    state.entities.iter().for_each(|entity| {
        let circle = Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Vec2::new(entity.position.x, entity.position.y),
            20.0,
            0.2,
            Color::WHITE,
        )
        .unwrap();
        graphics::draw(ctx, &circle, DrawParam::default());
    });
    Ok(())
}
