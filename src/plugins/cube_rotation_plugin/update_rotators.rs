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
    for (selection, hover, CubeRotator(rots, is_active), mut mat) in query.iter_mut() {
        if selection.selected() {
            rotation_state.selected_rotator = Some(rots.clone());
        }

        if *is_active {
            *mat = app_assets.rotator_active_material.clone();
        } else {
            if hover.hovered() {
                *mat = app_assets.rotator_available_highlighted_material.clone();
            } else {
                *mat = app_assets.rotator_available_material.clone();
            }
        }
    }
}

pub fn highlight_system(
    mut query: Query<
        (
            &mut CubeRotator,
            &mut Handle<StandardMaterial>,
        )
    >,
    rotation_state: ResMut<CubeRotationState>,
    app_assets: Res<AppAssets>,
) {
    if rotation_state.is_changed() {
        if let Some(rots) = rotation_state.selected_rotator {
            for (mut rotator, mut mat) in query.iter_mut() {
                if rotator.0.iter().all(|x| rots.contains(x)) {
                    rotator.1 = true;
                    *mat = app_assets.rotator_active_material.clone();
                } else {
                    rotator.1 = false;
                    *mat = app_assets.rotator_available_material.clone();
                }
            }
        }
    }
}
