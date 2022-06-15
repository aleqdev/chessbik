use bevy::prelude::*;
use bevy_mod_picking::{Hover, Selection};

use crate::{
    app_assets::AppAssets,
    commons::{CubeRotationState, MageMoveIndicatorMarker, SelectedPieceReference},
};

pub fn system(
    mut query: Query<
        (
            &Selection,
            &Hover,
            &mut Handle<StandardMaterial>,
            &MageMoveIndicatorMarker,
        ),
        Or<(Changed<Selection>, Changed<Hover>)>,
    >,
    app_assets: Res<AppAssets>,
    mut cube_rotation_state: ResMut<CubeRotationState>,
    mut selected_ref: ResMut<SelectedPieceReference>,
) {
    if let Ok((selection, hover, mut material, marker)) = query.get_single_mut() {
        if selection.selected() {
            selected_ref.0 = None;
            cube_rotation_state.is_rotating = true;
            cube_rotation_state.mage = Some(marker.0);
        } else {
            if hover.hovered() {
                *material = app_assets.selected.clone();
            } else {
                *material = app_assets.mage_move_material.clone();
            }
        }
    }
}
