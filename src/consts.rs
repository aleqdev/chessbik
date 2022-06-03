use bevy::prelude::*;

use crate::default_cube::DefaultCubeQuery;

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

lazy_static::lazy_static! {
    pub static ref DEFAULT_CUBE_QUERIES: [&'static DefaultCubeQuery; 6] = [
        &*crate::default_cube::TOP,
        &*crate::default_cube::LEFT,
        &*crate::default_cube::FORWARD,
        &*crate::default_cube::RIGHT,
        &*crate::default_cube::BACK,
        &*crate::default_cube::BOTTOM,
    ];

    pub static ref DEFAULT_CUBE_SCALE: Vec3 = Vec3::splat(0.3);

    pub static ref PIECE_SCALE: Vec3 = Vec3::splat(0.02);

    pub static ref CAMERA_POSITION: Vec3 = Vec3::new(7., 3., 4.);
}
