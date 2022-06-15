use bevy::prelude::*;

use crate::{
    app_assets::AppAssets,
    commons::{AvailableMovesStorage, CubeRotationState, SelectedPieceReference},
    Cube, GameRecord,
};

use super::AvailableMovesIndicator;

pub fn system(
    selected: Res<SelectedPieceReference>,
    mut indicator: ResMut<AvailableMovesIndicator>,
    record: Option<Res<GameRecord>>,
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    cube: Option<ResMut<Cube>>,
    mut moves: ResMut<AvailableMovesStorage>,
    rotation_state: Res<CubeRotationState>,
) {
    if rotation_state.is_rotating {
        return;
    }

    if let Some(record) = record {
        if let Some(cube) = cube {
            let board = record.board;

            if selected.is_changed() {
                match selected.0 {
                    None => {
                        indicator.update(&mut commands, app_assets.as_ref(), cube.id, &vec![]);
                    }
                    Some(reference) => {
                        moves.0 = board.get_available_moves(*reference);
                        indicator.update(&mut commands, app_assets.as_ref(), cube.id, &moves.0)
                    }
                };
            }
        } else {
            indicator.clear_indicators();
        }
    }
}
