use bevy::prelude::*;
use chessbik_board::{PiecePosition, PieceTy};
use chessbik_commons::Cell;

use crate::{
    app_assets::AppAssets, commons::CellMaterials, cube_transform, events::UpdateCubeDisplayEvent,
    BoardReference, GameRecord,
};

use super::cube::{Cube, Displayer};

pub fn system(
    mut commands: Commands,
    mut events: EventReader<UpdateCubeDisplayEvent>,
    cube: Option<ResMut<Cube>>,
    game_record: Option<Res<GameRecord>>,
    app_assets: Res<AppAssets>,
) {
    if let Some(game_record) = game_record {
        if let Some(mut cube) = cube {
            for e in events.iter() {
                for &diff in e.0.iter() {
                    despawn_displayer(&mut commands, &cube.displayers[diff]);
                    cube.displayers[diff] = spawn_displayer(
                        &mut commands,
                        cube.id,
                        &app_assets,
                        &game_record.board.at(diff),
                        diff,
                    );
                }
            }
        }
    }
}

pub fn despawn_displayer(commands: &mut Commands, d: &Displayer) {
    if let Some(plane) = d.plane {
        commands.entity(plane).despawn();
    }

    if let Some(piece) = d.piece {
        commands.entity(piece).despawn();
    }
}

pub fn spawn_displayer(
    commands: &mut Commands,
    cube: Entity,
    app_assets: &AppAssets,
    cell: &Cell,
    diff: usize,
) -> Displayer {
    let reference = BoardReference::from(PiecePosition(diff));

    let transform = {
        let (pos, quat) = cube_transform::transform(diff);
        Transform::from_translation(pos).with_rotation(quat)
    };

    let materials = CellMaterials::from_side(cell.side, app_assets);

    let plane = commands
        .spawn_bundle(PbrBundle {
            mesh: app_assets.plane_mesh.clone(),
            material: materials.default.clone(),
            transform: transform.clone(),
            ..default()
        })
        .insert(reference.clone())
        .insert(materials)
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .id();

    let transform = transform.with_scale(crate::PIECE_SCALE);

    let piece = cell.piece.map(|piece| {
        commands
            .spawn_bundle(PbrBundle {
                mesh: get_piece_mesh(piece.ty, app_assets),
                material: crate::commons::get_piece_material(piece.color, app_assets),
                transform: transform,
                ..default()
            })
            .insert(reference.clone())
            .insert(crate::commons::PieceMarker)
            .insert_bundle(bevy_mod_picking::PickableBundle::default())
            .id()
    });

    if let Some(piece) = piece {
        commands.entity(cube).push_children(&[plane, piece]);
    } else {
        commands.entity(cube).push_children(&[plane]);
    }

    Displayer {
        piece,
        plane: Some(plane),
    }
}

fn get_piece_mesh(ty: PieceTy, app_assets: &AppAssets) -> Handle<Mesh> {
    match ty {
        PieceTy::PAWN => app_assets.pieces_meshes.pawn.clone(),
        PieceTy::ROOK => app_assets.pieces_meshes.rook.clone(),
        PieceTy::KNIGHT => app_assets.pieces_meshes.knight.clone(),
        PieceTy::BISHOP => app_assets.pieces_meshes.bishop.clone(),
        PieceTy::QUEEN => app_assets.pieces_meshes.queen.clone(),
        PieceTy::KING => app_assets.pieces_meshes.king.clone(),
        PieceTy::MAGE => app_assets.pieces_meshes.mage.clone(),
    }
}
