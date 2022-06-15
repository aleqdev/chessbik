use bevy::prelude::*;

use crate::{
    commons::{CubeDisplayerMarker, CubeRotationState},
    cube_transform, BoardReference,
};

pub fn system(
    cube_rotation_state: Res<CubeRotationState>,
    mut query: Query<(&mut Transform, &BoardReference), With<CubeDisplayerMarker>>,
) {
    if cube_rotation_state.is_changed() {
        for (mut t, r) in query.iter_mut() {
            let mut transform = {
                let (pos, quat) = cube_transform::transform(**r);
                Transform::from_translation(pos).with_rotation(quat)
            };
            if cube_rotation_state.is_rotating {
                transform.translation += transform.up();
            }
            *t = transform;
        }
    }
}
