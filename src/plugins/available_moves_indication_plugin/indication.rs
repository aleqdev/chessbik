use bevy::prelude::*;


use crate::{
    commons::SelectedPieceReference, GameRecord, plugins::cube_plugin::cube::Cube, app_assets::AppAssets
};

use super::AvailableMovesIndicator;

pub fn system(
    selected: Res<SelectedPieceReference>,
    mut indicator: ResMut<AvailableMovesIndicator>,
    record: Option<Res<GameRecord>>,
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    cube: Option<ResMut<Cube>>,
) {
    if let Some(record) = record {
        if let Some(cube) = cube {
            let board = record.board;

            if selected.is_changed() {
                match selected.into_inner().0 {
                    None => {}
                    Some(reference) => {
                        let moves = board.get_available_moves(*reference);
                        indicator.update(
                            moves.into_iter(),
                            &mut commands,
                            &app_assets,
                            cube.id
                        )
                    }
                }
            }
        } else {
            indicator.clear_indicators();
        }
    }
}
