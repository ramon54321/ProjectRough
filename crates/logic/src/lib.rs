use ggez::timer::delta;
use ggez::{input, Context};
use glam::Vec2;
use state::{Entity, Shape, State};

pub fn init(ctx: &Context, state: &mut State) {
    state.entities.insert(
        String::from("player"),
        Entity {
            shape: Shape::Dome { radius: 100.0 },
            position: Vec2::new(300.0, 300.0),
            velocity: Vec2::default(),
        },
    );
    println!("{:?}", state);
}

pub fn update(ctx: &Context, state: &mut State) {
    let delta = delta(ctx).as_secs_f32();
    let mouse_position = input::mouse::position(ctx);
    update_physics(delta, state);
    update_constraints(state);
}

fn update_physics(delta: f32, state: &mut State) {
    state.entities.iter_mut().for_each(|(_, entity)| {
        entity.position = entity.position + entity.velocity * delta;
    })
}

fn update_constraints(state: &mut State) {
    //state
    //.entities
    //.get_mut(&String::from("player"))
    //.inspect_mut(|entity| entity.position.x = 50.0);
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

