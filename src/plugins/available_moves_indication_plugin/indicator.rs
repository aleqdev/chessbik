use bevy::prelude::*;
use chessbik_board::PieceMove;

use crate::{app_assets::AppAssets, cube_transform, BoardReference};

#[derive(Default)]
pub struct AvailableMovesIndicator {
    pub indicators: Vec<Entity>,
}

impl AvailableMovesIndicator {
    pub fn clear_indicators(&mut self) {
        self.indicators.clear();
    }

    pub fn update(
        &mut self,
        commands: &mut Commands,
        app_assets: &AppAssets,
        cube: Entity,
        moves: &Vec<PieceMove>,
    ) {
        for i in self.indicators.drain(..) {
            commands.entity(i).despawn();
        }

        commands.entity(cube).with_children(|parent| {
            for m in moves.iter() {
                match m {
                    PieceMove::Slide(pos) | PieceMove::Take(pos) => {
                        let (vec3, quat) = cube_transform::transform(*pos);
                        let mut transform = Transform::from_translation(vec3).with_rotation(quat);
                        transform.translation += transform.up() * crate::MOVE_INDICATOR_OFFSET;

                        self.indicators.push(
                            parent
                                .spawn_bundle(PbrBundle {
                                    mesh: app_assets.plane_mesh.clone(),
                                    material: app_assets.move_indicator_material.clone(),
                                    transform,
                                    ..default()
                                })
                                .insert(BoardReference(*pos))
                                .id(),
                        );
                    }
                    PieceMove::Rotation(..) => {}
                }
            }
        });
    }
}
