use bevy::{
    math::{const_quat, const_vec3},
    prelude::*,
};

use chessbik_board::CubeRotation;

use crate::{
    app_assets::AppAssets,
    commons::{AvailableMovesStorage, CubeRotationState, CubeRotator},
    consts,
};

pub fn system(
    cube_rotation_state: ResMut<CubeRotationState>,
    app_assets: Res<AppAssets>,
    mut rotators_parent: Local<Option<Entity>>,
    mut commands: Commands,
    moves: Res<AvailableMovesStorage>,
) {
    if cube_rotation_state.is_changed() {
        if let Some(parent) = *rotators_parent {
            commands.entity(parent).despawn_recursive();
            *rotators_parent = None;
        }

        if cube_rotation_state.is_rotating {
            *rotators_parent = Some(
                commands
                    .spawn()
                    .insert_bundle(TransformBundle {
                        local: Transform::from_scale(consts::DEFAULT_CUBE_SCALE),
                        ..default()
                    })
                    .with_children(|parent| {
                        for (vec3, quat, rots) in ROTATORS.iter() {
                            let av = rots
                                .iter()
                                .any(|r| moves.0.iter().any(|m| m.eq_rotation(r)));

                            let material = if av {
                                app_assets.rotator_available_material.clone()
                            } else {
                                app_assets.rotator_unavailable_material.clone()
                            };

                            let mut e = parent.spawn_bundle(PbrBundle {
                                mesh: app_assets.rotator_mesh.clone(),
                                material,
                                transform: Transform {
                                    rotation: *quat,
                                    translation: *vec3,
                                    ..Transform::identity()
                                },
                                ..default()
                            });

                            e.insert(CubeRotator(rots.clone()));

                            if av {
                                e.insert_bundle(bevy_mod_picking::PickableBundle::default());
                            }
                        }
                    })
                    .id(),
            )
        }
    }
}

use CubeRotation::*;

const ROTATORS: [(Vec3, Quat, [CubeRotation; 3]); 36] = [
    (
        const_vec3!([-1., 2., -2.]),
        const_quat!([1., 0., 0., 0.]),
        [U, U2, U_P],
    ),
    (
        const_vec3!([-1., 2., 2.]),
        const_quat!([0.7071068, 0., 0., -0.7071068]),
        [U, U2, U_P],
    ),
    (
        const_vec3!([-1., -2., -2.]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
        [U, U2, U_P],
    ),
    (
        const_vec3!([-1., -2., 2.]),
        const_quat!([0., 0., 0., 1.]),
        [U, U2, U_P],
    ),
    (
        const_vec3!([0., 2., -2.]),
        const_quat!([1., 0., 0., 0.]),
        [E, E2, E_P],
    ),
    (
        const_vec3!([0., 2., 2.]),
        const_quat!([0.7071068, 0., 0., -0.7071068]),
        [E, E2, E_P],
    ),
    (
        const_vec3!([0., -2., -2.]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
        [E, E2, E_P],
    ),
    (
        const_vec3!([0., -2., 2.]),
        const_quat!([0., 0., 0., 1.]),
        [E, E2, E_P],
    ),
    (
        const_vec3!([1., 2., -2.]),
        const_quat!([1., 0., 0., 0.]),
        [D, D2, D_P],
    ),
    (
        const_vec3!([1., 2., 2.]),
        const_quat!([0.7071068, 0., 0., -0.7071068]),
        [D, D2, D_P],
    ),
    (
        const_vec3!([1., -2., -2.]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
        [D, D2, D_P],
    ),
    (
        const_vec3!([1., -2., 2.]),
        const_quat!([0., 0., 0., 1.]),
        [D, D2, D_P],
    ),
    (
        const_vec3!([-2., 2., -1.]),
        const_quat!([0.7071068, 0., -0.7071068, 0.]),
        [L, L2, L_P],
    ),
    (
        const_vec3!([2., 2., -1.]),
        const_quat!([0.7071068, 0., 0.7071068, 0.]),
        [L, L2, L_P],
    ),
    (
        const_vec3!([-2., -2., -1.]),
        const_quat!([0., 0.7071068, 0., -0.7071068]),
        [L, L2, L_P],
    ),
    (
        const_vec3!([2., -2., -1.]),
        const_quat!([0., 0.7071068, 0., 0.7071068]),
        [L, L2, L_P],
    ),
    (
        const_vec3!([-2., 2., 0.]),
        const_quat!([0.7071068, 0., -0.7071068, 0.]),
        [M, M2, M_P],
    ),
    (
        const_vec3!([2., 2., 0.]),
        const_quat!([0.7071068, 0., 0.7071068, 0.]),
        [M, M2, M_P],
    ),
    (
        const_vec3!([-2., -2., 0.]),
        const_quat!([0., 0.7071068, 0., -0.7071068]),
        [M, M2, M_P],
    ),
    (
        const_vec3!([2., -2., 0.]),
        const_quat!([0., 0.7071068, 0., 0.7071068]),
        [M, M2, M_P],
    ),
    (
        const_vec3!([-2., 2., 1.]),
        const_quat!([0.7071068, 0., -0.7071068, 0.]),
        [R, R2, R_P],
    ),
    (
        const_vec3!([2., 2., 1.]),
        const_quat!([0.7071068, 0., 0.7071068, 0.]),
        [R, R2, R_P],
    ),
    (
        const_vec3!([-2., -2., 1.]),
        const_quat!([0., 0.7071068, 0., -0.7071068]),
        [R, R2, R_P],
    ),
    (
        const_vec3!([2., -2., 1.]),
        const_quat!([0., 0.7071068, 0., 0.7071068]),
        [R, R2, R_P],
    ),
    (
        const_vec3!([-2., 1., 2.]),
        const_quat!([0., 0., 0.7071068, -0.7071068]),
        [F, F2, F_P],
    ),
    (
        const_vec3!([2., 1., 2.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
        [F, F2, F_P],
    ),
    (
        const_vec3!([-2., 1., -2.]),
        const_quat!([0.7071068, 0.7071068, 0., 0.]),
        [F, F2, F_P],
    ),
    (
        const_vec3!([2., 1., -2.]),
        const_quat!([0.7071068, -0.7071068, 0., 0.]),
        [F, F2, F_P],
    ),
    (
        const_vec3!([-2., 0., 2.]),
        const_quat!([0., 0., 0.7071068, -0.7071068]),
        [S, S2, S_P],
    ),
    (
        const_vec3!([2., 0., 2.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
        [S, S2, S_P],
    ),
    (
        const_vec3!([-2., 0., -2.]),
        const_quat!([0.7071068, 0.7071068, 0., 0.]),
        [S, S2, S_P],
    ),
    (
        const_vec3!([2., 0., -2.]),
        const_quat!([0.7071068, -0.7071068, 0., 0.]),
        [S, S2, S_P],
    ),
    (
        const_vec3!([-2., -1., 2.]),
        const_quat!([0., 0., 0.7071068, -0.7071068]),
        [B, B2, B_P],
    ),
    (
        const_vec3!([2., -1., 2.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
        [B, B2, B_P],
    ),
    (
        const_vec3!([-2., -1., -2.]),
        const_quat!([0.7071068, 0.7071068, 0., 0.]),
        [B, B2, B_P],
    ),
    (
        const_vec3!([2., -1., -2.]),
        const_quat!([0.7071068, -0.7071068, 0., 0.]),
        [B, B2, B_P],
    ),
];
