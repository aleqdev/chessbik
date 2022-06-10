use bevy::prelude::{EventWriter, Local, Res, ResMut};
use bevy_egui::{egui, EguiContext};
use chessbik_commons::PlayerColor;

use crate::{
    commons::{JoinGameBuffer, PlayerNameBuffer},
    events::{UiJoinGameEvent, UiLeaveGameEvent, UiLobbyCopyEvent, UiNewGameEvent, UiRequestEngineEvent, UiRequestOpponentEvent},
    GameRecord, PlayerRecord,
};

pub fn system(
    mut ctx: ResMut<EguiContext>,
    mut copy_lobby_writer: EventWriter<UiLobbyCopyEvent>,
    mut new_game_writer: EventWriter<UiNewGameEvent>,
    mut leave_game_writer: EventWriter<UiLeaveGameEvent>,
    mut join_game_writer: EventWriter<UiJoinGameEvent>,
    mut request_engine_writer: EventWriter<UiRequestEngineEvent>,
    mut request_opponent_writer: EventWriter<UiRequestOpponentEvent>,
    mut name: ResMut<PlayerNameBuffer>,
    mut join_game: ResMut<JoinGameBuffer>,
    mut lobby_copied: Local<bool>,
    game_record: Option<Res<GameRecord>>,
) {
    let make_new_game_button = |ui: &mut egui::Ui| {
        if ui.button(" new game ").clicked() {
            new_game_writer.send_default();
        }
    };

    let mut make_leave_game_button = |ui: &mut egui::Ui| {
        if ui.button("leave game").clicked() {
            leave_game_writer.send_default();
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
                println!("lost focus");
            }
            ui.label("your name:");
        });
    };

    let make_idle_menu_first_column = |ui: &mut egui::Ui| {
        ui.horizontal_centered(make_new_game_button);
    };

    let make_idle_menu_second_column = |ui: &mut egui::Ui| {
        ui.horizontal_centered(|ui| {
            if ui.button("join game:").clicked() {
                join_game_writer.send_default();
            }
            if ui
                .add(
                    egui::TextEdit::singleline(&mut join_game.0)
                        .desired_width(160.)
                        .hint_text("paste lobby")
                        .password(true),
                )
                .lost_focus()
            {
                println!("lost focus");
            }
        });
    };

    let make_ingame_menu_first_column = |ui: &mut egui::Ui| {
        ui.horizontal_centered(|ui| {
            make_leave_game_button(ui);
            make_copy_lobby_button(ui);
        });
    };

    let make_ingame_menu_second_column = |ui: &mut egui::Ui| {
        if let Some(ref game_record) = game_record {
            ui.columns(5, |ui| {
                match game_record.players.white {
                    PlayerRecord::None => {
                        if ui[0].button("engine").clicked() {
                            request_engine_writer.send(UiRequestEngineEvent(PlayerColor::WHITE));
                        }
                        if ui[1].button("play").clicked() {
                            request_opponent_writer.send(UiRequestOpponentEvent(PlayerColor::WHITE));
                        }
                    },
                    PlayerRecord::Engine => {
                        ui[0].label(egui::RichText::new("engine").color(egui::Color32::LIGHT_YELLOW));
                    }
                    PlayerRecord::Opponent(ref name, false) => {
                        ui[0].label(egui::RichText::new(name));
                    },
                    PlayerRecord::Opponent(_, true) => {
                        ui[0].label(egui::RichText::new("you").color(egui::Color32::LIGHT_GREEN));
                    }
                };
                ui[2].vertical_centered(|ui| {
                    ui.label("♙ vs ♟");
                });
                match game_record.players.black {
                    PlayerRecord::None => {
                        if ui[3].button("play").clicked() {
                            request_opponent_writer.send(UiRequestOpponentEvent(PlayerColor::BLACK));
                        }
                        if ui[4].button("engine").clicked() {
                            request_engine_writer.send(UiRequestEngineEvent(PlayerColor::BLACK));
                        }
                    },
                    PlayerRecord::Engine => {
                        ui[4].label(egui::RichText::new("engine").color(egui::Color32::LIGHT_YELLOW));
                    }
                    PlayerRecord::Opponent(ref name, false) => {
                        ui[4].label(egui::RichText::new(name));
                    },
                    PlayerRecord::Opponent(_, true) => {
                        ui[4].label(egui::RichText::new("you").color(egui::Color32::LIGHT_GREEN));
                    }
                };
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
    egui::TopBottomPanel::top("menu").max_height(22.).show(ctx.ctx_mut(), make_root_element);
}
