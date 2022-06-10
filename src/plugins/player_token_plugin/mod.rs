use bevy::prelude::*;

mod initialize_buffer;

pub struct PlayerTokenPlugin;

impl Plugin for PlayerTokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_buffer::system);
    }
}
