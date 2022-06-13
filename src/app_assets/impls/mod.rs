use bevy::prelude::*;

pub use super::*;

impl AppAssets {
    pub fn new_default(
        asset_server: &Res<AssetServer>,
        materials_assets: &mut ResMut<Assets<StandardMaterial>>,
        meshes_assets: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        Self {
            planes: PlanesMaterials {
                default: MaterialsForSides {
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
                highlighted: MaterialsForSides {
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
            pieces: PiecesMaterials {
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
            plane_mesh: meshes_assets.add(Mesh::from(shape::Plane {
                size: crate::DEFAULT_CUBE_PLANE_SIZE,
            })),
            pieces_meshes: PiecesMeshes {
                pawn: asset_server.load("pawn.stl"),
                bishop: asset_server.load("bishop.stl"),
                knight: asset_server.load("knight.stl"),
                rook: asset_server.load("rook.stl"),
                queen: asset_server.load("queen.stl"),
                king: asset_server.load("king.stl"),
                mage: asset_server.load("mage.stl"),
            },
            move_indicator_material: materials_assets.add(StandardMaterial {
                base_color: crate::MOVE_INDICATOR_COLOR,
                alpha_mode: AlphaMode::Blend,
                base_color_texture: Some(asset_server.load("move.png")),
                unlit: true,
                ..default()
            }),
        }
    }
}

pub fn make_side_material(
    color: Color,
    asset_server: &Res<AssetServer>,
    materials_assets: &mut ResMut<Assets<StandardMaterial>>,
) -> MaterialTy {
    let mut mat: StandardMaterial = color.into();
    mat.unlit = true;
    mat.cull_mode = None;
    mat.base_color_texture = Some(asset_server.load("side.png"));
    materials_assets.add(mat)
}

pub fn make_piece_material(
    color: Color,
    materials_assets: &mut ResMut<Assets<StandardMaterial>>,
) -> MaterialTy {
    let mut mat: StandardMaterial = color.into();
    mat.metallic = 0.9;
    mat.reflectance = 0.2;
    materials_assets.add(mat)
}
