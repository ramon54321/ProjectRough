use crate::Game;
use ggez::Context;
use ggez::timer::delta;

pub fn update(ctx: &Context, game: &mut Game) {
    let delta = delta(ctx).as_secs_f32();
    game.x += 100.0 * delta;
}
