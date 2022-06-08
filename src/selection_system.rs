use bevy::prelude::*;
use bevy_mod_picking::{Hover, Selection};
use chessbik_board::Board;
use chessbik_commons::Cell;

use crate::{
    app_materials::AppMaterials,
    commons::{self, CellMaterials, SelectedPieceReference},
    BoardReference,
};

pub fn system(
    mut query_pieces: Query<
        (
            &mut Handle<StandardMaterial>,
            &BoardReference,
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
    board: Res<Board<Cell>>,
    materials: Res<AppMaterials>,
    mut selected_reference: ResMut<SelectedPieceReference>,
) {
    for (mut mat, fref, hover, selection) in query_pieces.iter_mut() {
        if selection.selected() {
            *mat = materials.selected.clone();
            selected_reference.0 = Some(*fref);
            continue;
        }

        match board.at(*fref).piece {
            Some(piece) => match hover.hovered() {
                true => *mat = commons::get_piece_material_hovered(piece.color, &materials),
                false => *mat = commons::get_piece_material(piece.color, &materials),
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
