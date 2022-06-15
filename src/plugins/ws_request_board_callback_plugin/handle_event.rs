use bevy::prelude::*;

use crate::{commons::BoardReserve, events::WsRequestBoardCallbackEvent, GameRecord};

pub fn system(
    mut ws_reader: EventReader<WsRequestBoardCallbackEvent>,
    game_state: Option<ResMut<GameRecord>>,
    mut board_reserve: ResMut<BoardReserve>,
) {
    if let Some(mut game_state) = game_state {
        for event in ws_reader.iter() {
            game_state.board = event.0.clone();
            board_reserve.0 = Some(event.0);
        }
    }
}
