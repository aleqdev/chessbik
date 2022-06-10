use bevy::prelude::*;

use crate::commons::PlayerTokenBuffer;

pub fn system(mut commands: Commands) {
    commands.insert_resource(PlayerTokenBuffer(None));
}
