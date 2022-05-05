use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event::run,
    event::EventHandler,
    graphics::{self, clear, present, Rect},
    Context, ContextBuilder, GameError, GameResult,
};
use logic::{init, update};
use render::render;
use state::State;

struct Game {
    state: State,
}

impl Game {
    fn new() -> Game {
        Game {
            state: State::new(),
        }
    }
}

impl EventHandler<GameError> for Game {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        render(ctx, &self.state)?;
        present(ctx)?;
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        update(ctx, &mut self.state);
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ContextBuilder::new("my_game", "ramon")
        .window_setup(WindowSetup {
            title: String::from("Project Rough"),
            vsync: true,
            samples: NumSamples::Four,
            ..Default::default()
        })
        .window_mode(WindowMode {
            width: 1280.0 * 2.0,
            height: 720.0 * 2.0,
            ..Default::default()
        });
    let (mut ctx, event_loop) = context_builder.build()?;
    let screen_ratio = 1280.0 / 720.0;
    graphics::set_screen_coordinates(
        &mut ctx,
        Rect {
            x: -8.0 * screen_ratio,
            y: 8.0,
            w: 16.0 * screen_ratio,
            h: -16.0,
        },
    )
    .expect("Failed to set screen coordinates");
    let mut game = Game::new();
    init(&ctx, &mut game.state);
    run(ctx, event_loop, game)
}
