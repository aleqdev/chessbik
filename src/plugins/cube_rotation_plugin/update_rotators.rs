use bevy::prelude::*;
use bevy_mod_picking::{Hover, Selection};

use crate::{
    app_assets::AppAssets,
    commons::{CubeRotationState, CubeRotator},
};

pub fn system(
    mut query: Query<
        (
            &Selection,
            &Hover,
            &CubeRotator,
            &mut Handle<StandardMaterial>,
        ),
        Or<(Changed<Selection>, Changed<Hover>)>,
    >,
    mut rotation_state: ResMut<CubeRotationState>,
    app_assets: Res<AppAssets>,
) {
    for (selection, hover, CubeRotator(rots), mut mat) in query.iter_mut() {
        if selection.selected() {
            rotation_state.selected_rotator = Some(rots.clone());
            return;
        }

        if hover.hovered() {
            *mat = app_assets.rotator_available_highlighted_material.clone();
        } else {
            *mat = app_assets.rotator_available_material.clone();
        }
    }
}
