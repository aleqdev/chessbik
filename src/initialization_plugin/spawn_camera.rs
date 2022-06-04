use bevy::prelude::*;
use smooth_bevy_cameras::{controllers::orbit::{OrbitCameraBundle, OrbitCameraController}};

pub fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert_bundle(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: Vec2::splat(0.01),
                mouse_wheel_zoom_sensitivity: 0.1,
                smoothing_weight: 0.5,
                ..default()
            },
            *crate::CAMERA_POSITION,
            Vec3::ZERO,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
                point_light: PointLight {
                    color: crate::LIGHT_COLOR,
                    intensity: crate::LIGHT_INTENSITY * 2.,
                    ..default()
                },
                ..default()
            });
        });
}
