use std::collections::HashMap;

use glam::Vec2;

#[derive(Debug)]
pub struct State {
    pub entities: HashMap<String, Entity>,
}

impl State {
    pub fn new() -> State {
        State {
            entities: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub shape: Shape,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Debug)]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}
