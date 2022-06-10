use bevy::prelude::*;

use crate::app_assets::AppAssets;

pub fn system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials_assets: ResMut<Assets<StandardMaterial>>,
    mut meshes_assets: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(AppAssets::new_default(
        &asset_server,
        &mut materials_assets,
        &mut meshes_assets,
    ));
}
