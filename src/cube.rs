use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Displayer {
    pub piece: Option<Entity>,
    pub plane: Option<Entity>,
}

pub struct Cube {
    pub id: Entity,
    pub displayers: [Displayer; 54],
}

impl Cube {
    pub fn new(id: Entity) -> Self {
        Self {
            id,
            displayers: [Displayer::default(); 54],
        }
    }
}
