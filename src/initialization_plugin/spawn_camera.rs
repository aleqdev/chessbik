use bevy::prelude::*;

pub fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(
            Transform::from_translation(*crate::CAMERA_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
        )
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_orbit_controls::OrbitCamera {
            rotate_button: MouseButton::Right,
            pan_button: MouseButton::Middle,
            rotate_sensitivity: 0.8,
            pan_sensitivity: 0.,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
                point_light: PointLight {
                    color: crate::LIGHT_COLOR,
                    intensity: crate::LIGHT_INTENSITY * 2.,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
