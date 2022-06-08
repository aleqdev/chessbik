use std::f32::consts::PI;

use bevy::math::{const_quat, const_vec3};
use chessbik_commons::Cell;

use crate::{
    app_materials::{AppMaterials, MaterialTy},
    *,
};

pub struct DefaultCubeQueryItem {
    pub position: Vec3,
    pub cell: Cell,
}

pub struct DefaultCubeQuery {
    pub direction: Quat,
    pub piece_mirrored: bool,
    pub cells: [DefaultCubeQueryItem; 0],
    pub default_material_getter: fn(&AppMaterials) -> MaterialTy,
    pub highlighted_material_getter: fn(&AppMaterials) -> MaterialTy,
}

pub const TRANSFORMS: [(Vec3, Quat); 54] = [
    (const_vec3!([-1., 1.5, -1.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([-1., 1.5, 0.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([-1., 1.5, 1.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([0., 1.5, -1.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([0., 1.5, 0.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([0., 1.5, 1.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([1., 1.5, -1.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([1., 1.5, 0.]), const_quat!([0., 0., 0., 1.])),
    (const_vec3!([1., 1.5, 1.]), const_quat!([0., 0., 0., 1.])),
    (
        const_vec3!([-1., 1., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([0., 1., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([1., 1., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([-1., 0., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([0., 0., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([1., 0., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([-1., -1., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([0., -1., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([1., -1., -1.5]),
        const_quat!([-0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([1.5, 1., -1.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, 1., 0.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, 1., 1.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, 0., -1.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, 0., 0.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, 0., 1.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, -1., -1.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, -1., 0.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1.5, -1., 1.]),
        const_quat!([0., 0., -0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([1., 1., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([0., 1., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([-1., 1., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([1., 0., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([0., 0., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([-1., 0., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([1., -1., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([0., -1., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([-1., -1., 1.5]),
        const_quat!([0.7071068, 0., 0., 0.7071068]),
    ),
    (
        const_vec3!([-1.5, 1., 1.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, 1., 0.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, 1., -1.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, 0., 1.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, 0., 0.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, 0., -1.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, -1., 1.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, -1., 0.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (
        const_vec3!([-1.5, -1., -1.]),
        const_quat!([0., 0., 0.7071068, 0.7071068]),
    ),
    (const_vec3!([1., -1.5, -1.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([1., -1.5, 0.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([1., -1.5, 1.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([0., -1.5, -1.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([0., -1.5, 0.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([0., -1.5, 1.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([-1., -1.5, -1.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([-1., -1.5, 0.]), const_quat!([1., 0., 0., 0.])),
    (const_vec3!([-1., -1.5, 1.]), const_quat!([1., 0., 0., 0.])),
];

lazy_static::lazy_static! {
    pub static ref TOP: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
        piece_mirrored: false,
        cells: [
        ],
        default_material_getter: |mats| mats.cells.default.top.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.top.clone()
    };

    pub static ref LEFT: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, -PI / 2., 0., 0.),
        piece_mirrored: false,
        cells: [
        ],
        default_material_getter: |mats| mats.cells.default.left.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.left.clone()
    };

    pub static ref FORWARD: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., -PI / 2.),
        piece_mirrored: false,
        cells: [
        ],
        default_material_getter: |mats| mats.cells.default.forward.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.forward.clone()
    };

    pub static ref RIGHT: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.),
        piece_mirrored: false,
        cells: [
        ],
        default_material_getter: |mats| mats.cells.default.right.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.right.clone()
    };

    pub static ref BACK: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.),
        piece_mirrored: true,
        cells: [
        ],
        default_material_getter: |mats| mats.cells.default.back.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.back.clone()
    };

    pub static ref BOTTOM: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, PI, 0., 0.),
        piece_mirrored: false,
        cells: [
        ],
        default_material_getter: |mats| mats.cells.default.bottom.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.bottom.clone()
    };
}
