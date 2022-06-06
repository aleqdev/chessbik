use bevy::prelude::{EventWriter, ResMut};
use bevy_egui::{egui, EguiContext};

use crate::events::{UiLobbyCopyEvent, UiNewGameEvent};

pub fn system(
    mut ctx: ResMut<EguiContext>,
    mut copy_lobby_writer: EventWriter<UiLobbyCopyEvent>,
    mut new_game_writer: EventWriter<UiNewGameEvent>,
) {
    egui::TopBottomPanel::top("menu").show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::left_to_right(), |ui| {
            if ui
                .button(egui::RichText::new("copy lobby").size(22.))
                .clicked()
            {
                copy_lobby_writer.send_default();
            }

            if ui
                .button(egui::RichText::new("new game").size(22.))
                .clicked()
            {
                new_game_writer.send_default();
            }
        });
    });
}
