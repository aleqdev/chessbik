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

pub const MOVE_INDICATOR_OFFSET: f32 = 0.01;
pub const MOVE_INDICATOR_COLOR: Color = Color::rgb(1., 0., 1.);

pub const WS_URL: &'static str = "";

pub const DEFAULT_CUBE_SCALE: Vec3 = const_vec3!([0.3, 0.3, 0.3]);

pub const PIECE_SCALE: Vec3 = const_vec3!([0.02, 0.02, 0.02]);

pub const CAMERA_POSITION: Vec3 = const_vec3!([2., 2., 2.5]);
