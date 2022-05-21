use bevy::prelude::*;

pub fn get_piece_material(color: crate::PieceColor) -> StandardMaterial {
    match color {
        crate::PieceColor::WHITE => {
            let mut mat: StandardMaterial = Color::rgb(0.45, 0.44, 0.5).into();
            mat.metallic = 0.9;
            mat.reflectance = 0.2;
            mat
        }
        crate::PieceColor::BLACK => {
            let mut mat: StandardMaterial = Color::rgb(0.12, 0.1, 0.1).into();
            mat.metallic = 0.9;
            mat.reflectance = 0.2;
            mat
        }
    }
}
