use bevy::prelude::*;

use crate::{
    commons::{self, SelectedPieceReference},
    AvailableMovesIndicator, Field, Field2CubeTransforms, GetAvailableMoves,
};

pub fn system(
    selected: Res<SelectedPieceReference>,
    mut indicator: ResMut<AvailableMovesIndicator>,
    field: Res<Field>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transforms: Res<Field2CubeTransforms>,
    cube: Query<Entity, With<commons::CubeMarker>>,
) {
    if selected.is_changed() {
        println!("updating");
        match selected.into_inner().0 {
            None => {}
            Some(reference) => {
                let moves = field.get_available_moves(*reference);
                indicator.update(
                    moves.into_iter(),
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    &transforms,
                    cube.single(),
                )
            }
        }
    }
}
