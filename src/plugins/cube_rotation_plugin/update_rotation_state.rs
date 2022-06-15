use bevy::prelude::*;

use crate::{commons::CubeRotationState, GameRecord};

pub fn system(record: Option<Res<GameRecord>>, mut rotation_state: ResMut<CubeRotationState>) {
    if record.is_none() {
        rotation_state.is_rotating = false;
    }
}
