use bevy::{app::PluginGroupBuilder, prelude::*};

mod camera_plugin;
mod copy_lobby_plugin;
mod custom_picking_plugin;
mod disable_ctx_menu_plugin;
mod menu_plugin;
mod resources_plugin;
mod websocket_plugin;

pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(camera_plugin::CameraPlugin);
        group.add(custom_picking_plugin::CustomPickingPlugin);
        group.add(resources_plugin::ResourcesPlugin);
        group.add(menu_plugin::MenuPlugin);
        group.add(copy_lobby_plugin::CopyLobbyPlugin);
        group.add(disable_ctx_menu_plugin::DisableCtxMenuPlugin);
        group.add(websocket_plugin::WebsocketPlugin);
    }
}
