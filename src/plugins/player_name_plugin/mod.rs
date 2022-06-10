use bevy::prelude::*;

mod initialize_buffer;

pub struct PlayerNamePlugin;

impl Plugin for PlayerNamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_buffer::system);
    }
}
