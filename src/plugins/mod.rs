use bevy::{app::PluginGroupBuilder, prelude::*};

mod app_assets_plugin;
mod camera_plugin;
mod copy_lobby_plugin;
mod cube_plugin;
mod custom_picking_plugin;
mod disable_ctx_menu_plugin;
mod menu_plugin;
mod player_name_plugin;
mod player_token_plugin;
mod sides_lights_plugin;
mod ui_join_game_plugin;
mod ui_leave_game_plugin;
mod ui_new_game_plugin;
mod ui_request_engine_plugin;
mod ui_request_opponent_plugin;
mod websocket_plugin;
mod ws_consider_requesting_board_plugin;
mod ws_consider_requesting_players_plugin;
mod ws_consider_subscription_plugin;
mod ws_request_board_callback_plugin;
mod ws_request_player_token_callback_plugin;
mod ws_request_players_callback_plugin;
mod ui_change_name_plugin;

pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(camera_plugin::CameraPlugin);
        group.add(custom_picking_plugin::CustomPickingPlugin);
        group.add(app_assets_plugin::AppAssetsPlugin);
        group.add(menu_plugin::MenuPlugin);
        group.add(copy_lobby_plugin::CopyLobbyPlugin);
        group.add(disable_ctx_menu_plugin::DisableCtxMenuPlugin);
        group.add(websocket_plugin::WebsocketPlugin);
        group.add(ui_new_game_plugin::NewGamePlugin);
        group.add(ws_consider_subscription_plugin::ConsiderSubscriptionPlugin);
        group.add(ws_request_board_callback_plugin::RequestBoardCallbackPlugin);
        group.add(sides_lights_plugin::SidesLightsPlugin);
        group.add(cube_plugin::CubePlugin);
        group.add(player_name_plugin::PlayerNamePlugin);
        group.add(ui_leave_game_plugin::LeaveGamePlugin);
        group.add(ui_join_game_plugin::JoinGamePlugin);
        group.add(player_token_plugin::PlayerTokenPlugin);
        group.add(ws_request_player_token_callback_plugin::RequestPlayerTokenCallbackPlugin);
        group.add(ui_request_engine_plugin::RequestEnginePlugin);
        group.add(ui_request_opponent_plugin::RequestOpponentPlugin);
        group.add(ws_request_players_callback_plugin::RequestPlayersCallbackPlugin);
        group.add(ws_consider_requesting_board_plugin::ConsiderRequestingBoardPlugin);
        group.add(ws_consider_requesting_players_plugin::ConsiderRequestingPlayersPlugin);
        group.add(ui_change_name_plugin::ChangeNamePlugin);
    }
}
