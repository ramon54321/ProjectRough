use ggez::GameResult;

pub struct Game {
    pub x: f32,
    pub y: f32,
}
impl Game {
    pub fn new() -> GameResult<Game> {
        Ok(Game { x: 50.0, y: 50.0 })
    }
}
