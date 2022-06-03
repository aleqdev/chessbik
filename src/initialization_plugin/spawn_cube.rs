use bevy::prelude::*;

use crate::commons;

pub fn spawn_cube(commands: &mut Commands, state: &mut super::InitializationSystemState) {
    let sides = crate::DEFAULT_CUBE_QUERIES
        .iter()
        .map(|f| {
            let side = super::spawn_cube_side(commands, state, f);
            commands
                .entity(side)
                .insert_bundle(TransformBundle::default())
                .id()
        })
        .collect::<Vec<_>>();
    commands
        .spawn()
        .insert_bundle(TransformBundle {
            local: Transform::from_scale(*crate::DEFAULT_CUBE_SCALE),
            ..default()
        })
        .insert(commons::CubeMarker)
        .push_children(sides.as_slice());
}
