use bevy::prelude::*;
use chessbik_board::Board;
use chessbik_commons::Cell;

use crate::{events::UpdateCubeDisplayEvent, GameRecord};

use super::{compute_board_difference, cube::Cube};

pub fn system(
    game_record: Option<Res<GameRecord>>,
    cube: Option<ResMut<Cube>>,
    mut last_board: Local<Option<Board<Cell>>>,
    mut commands: Commands,
    mut update_display_events: EventWriter<UpdateCubeDisplayEvent>,
) {
    if let Some(game_record) = game_record {
        if cube.is_none() {
            let id = spawn_cube(&mut commands);
            commands.insert_resource(Cube::new(id));
        }

        if last_board.map_or(true, |last| last != game_record.board) {
            let diff =
                compute_board_difference::compute_optional(last_board.as_ref(), &game_record.board);

            update_display_events.send(diff.into());

            *last_board = Some(game_record.board);
        }
    } else {
        if let Some(cube) = cube {
            commands.entity(cube.id).despawn_recursive();
            commands.remove_resource::<Cube>();
        }
        *last_board = None;
    }
}

pub fn spawn_cube(commands: &mut Commands) -> Entity {
    commands
        .spawn()
        .insert_bundle(TransformBundle {
            local: Transform::from_scale(crate::DEFAULT_CUBE_SCALE),
            ..default()
        })
        .id()
}
