use crate::{
    app_assets::AppAssets,
    commons::{CubeRotationState, MageMoveIndicatorMarker, SelectedPieceReference},
    cube_transform, Cube, GameRecord,
};
use bevy::prelude::*;
use chessbik_board::PieceTy;

pub fn system(
    selected_ref: Res<SelectedPieceReference>,
    record: Option<Res<GameRecord>>,
    app_assets: Res<AppAssets>,
    mut commands: Commands,
    mut move_plane_reference: Local<Option<Entity>>,
    cube: Option<ResMut<Cube>>,
    rotation_state: Res<CubeRotationState>,
) {
    let mut despawn_ref = || {
        if let Some(r) = *move_plane_reference {
            commands.entity(r).despawn();
            *move_plane_reference = None;
        }
    };

    if rotation_state.is_changed() {
        if rotation_state.is_rotating {
            despawn_ref();
        }
    }

    if !selected_ref.is_changed() {
        return;
    }

    if record.is_none() {
        return;
    }
    let record = record.unwrap();

    if cube.is_none() {
        *move_plane_reference = None;
        return;
    };
    let cube = cube.unwrap();

    if selected_ref.0.is_none() {
        despawn_ref();
        return;
    }

    let r = selected_ref.0.unwrap();

    if record
        .board
        .at(*r)
        .piece
        .map(|p| p.ty == PieceTy::MAGE)
        .unwrap_or(false)
    {
        if move_plane_reference.is_some() {
            return;
        }

        let (vec3, quat) = cube_transform::transform(*r);
        let mut transform = Transform::from_translation(vec3).with_rotation(quat);

        transform.translation += transform.up() * crate::MAGE_MOVE_INDICATOR_OFFSET;

        commands.entity(cube.id).with_children(|parent| {
            *move_plane_reference = Some(
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: app_assets.mage_move_mesh.clone(),
                        material: app_assets.mage_move_material.clone(),
                        transform,
                        ..default()
                    })
                    .insert(MageMoveIndicatorMarker(*r))
                    .insert_bundle(bevy_mod_picking::PickableBundle::default())
                    .id(),
            );
        });
    } else {
        despawn_ref()
    }
}
