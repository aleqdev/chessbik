use bevy::prelude::{default, ResMut};
use bevy_egui::{
    egui::{
        style::{WidgetVisuals, Widgets},
        Color32, FontFamily, FontId, Rounding, Stroke, TextStyle, Visuals,
    },
    EguiContext,
};

pub fn system(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(Visuals {
        window_rounding: 0.0.into(),
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: Color32::from_rgb(15, 17, 29), // window background
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines, windows outlines
                fg_stroke: Stroke::new(2.0, Color32::from_gray(140)), // normal text color
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: Color32::from_rgb(25, 97, 79), // button background
                bg_stroke: Stroke::new(2.0, Color32::from_rgb(25, 97, 79)),
                fg_stroke: Stroke::new(2.0, Color32::from_gray(180)), // button text
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: Color32::from_rgb(35, 137, 89),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(65, 137, 89)), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(2.0, Color32::from_gray(240)),
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            active: WidgetVisuals {
                bg_fill: Color32::from_rgb(135, 157, 99),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(2.0, Color32::WHITE),
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            },
            open: WidgetVisuals {
                bg_fill: Color32::from_gray(67),
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                fg_stroke: Stroke::new(2.0, Color32::from_gray(210)),
                rounding: Rounding::same(0.0),
                expansion: 0.0,
            }
        },
        ..default()
    });

    let mut style = (*egui_ctx.ctx_mut().style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(22.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(22.0, FontFamily::Monospace)),
        (
            TextStyle::Monospace,
            FontId::new(22.0, FontFamily::Monospace),
        ),
        (TextStyle::Button, FontId::new(22.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(22.0, FontFamily::Monospace)),
    ]
    .into();

    egui_ctx.ctx_mut().set_style(style);
}
