use bevy::prelude::*;
use bevy_mod_picking::{Hover, Selection};

use crate::{app_materials::AppMaterials, commons::CellMaterials, Field, FieldReference};

pub fn system(
    mut query_pieces: Query<
        (
            &mut Handle<StandardMaterial>,
            &FieldReference,
            &Hover,
            &Selection,
        ),
        (
            Or<(Changed<Hover>, Changed<Selection>)>,
            With<crate::commons::PieceMarker>,
        ),
    >,
    mut query_cells: Query<
        (
            &mut Handle<StandardMaterial>,
            &CellMaterials,
            &Hover,
            &Selection,
        ),
        (
            Or<(Changed<Hover>, Changed<Selection>)>,
            Without<crate::commons::PieceMarker>,
        ),
    >,
    field: Res<Field>,
    materials: Res<AppMaterials>,
) {
    for (mut mat, fref, hover, selection) in query_pieces.iter_mut() {
        if selection.selected() {
            *mat = materials.selected.clone();
            continue;
        }

        match field.at(fref).value {
            Some(piece) => match hover.hovered() {
                true => *mat = crate::commons::get_piece_material_hovered(piece.color, &materials),
                false => *mat = crate::commons::get_piece_material(piece.color, &materials),
            },
            None => {}
        }
    }

    for (mut mat, mats, hover, selection) in query_cells.iter_mut() {
        if selection.selected() {
            *mat = materials.selected.clone();
            continue;
        }

        match hover.hovered() {
            true => *mat = mats.highlighted.clone(),
            false => *mat = mats.default.clone(),
        }
    }
}
