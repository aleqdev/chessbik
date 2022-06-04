use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_mod_picking::{
    mesh_events_system, mesh_focus, pause_for_picking_blockers, PausedForBlockers,
    PickingEvent, PickingPluginsState, PickingSystem
};

mod mesh_selection;

pub struct CustomPickingPlugin;

impl Plugin for CustomPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_mod_picking::PickingPlugin)
            .init_resource::<PickingPluginsState>()
            .init_resource::<PausedForBlockers>()
            .add_event::<PickingEvent>()
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(|state: Res<PickingPluginsState>| {
                        if state.enable_interacting {
                            ShouldRun::Yes
                        } else {
                            ShouldRun::No
                        }
                    })
                    .with_system(
                        pause_for_picking_blockers
                            .label(PickingSystem::PauseForBlockers)
                            .after(PickingSystem::UpdateRaycast),
                    )
                    .with_system(
                        mesh_focus
                            .label(PickingSystem::Focus)
                            .after(PickingSystem::PauseForBlockers),
                    )
                    .with_system(
                        mesh_selection::mesh_selection
                            .label(PickingSystem::Selection)
                            .before(PickingSystem::Events)
                            .after(PickingSystem::Focus),
                    )
                    .with_system(mesh_events_system.label(PickingSystem::Events)),
            );
    }
}