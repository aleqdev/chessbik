use bevy::prelude::{EventWriter, Local, Res, ResMut};
use bevy_egui::{egui, EguiContext};
use chessbik_commons::{IsOwning, PlayerColor, PlayerRecord};
use egui_extras::{Size, StripBuilder};

use crate::{
    commons::{CubeRotationState, PlayerNameBuffer},
    events::{
        UiChangeNameEvent, UiJoinGameEvent, UiLeaveGameEvent, UiLobbyCopyEvent, UiNewGameEvent,
        UiRequestEngineEvent, UiRequestOpponentEvent,
    },
    GameRecord,
};

pub fn system(
    mut ctx: ResMut<EguiContext>,
    mut copy_lobby_writer: EventWriter<UiLobbyCopyEvent>,
    mut new_game_writer: EventWriter<UiNewGameEvent>,
    mut leave_game_writer: EventWriter<UiLeaveGameEvent>,
    mut join_game_writer: EventWriter<UiJoinGameEvent>,
    mut request_engine_writer: EventWriter<UiRequestEngineEvent>,
    mut request_opponent_writer: EventWriter<UiRequestOpponentEvent>,
    mut change_name_writer: EventWriter<UiChangeNameEvent>,
    mut name: ResMut<PlayerNameBuffer>,
    mut lobby_copied: Local<bool>,
    game_record: Option<Res<GameRecord>>,
    mut rotation_state: ResMut<CubeRotationState>,
) {
    let make_new_game_button = |ui: &mut egui::Ui| {
        if ui.button(" new game ").clicked() {
            new_game_writer.send_default();
        }
    };

    let mut make_leave_game_button = |ui: &mut egui::Ui| {
        if ui.button("leave game").clicked() {
            leave_game_writer.send_default();
            *rotation_state = Default::default();
        }
    };

    let mut make_copy_lobby_button = |ui: &mut egui::Ui| {
        let copy_lobby_text = if *lobby_copied {
            "  copied  "
        } else {
            "copy lobby"
        };

        let copy_lobby = ui.button(copy_lobby_text);

        if copy_lobby.clicked() {
            copy_lobby_writer.send_default();
            *lobby_copied = true;
        };

        if !copy_lobby.hovered() {
            *lobby_copied = false;
        }
    };

    let make_name_editor = |ui: &mut egui::Ui| {
        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            ui.add_space(16.);
            if ui
                .add(
                    egui::TextEdit::singleline(&mut name.0)
                        .desired_width(130.)
                        .hint_text("empty!"),
                )
                .lost_focus()
            {
                change_name_writer.send_default();
            }
            ui.label("your name:");
        });
    };

    let make_idle_menu_first_column = |ui: &mut egui::Ui| {
        ui.horizontal_centered(make_new_game_button);
    };

    let make_idle_menu_second_column = |ui: &mut egui::Ui| {
        ui.horizontal_centered(|ui| {
            if ui.button("join game from clipboard").clicked() {
                join_game_writer.send(UiJoinGameEvent);
            }
        });
    };

    let make_ingame_menu_first_column = |ui: &mut egui::Ui| {
        ui.horizontal_centered(|ui| {
            make_leave_game_button(ui);
            make_copy_lobby_button(ui);
        });
    };

    let make_ingame_menu_second_column_first_strip =
        |ui: &mut egui::Ui,
         game_record: &Res<GameRecord>,
         request_opponent_writer: &mut EventWriter<UiRequestOpponentEvent>,
         request_engine_writer: &mut EventWriter<UiRequestEngineEvent>| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                match game_record.players.white {
                    PlayerRecord::None => {
                        if ui.button("play").clicked() {
                            request_opponent_writer
                                .send(UiRequestOpponentEvent(PlayerColor::WHITE));
                        }
                        if ui.button("engine").clicked() {
                            request_engine_writer.send(UiRequestEngineEvent(PlayerColor::WHITE));
                        }
                    }
                    PlayerRecord::Engine(..) => {
                        ui.label(egui::RichText::new("engine").color(egui::Color32::GOLD));
                    }
                    PlayerRecord::Opponent(ref name, IsOwning(false)) => {
                        ui.label(egui::RichText::new(name.0.clone()));
                    }
                    PlayerRecord::Opponent(_, IsOwning(true)) => {
                        ui.label(egui::RichText::new("you").color(egui::Color32::LIGHT_GREEN));
                    }
                };
            });
        };

    let make_ingame_menu_second_column_second_strip = |ui: &mut egui::Ui| {
        ui.vertical_centered(|ui| {
            ui.label("♟ vs ♙");
        });
    };

    let make_ingame_menu_second_column_third_strip =
        |ui: &mut egui::Ui,
         game_record: &Res<GameRecord>,
         request_opponent_writer: &mut EventWriter<UiRequestOpponentEvent>,
         request_engine_writer: &mut EventWriter<UiRequestEngineEvent>| {
            ui.horizontal_centered(|ui| {
                match game_record.players.black {
                    PlayerRecord::None => {
                        if ui.button("play").clicked() {
                            request_opponent_writer
                                .send(UiRequestOpponentEvent(PlayerColor::BLACK));
                        }
                        if ui.button("engine").clicked() {
                            request_engine_writer.send(UiRequestEngineEvent(PlayerColor::BLACK));
                        }
                    }
                    PlayerRecord::Engine(..) => {
                        ui.label(egui::RichText::new("engine").color(egui::Color32::GOLD));
                    }
                    PlayerRecord::Opponent(ref name, IsOwning(false)) => {
                        ui.label(egui::RichText::new(name.0.clone()));
                    }
                    PlayerRecord::Opponent(_, IsOwning(true)) => {
                        ui.label(egui::RichText::new("you").color(egui::Color32::LIGHT_GREEN));
                    }
                };
            });
        };

    let make_ingame_menu_second_column = |ui: &mut egui::Ui| {
        if let Some(ref game_record) = game_record {
            StripBuilder::new(ui)
                .size(Size::relative(0.4))
                .size(Size::relative(0.2))
                .size(Size::relative(0.4))
                .clip(false)
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        make_ingame_menu_second_column_first_strip(
                            ui,
                            game_record,
                            &mut request_opponent_writer,
                            &mut request_engine_writer,
                        )
                    });
                    strip.cell(make_ingame_menu_second_column_second_strip);
                    strip.cell(|ui| {
                        make_ingame_menu_second_column_third_strip(
                            ui,
                            game_record,
                            &mut request_opponent_writer,
                            &mut request_engine_writer,
                        )
                    });
                });
        }
    };

    let make_root_element = |ui: &mut egui::Ui| {
        if game_record.is_none() {
            egui_extras::StripBuilder::new(ui)
                .size(egui_extras::Size::relative(0.3))
                .size(egui_extras::Size::relative(0.4))
                .size(egui_extras::Size::relative(0.3))
                .clip(false)
                .horizontal(|mut strip| {
                    strip.cell(make_idle_menu_first_column);
                    strip.cell(make_idle_menu_second_column);
                    strip.cell(make_name_editor);
                });
        } else {
            egui_extras::StripBuilder::new(ui)
                .size(egui_extras::Size::relative(0.25))
                .size(egui_extras::Size::relative(0.5))
                .size(egui_extras::Size::relative(0.25))
                .clip(false)
                .horizontal(|mut strip| {
                    strip.cell(make_ingame_menu_first_column);
                    strip.cell(make_ingame_menu_second_column);
                    strip.cell(make_name_editor);
                });
        }
    };
    egui::TopBottomPanel::top("menu")
        .max_height(22.)
        .show(ctx.ctx_mut(), make_root_element);
}
