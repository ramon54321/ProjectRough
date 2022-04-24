use ggez::graphics::{DrawMode, DrawParam, Rect};
use ggez::Context;
use ggez::{graphics, graphics::Color, graphics::Mesh, GameResult};
use state::{Entity, Shape, State};

pub fn render(ctx: &mut Context, state: &State) -> GameResult {
    state.entities.iter().for_each(|(_, entity)| {
        let mesh = get_entity_mesh(ctx, entity).unwrap();
        graphics::draw(ctx, &mesh, DrawParam::default());
    });
    Ok(())
}

fn get_entity_mesh(ctx: &mut Context, entity: &Entity) -> Option<Mesh> {
    match entity.shape {
        Shape::Circle { radius: r } => {
            Mesh::new_circle(ctx, DrawMode::fill(), entity.position, r, 0.2, Color::WHITE).ok()
        }
        Shape::Rectangle { .. } => Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            get_entity_bounds(&entity),
            Color::WHITE,
        )
        .ok(),
    }
}

fn get_entity_bounds(entity: &Entity) -> Rect {
    match entity.shape {
        Shape::Circle { radius: r } => Rect {
            x: entity.position.x - r,
            y: entity.position.y - r,
            w: r * 2.0,
            h: r * 2.0,
        },
        Shape::Rectangle {
            width: w,
            height: h,
        } => Rect {
            x: entity.position.x - w / 2.0,
            y: entity.position.y - w / 2.0,
            w,
            h,
        },
    }
}
