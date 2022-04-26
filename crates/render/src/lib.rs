use ggez::graphics::{DrawMode, DrawParam, Rect};
use ggez::Context;
use ggez::{graphics, graphics::Color, graphics::Mesh, GameResult};
use glam::Vec2;
use state::{Entity, Shape, State};
use std::f32::consts::PI;

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
        Shape::Dome { radius: r } => {
            let polygon: Vec<Vec2> = create_dome_points(50)
                .iter()
                .map(|p| *p * r)
                .map(|p| p + entity.position)
                .collect();
            Mesh::new_polygon(ctx, DrawMode::fill(), &polygon, Color::WHITE).ok()
        }
    }
}

fn create_dome_points(n: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(n);
    for i in 0..n + 1 {
        let theta: f32 = PI - (i as f32 * (PI / n as f32));
        let point = Vec2::new(theta.cos(), -theta.sin());
        points.push(point);
    }
    points
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
        Shape::Dome { radius: r } => Rect {
            x: entity.position.x - r,
            y: entity.position.y - r,
            w: r * 2.0,
            h: r,
        },
    }
}
