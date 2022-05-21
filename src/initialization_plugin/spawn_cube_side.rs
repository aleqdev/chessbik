use bevy::prelude::*;

pub fn spawn_cube_side(
    commands: &mut Commands,
    state: &mut super::InitializationSystemState,
    query: &super::DefaultCubeQuery,
) -> Entity {
    commands
        .spawn()
        .with_children(|parent| super::spawn_cube_side_cells(parent, state, query))
        .id()
}
