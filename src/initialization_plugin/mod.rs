use bevy::prelude::*;

pub use crate::{
    default_cube::{DefaultCubeQuery, DefaultCubeQueryItem},
    FieldReference, PiecePosition,
};

mod get_piece_material;
mod get_piece_stl;
mod spawn_camera;
mod spawn_cube;
mod spawn_cube_side;
mod spawn_cube_side_cells;
mod spawn_lights;

pub use get_piece_material::get_piece_material;
pub use get_piece_stl::get_piece_stl;
pub use spawn_camera::spawn_camera;
pub use spawn_cube::spawn_cube;
pub use spawn_cube_side::spawn_cube_side;
pub use spawn_cube_side_cells::spawn_cube_side_cells;
pub use spawn_lights::spawn_lights;

pub struct InitializationSystemState<'m0, 's0, 'a0> {
    pub meshes: ResMut<'m0, Assets<Mesh>>,
    pub materials: ResMut<'s0, Assets<StandardMaterial>>,
    pub asset_server: Res<'a0, AssetServer>,
    pub field_refs: std::ops::Range<usize>,
}

pub fn system(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut state = InitializationSystemState {
        meshes: meshes,
        materials: materials,
        asset_server: asset_server,
        field_refs: (0..54),
    };

    spawn_camera(&mut commands);

    spawn_lights(&mut commands);

    spawn_cube(&mut commands, &mut state);
}
