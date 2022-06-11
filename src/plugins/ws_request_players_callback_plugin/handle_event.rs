use bevy::prelude::*;

use crate::{events::WsRequestPlayersCallbackEvent, GameRecord};

pub fn system(
    mut ws_reader: EventReader<WsRequestPlayersCallbackEvent>,
    game_record: Option<ResMut<GameRecord>>,
) {
    if let Some(mut game_record) = game_record {
        for event in ws_reader.iter() {
            game_record.players = event.0.clone();
        }
    }
}
