use ggez::timer::delta;
use ggez::{input, Context};
use state::State;

pub fn update(ctx: &Context, state: &mut State) {
    let delta = delta(ctx).as_secs_f32();
    let mouse_position = input::mouse::position(ctx);
    state.x = mouse_position.x;
    state.y = mouse_position.y;
}
