use crate::Game;
use ggez::{Context, input};
//use ggez::timer::delta;

pub fn update(ctx: &Context, game: &mut Game) {
    //let delta = delta(ctx).as_secs_f32();
    let mouse_position = input::mouse::position(ctx);
    game.x = mouse_position.x;
    game.y = mouse_position.y;
}
