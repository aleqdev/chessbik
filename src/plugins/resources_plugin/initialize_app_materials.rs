use bevy::prelude::*;

use crate::app_materials::AppMaterials;

pub fn system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AppMaterials::new_default(
        &asset_server,
        &mut materials_assets,
    ));
}
