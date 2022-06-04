use bevy::prelude::{shape::Plane, *};
use chessbik_board::PieceMove;

use crate::Board2CubeTransforms;

#[derive(Default)]
pub struct AvailableMovesIndicator {
    pub moves: Vec<PieceMove>,
    pub indicators: Vec<Entity>,
}

impl AvailableMovesIndicator {
    pub fn update<Iter>(
        &mut self,
        moves: Iter,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        transforms: &Res<Board2CubeTransforms>,
        cube: Entity
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
                let mut transform = transforms.transform(m.pos);
                transform.translation += transform.up() * crate::MOVE_INDICATOR_OFFSET;

                self.indicators.push(parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(Plane { size: crate::DEFAULT_CUBE_PLANE_SIZE })),
                    material: materials.add(StandardMaterial {
                        base_color: crate::MOVE_INDICATOR_COLOR,
                        alpha_mode: AlphaMode::Blend,
                        base_color_texture: Some(asset_server.load("move.png")),
                        unlit: true,
                        ..default()
                    }),
                    transform,
                    ..default()
                }).id());
            }
        });
    }
}
