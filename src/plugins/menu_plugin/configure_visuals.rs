use bevy::prelude::{ResMut, default};
use bevy_egui::{EguiContext, egui::{Visuals, Rounding, Stroke, Color32, style::{Widgets, WidgetVisuals}}};

pub fn system(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(Visuals {
        window_rounding: 0.0.into(),
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: Color32::from_gray(7), // window background
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines, windows outlines
                fg_stroke: Stroke::new(1.0, Color32::from_gray(140)), // normal text color
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: Color32::from_rgb(25, 37, 79), // button background
                bg_stroke: Default::default(),
                fg_stroke: Stroke::new(1.0, Color32::from_gray(180)), // button text
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: Color32::from_rgb(35, 57, 109),
                bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
                rounding: Rounding::same(0.0),
                expansion: 1.0,
            },
            active: WidgetVisuals {
                bg_fill: Color32::from_rgb(35, 147, 109),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(2.0, Color32::WHITE),
                rounding: Rounding::same(0.0),
                expansion: 1.0,
            },
            open: WidgetVisuals {
                bg_fill: Color32::from_gray(27),
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            ..default()
        },
        ..default()
    });
}