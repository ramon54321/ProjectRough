use glam::Vec2;

#[derive(Debug)]
pub struct State {
    pub entities: Vec<Entity>,
}

impl State {
    pub fn new() -> State {
        State {
            entities: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub position: Vec2,
    pub velocity: Vec2,
}
