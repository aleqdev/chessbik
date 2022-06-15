use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContext;
use chessbik_board::PieceMove;
use chessbik_commons::PieceMovePair;

use crate::commons::BoardReserve;
use crate::commons::CubeRotationState;
use crate::events::MakeMoveEvent;
use crate::GameRecord;

pub fn system(
    mut ctx: ResMut<EguiContext>,
    mut rotation_state: ResMut<CubeRotationState>,
    board_reserve: Res<BoardReserve>,
    game_record: Option<ResMut<GameRecord>>,
    mut make_move_writer: EventWriter<MakeMoveEvent>,
) {
    if !rotation_state.is_rotating {
        return;
    }

    if board_reserve.0.is_none() {
        return;
    }

    if game_record.is_none() {
        return;
    }
    let mut game_record = game_record.unwrap();

    let board_reserve = board_reserve.0.unwrap();

    egui::TopBottomPanel::bottom("rotation_menu")
        .max_height(28.)
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("cancel").clicked() {
                    *rotation_state = CubeRotationState::default();
                    game_record.board = board_reserve;
                }

                ui.add_space(50.);

                if rotation_state.selected_rotator.is_none() {
                    ui.label("select face to rotate: ");
                } else {
                    ui.label(" rotate using buttons: ");
                }

                match rotation_state.selected_rotator {
                    Some(rots) => {
                        let update_board = |x: usize| {
                            game_record.board = board_reserve;
                            game_record.board = game_record.get_board_after_rotations(rots[x]);
                        };

                        if ui.button("<").clicked() {
                            rotation_state.rotator_cycle = match rotation_state.rotator_cycle {
                                Some(x) => Some((x as isize - 1).rem_euclid(3) as usize),
                                None => Some(2),
                            };
                            rotation_state.rotator_cycle.map(update_board);
                        } else if ui.button(">").clicked() {
                            rotation_state.rotator_cycle = match rotation_state.rotator_cycle {
                                Some(x) => Some((x + 1) % 3),
                                None => Some(0),
                            };
                            rotation_state.rotator_cycle.map(update_board);
                        };

                        ui.add_space(50.);

                        match rotation_state.rotator_cycle {
                            Some(cycle) => {
                                if ui.button("submit").clicked() {
                                    make_move_writer.send(MakeMoveEvent(PieceMovePair {
                                        from: rotation_state.mage.unwrap(),
                                        mv: PieceMove::Rotation(rots[cycle]),
                                    }));

                                    *rotation_state = CubeRotationState::default();
                                    game_record.board = board_reserve;
                                }
                            }
                            None => {
                                ui.add_enabled_ui(false, |ui| {
                                    let _ = ui.button("submit");
                                });
                            }
                        }
                    }
                    None => {
                        ui.add_enabled_ui(false, |ui| {
                            let _ = ui.button("<");
                            let _ = ui.button(">");
                            ui.add_space(50.);
                            let _ = ui.button("submit");
                        });
                    }
                }
            });
        });
}
