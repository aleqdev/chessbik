use bevy::prelude::*;

use crate::{commons::SelectedPieceReference, AppLabels};

mod selection;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPieceReference>();
        app.add_system(selection::system.label(AppLabels::Selection));
    }
}
