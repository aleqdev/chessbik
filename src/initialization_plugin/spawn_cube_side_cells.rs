use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{commons, FieldReference, Piece, PiecePosition};

use super::{DefaultCubeQuery, InitializationSystemState};

pub fn spawn_cube_side_cells(
    parent: &mut ChildBuilder,
    state: &mut InitializationSystemState,
    query: &DefaultCubeQuery,
) {
    for super::DefaultCubeQueryItem { position, cell } in query.cells.iter() {
        let mesh = state.meshes.add(Mesh::from(shape::Plane {
            size: crate::DEFAULT_CUBE_PLANE_SIZE,
        }));
        let transform = Transform::from_translation(*position).with_rotation(query.direction);
        let reference = FieldReference::from(PiecePosition(state.field_refs.next().unwrap()));

        state.transforms.register(reference, transform);

        parent
            .spawn_bundle(PbrBundle {
                mesh,
                material: (query.default_material_getter)(&state.materials),
                transform,
                ..default()
            })
            .insert(reference.clone())
            .insert(commons::CellMaterials {
                default: (query.default_material_getter)(&state.materials),
                highlighted: (query.highlighted_material_getter)(&state.materials),
            })
            .insert_bundle(bevy_mod_picking::PickableBundle::default())
            .with_children(|parent| match cell.value {
                None => {}
                Some(Piece { ty, color }) => {
                    let mesh = state.asset_server.load(commons::get_piece_stl(ty));
                    let mut transform = Transform::from_scale(*crate::PIECE_SCALE)
                        .with_translation(Vec3::Y * 0.001);
                    if query.piece_mirrored {
                        transform.rotate(Quat::from_euler(EulerRot::XYZ, 0., PI, 0.));
                    }

                    parent
                        .spawn_bundle(PbrBundle {
                            mesh,
                            material: commons::get_piece_material(color, &state.materials),
                            transform,
                            ..default()
                        })
                        .insert(reference)
                        .insert(commons::PieceMarker)
                        .insert_bundle(bevy_mod_picking::PickableBundle::default());
                }
            });
    }
}
