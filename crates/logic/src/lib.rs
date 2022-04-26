use ggez::timer::delta;
use ggez::{input, Context};
use glam::Vec2;
use state::{Entity, Shape, State};

pub fn init(ctx: &Context, state: &mut State) {
    state.entities.insert(
        String::from("player"),
        Entity {
            shape: Shape::Dome { radius: 1.0 },
            position: Vec2::new(0.0, 5.0),
            velocity: Vec2::default(),
            impulse: Vec2::default(),
        },
    );
    println!("{:?}", state);
}

pub fn update(ctx: &Context, state: &mut State) {
    let delta = delta(ctx).as_secs_f32();
    let mouse_position = input::mouse::position(ctx);
    update_input(ctx, delta, state);
    update_physics(delta, state);
    update_constraints(state);
}

fn update_input(ctx: &Context, delta: f32, state: &mut State) {
    if (input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W)) {
        state
            .entities
            .get_mut("player")
            .inspect_mut(|e| e.impulse = Vec2::new(0.0, 10.0));
    }
}

fn update_physics(delta: f32, state: &mut State) {
    let gravity = Vec2::new(0.0, -9.81);
    state.entities.iter_mut().for_each(|(_, entity)| {
        if entity.impulse != Vec2::default() {
            entity.velocity = entity.velocity + entity.impulse / delta;
            entity.impulse = Vec2::default();
        }
        entity.velocity = entity.velocity + gravity * delta;
        entity.position = entity.position + entity.velocity * delta;
    })
}

fn update_constraints(state: &mut State) {
    state.entities.get_mut("player").inspect_mut(|entity| {
        if entity.position.y < 0.0 {
            entity.position.y = 0.0;
        }
    });
}

trait InspectMut<T> {
    fn inspect_mut<F>(&mut self, callback: F)
    where
        F: FnMut(&mut T);
}

impl<T> InspectMut<T> for Option<T> {
    fn inspect_mut<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut T),
    {
        self.iter_mut().for_each(|i| callback(i));
    }
}

