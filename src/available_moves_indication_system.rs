use bevy::prelude::*;
use chessbik_board::Board;
use chessbik_commons::Cell;

use crate::{
    commons::{self, SelectedPieceReference},
    AvailableMovesIndicator,
};

pub fn system(
    selected: Res<SelectedPieceReference>,
    mut indicator: ResMut<AvailableMovesIndicator>,
    board: Res<Board<Cell>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube: Query<Entity, With<commons::CubeMarker>>,
) {
    if selected.is_changed() {
        match selected.into_inner().0 {
            None => {}
            Some(reference) => {
                let moves = board.get_available_moves(*reference);
                indicator.update(
                    moves.into_iter(),
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    cube.single(),
                )
            }
        }
    }
}
