use bevy::{prelude::*, app::PluginGroupBuilder};

mod camera_plugin;
mod custom_picking_plugin;
mod resources_plugin;

pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(camera_plugin::CameraPlugin);
        group.add(custom_picking_plugin::CustomPickingPlugin);
        group.add(resources_plugin::ResourcesPlugin);
    }
}