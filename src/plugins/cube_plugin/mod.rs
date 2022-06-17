use bevy::prelude::*;

use crate::{events::UpdateCubeDisplayEvent, AppLabels};

pub mod compute_board_difference;
mod cube_display;
mod game_record_tracker;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            game_record_tracker::system
                .label(AppLabels::CubeDisplay)
                .after(AppLabels::Indication),
        );
        app.add_system(
            cube_display::system
                .label(AppLabels::CubeDisplay)
                .after(AppLabels::Indication),
        );
        app.add_event::<UpdateCubeDisplayEvent>();
    }
}
