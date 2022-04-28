use ggez::timer::delta;
use ggez::{
    input,
    input::keyboard::{is_key_pressed, KeyCode},
    Context,
};
use glam::Vec2;
use state::{Entity, Shape, State};

pub fn init(ctx: &Context, state: &mut State) {
    state.entities.insert(
        String::from("player"),
        Entity {
            shape: Shape::Dome { radius: 1.25 },
            position: Vec2::new(-5.0, 0.0),
            velocity: Vec2::default(),
        },
    );
    state.entities.insert(
        String::from("opponent"),
        Entity {
            shape: Shape::Dome { radius: 1.25 },
            position: Vec2::new(5.0, 0.0),
            velocity: Vec2::default(),
        },
    );
    state.entities.insert(
        String::from("ball"),
        Entity {
            shape: Shape::Circle { radius: 0.75 },
            position: Vec2::new(-5.0, 6.0),
            velocity: Vec2::default(),
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
    let floor = -6.0;
    let jump_velocity = 20.0;
    let downdash_acceleration = 64.0;
    let downdash_floor_buffer = 0.1;
    let slide_velocity = 12.0;
    state.entities.get_mut("player").inspect_mut(|e| {
        if is_key_pressed(ctx, KeyCode::W) {
            if e.position.y <= floor + 0.0001 {
                e.velocity.y = jump_velocity;
            }
        }
        if is_key_pressed(ctx, KeyCode::S) {
            if e.position.y > floor + downdash_floor_buffer {
                e.velocity.y = e.velocity.y - downdash_acceleration * delta;
            }
        }
        if is_key_pressed(ctx, KeyCode::A) {
            e.velocity.x = -slide_velocity;
        }
        if is_key_pressed(ctx, KeyCode::D) {
            e.velocity.x = slide_velocity;
        }
    });
    state.entities.get_mut("opponent").inspect_mut(|e| {
        if is_key_pressed(ctx, KeyCode::Up) {
            if e.position.y <= floor + 0.0001 {
                e.velocity.y = jump_velocity;
            }
        }
        if is_key_pressed(ctx, KeyCode::Down) {
            if e.position.y > floor + downdash_floor_buffer {
                e.velocity.y = e.velocity.y - downdash_acceleration * delta;
            }
        }
        if is_key_pressed(ctx, KeyCode::Left) {
            e.velocity.x = -slide_velocity;
        }
        if is_key_pressed(ctx, KeyCode::Right) {
            e.velocity.x = slide_velocity;
        }
    });
}

fn update_physics(delta: f32, state: &mut State) {
    let gravity = Vec2::new(0.0, -58.0);
    let horizontal_drag = 70.0;
    state.entities.iter_mut().for_each(|(_, entity)| {
        // Gravity
        entity.velocity = entity.velocity + gravity * delta;

        // Horizontal Drag
        entity.velocity.x = if entity.velocity.x > 0.0 {
            entity.velocity.x - horizontal_drag * delta
        } else if entity.velocity.x < 0.0 {
            entity.velocity.x + horizontal_drag * delta
        } else {
            entity.velocity.x
        };
        if entity.velocity.x < 0.8 && entity.velocity.x > -0.8 {
            entity.velocity.x = 0.0;
        }
        entity.position = entity.position + entity.velocity * delta;
    })
}

fn update_constraints(state: &mut State) {
    state.entities.get_mut("player").inspect_mut(|entity| {
        if entity.position.y < -6.0 {
            entity.position.y = -6.0;
        }
    });
    state.entities.get_mut("opponent").inspect_mut(|entity| {
        if entity.position.y < -6.0 {
            entity.position.y = -6.0;
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

