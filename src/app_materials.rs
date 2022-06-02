use bevy::prelude::*;

pub type MaterialTy = Handle<StandardMaterial>;

pub struct MaterialsForSides {
    pub top: MaterialTy,
    pub bottom: MaterialTy,
    pub left: MaterialTy,
    pub right: MaterialTy,
    pub forward: MaterialTy,
    pub back: MaterialTy,
}
pub struct CellsMaterials {
    pub default: MaterialsForSides,
    pub highlighted: MaterialsForSides,
}
pub struct PiecesMaterials {
    pub default_white: MaterialTy,
    pub default_black: MaterialTy,
    pub highlighted_white: MaterialTy,
    pub highlighted_black: MaterialTy,
}
pub struct AppMaterials {
    pub cells: CellsMaterials,
    pub pieces: PiecesMaterials,
    pub selected: MaterialTy,
}

impl AppMaterials {
    pub fn new_default(
        asset_server: &Res<AssetServer>,
        materials_assets: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            cells: crate::app_materials::CellsMaterials {
                default: crate::app_materials::MaterialsForSides {
                    top: make_side_material(
                        Color::rgb(1., 0.678, 0.678),
                        asset_server,
                        materials_assets,
                    ),
                    bottom: make_side_material(
                        Color::rgb(0.741, 0.698, 1.),
                        asset_server,
                        materials_assets,
                    ),
                    left: make_side_material(
                        Color::rgb(0.792, 1., 0.749),
                        asset_server,
                        materials_assets,
                    ),
                    right: make_side_material(
                        Color::rgb(0.8, 0.854, 0.529),
                        asset_server,
                        materials_assets,
                    ),
                    forward: make_side_material(
                        Color::rgb(0.607, 0.964, 1.),
                        asset_server,
                        materials_assets,
                    ),
                    back: make_side_material(
                        Color::rgb(1., 0.839, 0.647),
                        asset_server,
                        materials_assets,
                    ),
                },
                highlighted: crate::app_materials::MaterialsForSides {
                    top: make_side_material(
                        Color::rgb(1., 0.778, 0.778),
                        asset_server,
                        materials_assets,
                    ),
                    bottom: make_side_material(
                        Color::rgb(0.841, 0.798, 1.),
                        asset_server,
                        materials_assets,
                    ),
                    left: make_side_material(
                        Color::rgb(0.892, 1., 0.849),
                        asset_server,
                        materials_assets,
                    ),
                    right: make_side_material(
                        Color::rgb(0.9, 0.954, 0.729),
                        asset_server,
                        materials_assets,
                    ),
                    forward: make_side_material(
                        Color::rgb(0.707, 1., 1.),
                        asset_server,
                        materials_assets,
                    ),
                    back: make_side_material(
                        Color::rgb(1., 0.939, 0.747),
                        asset_server,
                        materials_assets,
                    ),
                },
            },
            pieces: crate::app_materials::PiecesMaterials {
                default_white: make_piece_material(Color::rgb(0.45, 0.44, 0.5), materials_assets),
                default_black: make_piece_material(Color::rgb(0.12, 0.1, 0.1), materials_assets),
                highlighted_white: make_piece_material(
                    Color::rgb(0.65, 0.64, 0.6),
                    materials_assets,
                ),
                highlighted_black: make_piece_material(
                    Color::rgb(0.32, 0.2, 0.2),
                    materials_assets,
                ),
            },
            selected: make_side_material(Color::rgb(0.5, 1., 1.), asset_server, materials_assets),
        }
    }
}

pub fn make_side_material(
    color: Color,
    asset_server: &Res<AssetServer>,
    materials_assets: &mut ResMut<Assets<StandardMaterial>>,
) -> crate::app_materials::MaterialTy {
    let mut mat: StandardMaterial = color.into();
    mat.unlit = true;
    mat.cull_mode = None;
    mat.base_color_texture = Some(asset_server.load("side.png"));
    materials_assets.add(mat)
}

pub fn make_piece_material(
    color: Color,
    materials_assets: &mut ResMut<Assets<StandardMaterial>>,
) -> crate::app_materials::MaterialTy {
    let mut mat: StandardMaterial = color.into();
    mat.metallic = 0.9;
    mat.reflectance = 0.2;
    materials_assets.add(mat)
}
