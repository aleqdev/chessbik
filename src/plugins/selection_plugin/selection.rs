use bevy::prelude::*;
use bevy_mod_picking::{Hover, Selection};
use chessbik_board::{Board, BoardStatus, PieceColor, PieceMove, PiecePosition};
use chessbik_commons::{Cell, IsOwning, PieceMovePair, PlayerColor, PlayerRecord};
use itertools::Itertools;

use crate::{
    app_assets::AppAssets,
    commons::{
        self, AvailableMovesStorage, CellMaterials, CubeRotationState, SelectedPieceReference,
    },
    events::MakeMoveEvent,
    BoardReference, GameRecord,
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
    app_assets: Res<AppAssets>,
    mut selected_ref: ResMut<SelectedPieceReference>,
    mut make_move_events: EventWriter<MakeMoveEvent>,
    moves: Res<AvailableMovesStorage>,
    rotation_state: Res<CubeRotationState>,
) {
    if rotation_state.is_rotating {
        return;
    }

    if let Some(record) = record {
        let board = record.board;

        for (mut material, board_ref, mats, hover, selection) in query_cells.iter_mut() {
            determine_selection(
                &board,
                &board_ref,
                &record,
                &mut selected_ref,
                &selection,
                &hover,
                &mut material,
                |mat| {
                    **mat = app_assets.selected.clone();
                },
                |mat| **mat = mats.highlighted.clone(),
                |mat| **mat = mats.default.clone(),
                true,
                &mut make_move_events,
                &moves,
            );
        }

        for (mut material, board_ref, hover, selection) in query_pieces
            .iter_mut()
            .sorted_by_key(|(_, _, _, s)| !s.selected())
        {
            determine_selection(
                &board,
                &board_ref,
                &record,
                &mut selected_ref,
                &selection,
                &hover,
                &mut material,
                |mat| {
                    **mat = app_assets.selected.clone();
                },
                |mat| {
                    if let Some(piece) = board.at(*board_ref).piece {
                        **mat = commons::get_piece_material_hovered(piece.color, &app_assets)
                    }
                },
                |mat| {
                    if let Some(piece) = board.at(*board_ref).piece {
                        **mat = commons::get_piece_material(piece.color, &app_assets)
                    }
                },
                false,
                &mut make_move_events,
                &moves,
            );
        }
    }
}

fn determine_selection(
    board: &Board<Cell>,
    board_ref: &BoardReference,
    record: &GameRecord,
    selected_ref: &mut SelectedPieceReference,
    selection: &Selection,
    hover: &Hover,
    material: &mut Mut<Handle<StandardMaterial>>,
    make_selected_fn: impl FnOnce(&mut Mut<Handle<StandardMaterial>>),
    make_hovered_fn: impl FnOnce(&mut Mut<Handle<StandardMaterial>>),
    make_default_fn: impl FnOnce(&mut Mut<Handle<StandardMaterial>>),
    is_plane: bool,
    make_move_events: &mut EventWriter<MakeMoveEvent>,
    moves: &Res<AvailableMovesStorage>,
) {
    if can_cell_be_selected(
        board.at(*board_ref),
        **board_ref,
        moves.0.as_ref(),
        record,
        selected_ref.0 != None,
        is_plane,
    ) {
        if selection.selected() {
            make_selected_fn(material);

            match selected_ref.0 {
                Some(sel) => match moves.0.iter().find(|m| m.eq_position(&**board_ref)) {
                    Some(p) => {
                        make_move_events.send(MakeMoveEvent(PieceMovePair { from: *sel, mv: *p }));
                    }
                    None => selected_ref.0 = Some(*board_ref),
                },
                None => selected_ref.0 = Some(*board_ref),
            }

            return;
        } else if let Some(oref) = selected_ref.0 {
            if oref == *board_ref {
                selected_ref.0 = None;
            }
        }

        match hover.hovered() {
            true => make_hovered_fn(material),
            false => make_default_fn(material),
        }
    } else if !selection.selected() {
        make_default_fn(material)
    }
}

fn can_cell_be_selected(
    cell: &Cell,
    pos: PiecePosition,
    available_moves: &[PieceMove],
    record: &GameRecord,
    is_second_selection: bool,
    is_plane: bool,
) -> bool {
    let is_owning_player = |color: PlayerColor| match color {
        PlayerColor::WHITE => {
            if let PlayerRecord::Opponent(_, IsOwning(true)) = record.players.white {
                true
            } else {
                false
            }
        }
        PlayerColor::BLACK => {
            if let PlayerRecord::Opponent(_, IsOwning(true)) = record.players.black {
                true
            } else {
                false
            }
        }
    };

    match is_second_selection {
        false => match is_plane {
            true => false,
            false => match cell.piece {
                Some(piece) => match piece.color {
                    PieceColor::WHITE => {
                        record.board.status == BoardStatus::WhitesMove
                            && is_owning_player(PlayerColor::WHITE)
                    }
                    PieceColor::BLACK => {
                        record.board.status == BoardStatus::BlacksMove
                            && is_owning_player(PlayerColor::BLACK)
                    }
                },
                None => false,
            },
        },

        true => match cell.piece {
            Some(piece) => match is_plane {
                true => false,
                false => match piece.color {
                    PieceColor::BLACK => match record.board.status {
                        BoardStatus::WhitesMove => {
                            available_moves.iter().any(|m| m.eq_position(&pos))
                        }
                        BoardStatus::BlacksMove => true,
                        BoardStatus::Mate => false,
                    },
                    PieceColor::WHITE => match record.board.status {
                        BoardStatus::WhitesMove => true,
                        BoardStatus::BlacksMove => {
                            available_moves.iter().any(|m| m.eq_position(&pos))
                        }
                        BoardStatus::Mate => false,
                    },
                },
            },
            None => available_moves.iter().any(|m| m.eq_position(&pos)),
        },
    }
}
