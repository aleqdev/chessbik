use bevy::{math::const_vec3, prelude::*};

pub const LIGHT_INTENSITY: f32 = 48000.;
pub const LIGHT_COLOR: Color = Color::WHITE;
pub const LIGHT_DISTANCE: f32 = 15.;
pub const LIGHT_POSITIONS: [(f32, f32, f32); 6] = [
    (LIGHT_DISTANCE, 0., 0.),
    (0., LIGHT_DISTANCE, 0.),
    (0., 0., LIGHT_DISTANCE),
    (-LIGHT_DISTANCE, 0., 0.),
    (0., -LIGHT_DISTANCE, 0.),
    (0., 0., -LIGHT_DISTANCE),
];

pub const DEFAULT_CUBE_PLANE_SIZE: f32 = 1.;
pub const DEFAULT_MAGE_MESH_SIZE: f32 = 0.5;

pub const MAGE_MOVE_INDICATOR_OFFSET: f32 = 1.;
pub const MAGE_MOVE_INDICATOR_COLOR: Color = Color::rgb(1., 0.5, 1.);

pub const ROTATOR_AVAILABLE_COLOR: Color = Color::rgb(1., 1., 0.6);
pub const ROTATOR_AVAILABLE_HIGHLIGHTED_COLOR: Color = Color::rgb(1., 1., 1.);
pub const ROTATOR_ACTIVE_COLOR: Color = Color::rgb(0.7, 1., 0.7);
pub const ROTATOR_UNAVAILABLE_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

pub const MOVE_INDICATOR_OFFSET: f32 = 0.01;
pub const MOVE_INDICATOR_COLOR: Color = Color::rgb(1., 0., 1.);

pub const WS_URL: &'static str = env!("WS_URL");

pub const DEFAULT_CUBE_SCALE: Vec3 = const_vec3!([0.3, 0.3, 0.3]);

pub const PIECE_SCALE: Vec3 = const_vec3!([1., 1., 1.]);

pub const CAMERA_POSITION: Vec3 = const_vec3!([2., 2., 2.5]);
