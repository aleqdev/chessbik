use std::f32::consts::PI;

use crate::*;

pub struct DefaultCubeQueryItem {
    pub position: Vec3,
    pub cell: Cell,
}
pub struct DefaultCubeQuery {
    pub direction: Quat,
    pub color: Color,
    pub piece_mirrored: bool,
    pub cells: [DefaultCubeQueryItem; 9],
}

lazy_static::lazy_static! {
    pub static ref TOP: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
        color: Color::rgb(1., 0.678, 0.678),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: (-1., 1.5, -1.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., 1.5,  0.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., 1.5,  1.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0., 1.5, -1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., 1.5,  0.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., 1.5,  1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1., 1.5, -1.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., 1.5,  0.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., 1.5,  1.).into(), cell: cell::WPAWN},
        ],
    };

    pub static ref LEFT: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.),
        color: Color::rgb(0.792, 1., 0.749),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: (-1.,  1., 1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0.,  1., 1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1.,  1., 1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: (-1.,  0., 1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0.,  0., 1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1.,  0., 1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: (-1., -1., 1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0., -1., 1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1., -1., 1.5).into(), cell: cell::WPAWN}
        ]
    };

    pub static ref FORWARD: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., -PI / 2.),
        color: Color::rgb(0.607, 0.964, 1.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: (1.5,  1., -1.).into(), cell: cell::WKNIGHT},
            DefaultCubeQueryItem {position: (1.5,  1.,  0.).into(), cell: cell::WQUEEN},
            DefaultCubeQueryItem {position: (1.5,  1.,  1.).into(), cell: cell::WKNIGHT},
            DefaultCubeQueryItem {position: (1.5,  0., -1.).into(), cell: cell::WROOK},
            DefaultCubeQueryItem {position: (1.5,  0.,  0.).into(), cell: cell::WKING},
            DefaultCubeQueryItem {position: (1.5,  0.,  1.).into(), cell: cell::WROOK},
            DefaultCubeQueryItem {position: (1.5, -1., -1.).into(), cell: cell::WBISHOP},
            DefaultCubeQueryItem {position: (1.5, -1.,  0.).into(), cell: cell::WMAGE},
            DefaultCubeQueryItem {position: (1.5, -1.,  1.).into(), cell: cell::WBISHOP}
        ]
    };

    pub static ref RIGHT: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, -PI / 2., 0., 0.),
        color: Color::rgb(0.8, 0.854, 0.529),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: ( 1.,  1., -1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0.,  1., -1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1.,  1., -1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 1.,  0., -1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0.,  0., -1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1.,  0., -1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 1., -1., -1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0., -1., -1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1., -1., -1.5).into(), cell: cell::BPAWN}
        ]
    };

    pub static ref BACK: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.),
        color: Color::rgb(1., 0.839, 0.647),
        piece_mirrored: true,
        cells: [
            DefaultCubeQueryItem {position: (-1.5,  1.,  1.).into(), cell: cell::BKNIGHT},
            DefaultCubeQueryItem {position: (-1.5,  1.,  0.).into(), cell: cell::BQUEEN},
            DefaultCubeQueryItem {position: (-1.5,  1., -1.).into(), cell: cell::BKNIGHT},
            DefaultCubeQueryItem {position: (-1.5,  0.,  1.).into(), cell: cell::BROOK},
            DefaultCubeQueryItem {position: (-1.5,  0.,  0.).into(), cell: cell::BKING},
            DefaultCubeQueryItem {position: (-1.5,  0., -1.).into(), cell: cell::BROOK},
            DefaultCubeQueryItem {position: (-1.5, -1.,  1.).into(), cell: cell::BBISHOP},
            DefaultCubeQueryItem {position: (-1.5, -1.,  0.).into(), cell: cell::BMAGE},
            DefaultCubeQueryItem {position: (-1.5, -1., -1.).into(), cell: cell::BBISHOP}
        ]
    };

    pub static ref BOTTOM: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, PI, 0., 0.),
        color: Color::rgb(0.741, 0.698, 1.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: ( 1., -1.5, -1.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., -1.5,  0.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., -1.5,  1.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0., -1.5, -1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., -1.5,  0.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., -1.5,  1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1., -1.5, -1.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., -1.5,  0.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., -1.5,  1.).into(), cell: cell::BPAWN}
        ]
    };
}
