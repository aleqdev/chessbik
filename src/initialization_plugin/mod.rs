use std::ops::Range;

use bevy::prelude::*;

use crate::{
    app_materials::AppMaterials, commons::SelectedPieceReference, AvailableMovesIndicator,
    Board2CubeTransforms,
};
pub use crate::{
    default_cube::{DefaultCubeQuery, DefaultCubeQueryItem},
    BoardReference
};

mod spawn_camera;
mod spawn_cube;
mod spawn_cube_side;
mod spawn_cube_side_cells;
mod spawn_lights;

pub use spawn_camera::spawn_camera;
pub use spawn_cube::spawn_cube;
pub use spawn_cube_side::spawn_cube_side;
pub use spawn_cube_side_cells::spawn_cube_side_cells;
pub use spawn_lights::spawn_lights;

pub struct InitializationSystemState<'m0, 'a0> {
    pub meshes: ResMut<'m0, Assets<Mesh>>,
    pub asset_server: Res<'a0, AssetServer>,
    pub field_refs: Range<usize>,
    pub materials: AppMaterials,
    pub transforms: Board2CubeTransforms,
}

pub fn system(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials_assets: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let materials =
        crate::app_materials::AppMaterials::new_default(&asset_server, &mut materials_assets);

    let mut state = InitializationSystemState {
        meshes: meshes,
        asset_server: asset_server,
        field_refs: (0..54),
        materials,
        transforms: default(),
    };

    spawn_camera(&mut commands);

    spawn_lights(&mut commands);

    spawn_cube(&mut commands, &mut state);

    commands.insert_resource(state.materials);
    commands.insert_resource(state.transforms);

    commands.init_resource::<AvailableMovesIndicator>();
    commands.init_resource::<SelectedPieceReference>();
}
