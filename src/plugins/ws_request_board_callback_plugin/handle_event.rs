use bevy::prelude::*;

use crate::{events::WsRequestBoardCallbackEvent, GameRecord};

pub fn system(
    mut ws_reader: EventReader<WsRequestBoardCallbackEvent>,
    game_state: Option<ResMut<GameRecord>>,
) {
    if let Some(mut game_state) = game_state {
        for event in ws_reader.iter() {
            game_state.board = event.0;
        }
    }
}
