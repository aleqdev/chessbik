use bevy::prelude::*;

use crate::events::UpdateCubeDisplayEvent;

pub(self) mod compute_board_difference;
pub(self) mod cube;
mod cube_display;
mod game_record_tracker;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_record_tracker::system);
        app.add_system(cube_display::system);
        app.add_event::<UpdateCubeDisplayEvent>();
    }
}
