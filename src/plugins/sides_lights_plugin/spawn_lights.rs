use bevy::prelude::*;

pub fn system(mut commands: Commands) {
    for position in crate::LIGHT_POSITIONS {
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::from(position)),
            point_light: PointLight {
                color: crate::LIGHT_COLOR,
                intensity: crate::LIGHT_INTENSITY,
                ..default()
            },
            ..default()
        });
    }
}
