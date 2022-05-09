use ggez::graphics::{DrawMode, DrawParam, PxScale, Rect, Text, TextFragment};
use ggez::Context;
use ggez::{graphics, graphics::Color, graphics::Mesh, GameResult};
use glam::Vec2;
use state::{Entity, Shape, State};
use std::f32::consts::PI;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;
const SCREEN_DENSITY: f32 = 2.0;
const WORLD_HEIGHT: f32 = 16.0;
const WORLD_WIDTH: f32 = WORLD_HEIGHT * SCREEN_RATIO;
const WORLD_TO_SCREEN_RATIO: f32 = (SCREEN_HEIGHT / WORLD_HEIGHT) * SCREEN_DENSITY;
const WORLD_WIDTH_HALF: f32 = WORLD_WIDTH / 2.0;
const WORLD_HEIGHT_HALF: f32 = WORLD_HEIGHT / 2.0;

pub fn render(ctx: &mut Context, state: &State) -> GameResult {
    let mut score_text = Text::new(TextFragment {
        text: format!("{} : {}", state.scores.0, state.scores.1),
        scale: Some(PxScale::from(120.0)),
        ..Default::default()
    });
    score_text.set_bounds(
        Vec2::new(SCREEN_WIDTH * SCREEN_DENSITY, f32::INFINITY),
        graphics::Align::Center,
    );
    graphics::draw(
        ctx,
        &score_text,
        DrawParam::default().dest(Vec2::new(0.0, 20.0)),
    )?;

    let fps_text = Text::new(TextFragment {
        text: format!("FPS: {:.0}", ggez::timer::fps(ctx)),
        scale: Some(PxScale::from(40.0)),
        ..Default::default()
    });
    graphics::draw(
        ctx,
        &fps_text,
        DrawParam::default().dest(Vec2::new(20.0, 20.0)),
    )?;

    // Entities
    state.entities.iter().for_each(|(_, entity)| {
        let mesh = get_entity_mesh(ctx, entity).unwrap();
        graphics::draw(ctx, &mesh, DrawParam::default());
    });

    // Floor
    let floor_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        world_to_screen_rect(&Rect {
            x: -WORLD_WIDTH_HALF,
            y: -7.0,
            w: WORLD_WIDTH,
            h: -1.0,
        }),
        Color::WHITE,
    )?;
    graphics::draw(ctx, &floor_mesh, DrawParam::default())?;

    // Net
    let net_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        world_to_screen_rect(&Rect {
            x: -0.5,
            y: -7.0,
            w: 1.0,
            h: 4.0,
        }),
        Color::WHITE,
    )?;
    graphics::draw(ctx, &net_mesh, DrawParam::default())?;

    Ok(())
}

fn world_to_screen_point(point: &Vec2) -> Vec2 {
    let x = ((point.x - -WORLD_WIDTH_HALF) / WORLD_WIDTH) * SCREEN_WIDTH;
    let y = SCREEN_HEIGHT - ((point.y - -WORLD_HEIGHT_HALF) / WORLD_HEIGHT) * SCREEN_HEIGHT;
    Vec2::new(x * SCREEN_DENSITY, y * SCREEN_DENSITY)
}

fn world_to_screen_rect(rect: &Rect) -> Rect {
    let origin = world_to_screen_point(&Vec2::new(rect.x, rect.y));
    Rect {
        x: origin.x,
        y: origin.y,
        w: rect.w * WORLD_TO_SCREEN_RATIO,
        h: -rect.h * WORLD_TO_SCREEN_RATIO,
    }
}

fn get_entity_mesh(ctx: &mut Context, entity: &Entity) -> Option<Mesh> {
    match entity.shape {
        Shape::Circle { radius: r } => Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            world_to_screen_point(&entity.position),
            r * WORLD_TO_SCREEN_RATIO,
            0.1,
            Color::WHITE,
        )
        .ok(),
        Shape::Rectangle { .. } => Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            world_to_screen_rect(&get_entity_bounds_world_space(&entity)),
            Color::WHITE,
        )
        .ok(),
        Shape::Dome { radius: r } => {
            let polygon: Vec<Vec2> = create_dome_points_world_space(50)
                .iter()
                .map(|p| *p * r)
                .map(|p| p + entity.position)
                .map(|p| world_to_screen_point(&p))
                .collect();
            Mesh::new_polygon(ctx, DrawMode::fill(), &polygon, Color::WHITE).ok()
        }
    }
}

fn create_dome_points_world_space(n: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(n);
    for i in 0..n + 1 {
        let theta: f32 = PI - (i as f32 * (PI / n as f32));
        let point = Vec2::new(theta.cos(), theta.sin());
        points.push(point);
    }
    points
}

fn get_entity_bounds_world_space(entity: &Entity) -> Rect {
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
