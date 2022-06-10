use bevy::prelude::*;

use crate::commons::PlayerNameBuffer;

pub fn system(mut commands: Commands) {
    commands.insert_resource(PlayerNameBuffer("someone".to_string()));
}
