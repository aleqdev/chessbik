use bevy::prelude::*;
use chessbik_board::PieceMove;

use crate::{cube_transform, app_assets::AppAssets, BoardReference};

#[derive(Default)]
pub struct AvailableMovesIndicator {
    pub moves: Vec<PieceMove>,
    pub indicators: Vec<Entity>,
}

impl AvailableMovesIndicator {
    pub fn clear_indicators(&mut self) {
        self.indicators.clear();
    }

    pub fn update<Iter>(
        &mut self,
        moves: Iter,
        commands: &mut Commands,
        app_assets: &Res<AppAssets>,
        cube: Entity,
    ) where
        Iter: Iterator,
        Iter::Item: Into<PieceMove>,
        Vec<PieceMove>: FromIterator<Iter::Item>,
    {
        self.moves = moves.collect();

        for i in self.indicators.drain(..) {
            commands.entity(i).despawn();
        }

        commands.entity(cube).with_children(|parent| {
            for m in self.moves.iter() {
                let (pos, quat) = cube_transform::transform(m.pos);
                let mut transform = Transform::from_translation(pos).with_rotation(quat);
                transform.translation += transform.up() * crate::MOVE_INDICATOR_OFFSET;

                self.indicators.push(
                    parent
                        .spawn_bundle(PbrBundle {
                            mesh: app_assets.plane_mesh.clone(),
                            material: app_assets.move_indicator_material.clone(),
                            transform,
                            ..default()
                        })
                        .insert_bundle(bevy_mod_picking::PickableBundle::default())
                        .insert(BoardReference(m.pos))
                        .id(),
                );
            }
        });
    }
}
