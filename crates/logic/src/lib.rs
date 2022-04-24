use ggez::timer::delta;
use ggez::{input, Context};
use glam::Vec2;
use state::{Entity, Shape, State};

pub fn init(ctx: &Context, state: &mut State) {
    state.entities.insert(
        String::from("player"),
        Entity {
            shape: Shape::Rectangle {
                width: 40.0,
                height: 160.0,
            },
            position: Vec2::new(50.0, 50.0),
            velocity: Vec2::new(50.0, 50.0),
        },
    );
    println!("{:?}", state);
}

pub fn update(ctx: &Context, state: &mut State) {
    let delta = delta(ctx).as_secs_f32();
    let mouse_position = input::mouse::position(ctx);
    update_physics(delta, state);
}

fn update_physics(delta: f32, state: &mut State) {
    state.entities.iter_mut().for_each(|(_, entity)| {
        entity.position = entity.position + entity.velocity * delta;
    })
}