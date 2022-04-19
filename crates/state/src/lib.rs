pub struct State {
    pub x: f32,
    pub y: f32,
}

impl State {
    pub fn new() -> State {
        State { x: 50.0, y: 50.0 }
    }
}
