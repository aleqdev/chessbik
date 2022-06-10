use bevy::prelude::*;

use crate::commons::JoinGameBuffer;

pub fn system(mut commands: Commands) {
    commands.insert_resource(JoinGameBuffer("".to_string()));
}
