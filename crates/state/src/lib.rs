use std::collections::HashMap;

use glam::Vec2;

#[derive(Debug)]
pub struct State {
    pub entities: HashMap<String, Entity>,
    pub scores: (isize, isize),
}

impl State {
    pub fn new() -> State {
        State {
            entities: HashMap::new(),
            scores: (0, 0),
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub shape: Shape,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Dome { radius: f32 },
}
