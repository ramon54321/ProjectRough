use std::ops::Mul;

use ggez::timer::delta;
use ggez::{
    input,
    input::keyboard::{is_key_pressed, KeyCode},
    Context,
};
use glam::{bool, Vec2};
use state::{Entity, Shape, State};

const MAXIMUM_FRAME_SECS: f32 = 1.0 / 15.0;
const FLOOR_HEIGHT: f32 = -7.0;
const DOME_RADIUS: f32 = 1.75;
const BALL_RADIUS: f32 = 0.75;

pub fn init(ctx: &Context, state: &mut State) {
    state.entities.insert(
        String::from("player"),
        Entity {
            shape: Shape::Dome {
                radius: DOME_RADIUS,
            },
            position: Vec2::new(-5.0, -6.0),
            velocity: Vec2::default(),
        },
    );
    state.entities.insert(
        String::from("opponent"),
        Entity {
            shape: Shape::Dome {
                radius: DOME_RADIUS,
            },
            position: Vec2::new(5.0, -6.0),
            velocity: Vec2::default(),
        },
    );
    state.entities.insert(
        String::from("ball"),
        Entity {
            shape: Shape::Circle {
                radius: BALL_RADIUS,
            },
            position: Vec2::new(-5.0, -2.0),
            velocity: Vec2::default(),
        },
    );
    println!("{:?}", state);
}

pub fn update(ctx: &Context, state: &mut State) {
    let delta = f32::min(MAXIMUM_FRAME_SECS, delta(ctx).as_secs_f32());
    let mouse_position = input::mouse::position(ctx);
    update_input(ctx, delta, state);
    update_physics(delta, state);
    update_constraints(state);
    update_collisions(delta, state);
    update_constraints(state);
}

//fn score_ball(state: &mut State) -> Result<(), String> {
//let ball_position = state.entities.get("ball").map(|ball| ball.position).ok_or("CAN_NOT_FIND_BALL")?;
//if ball_position.x < 0.0 {
//state.scores.0 = state.scores.0 + 1;
//} else {
//state.scores.1 = state.scores.1 + 1;
//}
//Ok(())
//}

fn update_input(ctx: &Context, delta: f32, state: &mut State) {
    let jump_velocity = 20.0;
    let downdash_acceleration = 64.0;
    let downdash_floor_buffer = 0.1;
    let slide_velocity = 20.0;
    if 5 > 3 {}
    state.entities.get_mut("player").inspect_mut(|e| {
        if is_key_pressed(ctx, KeyCode::W) {
            if e.position.y <= FLOOR_HEIGHT + 0.0001 {
                e.velocity.y = jump_velocity;
            }
        }
        if is_key_pressed(ctx, KeyCode::S) {
            if e.position.y > FLOOR_HEIGHT + downdash_floor_buffer {
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
            if e.position.y <= FLOOR_HEIGHT + 0.0001 {
                e.velocity.y = jump_velocity;
            }
        }
        if is_key_pressed(ctx, KeyCode::Down) {
            if e.position.y > FLOOR_HEIGHT + downdash_floor_buffer {
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
    let gravity = Vec2::new(0.0, -45.0);
    let horizontal_drag = 125.0;
    state.entities.iter_mut().for_each(|(name, entity)| {
        // Gravity
        entity.velocity = entity.velocity + gravity * delta;

        // Horizontal Drag
        if name != "ball" {
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
        }

        // Apply velocity
        entity.position = entity.position + entity.velocity * delta;
    });
}

fn update_constraints(state: &mut State) {
    state.entities.get_mut("player").inspect_mut(|entity| {
        if entity.position.y < FLOOR_HEIGHT {
            entity.position.y = FLOOR_HEIGHT;
            entity.velocity.y = 0.0;
        }
        if entity.position.x < -12.5 {
            entity.position.x = -12.5;
            entity.velocity.x = 0.0;
        }
        if entity.position.x > -2.0 {
            entity.position.x = -2.0;
            entity.velocity.x = 0.0;
        }
    });
    state.entities.get_mut("opponent").inspect_mut(|entity| {
        if entity.position.y < FLOOR_HEIGHT {
            entity.position.y = FLOOR_HEIGHT;
            entity.velocity.y = 0.0;
        }
        if entity.position.x > 12.5 {
            entity.position.x = 12.5;
            entity.velocity.x = 0.0;
        }
        if entity.position.x < 2.0 {
            entity.position.x = 2.0;
            entity.velocity.x = 0.0;
        }
    });
    state.entities.get_mut("ball").inspect_mut(|entity| {
        if entity.position.y < FLOOR_HEIGHT + BALL_RADIUS {
            if entity.position.x > 0.0 {
                state.scores.0 = state.scores.0 + 1;
            } else {
                state.scores.1 = state.scores.1 + 1;
            }
            entity.position = Vec2::new(-5.0, -3.0);
            entity.velocity = Vec2::default();
        }
        if entity.position.x > 12.0 {
            entity.position.x = 12.0;
            entity.velocity.x = 0.0;
        }
        if entity.position.x < -12.0 {
            entity.position.x = -12.0;
            entity.velocity.x = 0.0;
        }
    });
}

fn update_collisions(delta: f32, state: &mut State) {
    let player_position = state.entities.get("player").map(|e| e.position).unwrap();
    let player_velocity = state.entities.get("player").map(|e| e.velocity).unwrap();
    let opponent_position = state.entities.get("opponent").map(|e| e.position).unwrap();
    let opponent_velocity = state.entities.get("opponent").map(|e| e.velocity).unwrap();

    let ball = state.entities.get_mut("ball").unwrap();
    let vec_to_player = player_position - ball.position;
    let vec_to_opponent = opponent_position - ball.position;

    if vec_to_player.length() < DOME_RADIUS + BALL_RADIUS {
        update_ball_collision(ball, &vec_to_player, &player_position, &player_velocity);
    }
    if vec_to_opponent.length() < DOME_RADIUS + BALL_RADIUS {
        update_ball_collision(
            ball,
            &vec_to_opponent,
            &opponent_position,
            &opponent_velocity,
        );
    }
}

fn update_ball_collision(
    ball: &mut Entity,
    vec_to_other: &Vec2,
    other_position: &Vec2,
    other_velocity: &Vec2,
) {
    let normal_vec = vec_to_other
        .clone()
        .mul(-1.0)
        .normalize()
        .ensuring(|a| !a.is_nan(), || Vec2::new(0.0, 1.0));
    let tangent_vec = Vec2::new(-normal_vec.y, normal_vec.x).normalize();
    ball.position = *other_position + normal_vec * (DOME_RADIUS + BALL_RADIUS + 0.001);
    let relative_velocity = Vec2::new(
        ball.velocity.x - other_velocity.x,
        ball.velocity.y - other_velocity.y,
    );
    let tangent_velocity = tangent_vec * relative_velocity.dot(tangent_vec);
    let perpendicular_velocity = relative_velocity - tangent_velocity;
    let reflected_velocity = ball.velocity - 2.0 * perpendicular_velocity;
    let reflected_velocity = if reflected_velocity.length() > 30.0 {
        reflected_velocity.normalize_or_zero() * 30.0
    } else if reflected_velocity.length() < 15.0 {
        reflected_velocity.normalize_or_zero() * 15.0
    } else {
        reflected_velocity
    };
    ball.velocity = reflected_velocity * 0.95;
}

trait Ensuring {
    fn ensuring<P, D>(&mut self, predicate: P, default: D) -> Self
    where
        P: Fn(&Self) -> bool,
        D: Fn() -> Self;
}

impl<T: Clone> Ensuring for T {
    fn ensuring<P, D>(&mut self, predicate: P, default: D) -> T
    where
        P: Fn(&T) -> bool,
        D: Fn() -> T,
    {
        if predicate(&self) {
            self.clone()
        } else {
            default()
        }
    }
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
