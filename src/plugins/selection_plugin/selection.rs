use bevy::prelude::*;
use bevy_mod_picking::{Hover, Selection};
use chessbik_board::{Piece, PieceMove, PieceColor, BoardStatus, PiecePosition};
use chessbik_commons::Cell;

use crate::{
    app_assets::AppAssets,
    commons::{self, CellMaterials, SelectedPieceReference},
    BoardReference, GameRecord, plugins::available_moves_indication_plugin::AvailableMovesIndicator,
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
            &BoardReference,
            &CellMaterials,
            &Hover,
            &Selection,
        ),
        (
            Or<(Changed<Hover>, Changed<Selection>)>,
            Without<crate::commons::PieceMarker>,
        ),
    >,
    record: Option<Res<GameRecord>>,
    materials: Res<AppAssets>,
    mut selected_reference: ResMut<SelectedPieceReference>,
    indicator: Res<AvailableMovesIndicator>
) {
    if let Some(record) = record {
        let board = record.board;

        for (mut mat, fref, hover, selection) in query_pieces.iter_mut() {
            if can_cell_be_selected(
                board.at(*fref), 
                **fref, 
                indicator.moves.as_ref(),
                record.as_ref(),
                selected_reference.0 == None
            ) {
                if selection.selected() {
                    *mat = materials.selected.clone();
                    selected_reference.0 = Some(*fref);
                    continue;
                }
            }
    
            match board.at(*fref).piece {
                Some(piece) => match hover.hovered() {
                    true => *mat = commons::get_piece_material_hovered(piece.color, &materials),
                    false => *mat = commons::get_piece_material(piece.color, &materials),
                },
                None => {}
            }
        }
    
        for (mut mat, fref, mats, hover, selection) in query_cells.iter_mut() {
            if can_cell_be_selected(
                board.at(*fref), 
                **fref, 
                indicator.moves.as_ref(),
                record.as_ref(),
                selected_reference.0 == None
            ) {
                if selection.selected() {
                    *mat = materials.selected.clone();
                    selected_reference.0 = Some(*fref);
                    continue;
                }
            }
    
            match hover.hovered() {
                true => *mat = mats.highlighted.clone(),
                false => *mat = mats.default.clone(),
            }
        }
    }
}

fn can_cell_be_selected(cell: &Cell, pos: PiecePosition, available_moves: &[PieceMove], record: &GameRecord, is_second_selection: bool) -> bool {
    match is_second_selection {
        false => {
            match cell.piece {
                Some(piece) => match piece.color {
                    PieceColor::WHITE => record.board.status == BoardStatus::WhitesMove,
                    PieceColor::BLACK => record.board.status == BoardStatus::BlacksMove,
                },
                None => false,
            }
        }

        true => {
            if !available_moves.iter().any(|m| m.pos == pos) {
                return false;
            }

            match cell.piece {
                Some(piece) => match piece.color {
                    PieceColor::BLACK => record.board.status == BoardStatus::WhitesMove,
                    PieceColor::WHITE => record.board.status == BoardStatus::BlacksMove,
                },
                None => true,
            }
        }
    }
}