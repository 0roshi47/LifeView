use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::grid::GenerationType;
use crate::grid::Grid;
use crate::shapes::Shapes;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<EguiPlugin>());
        app.add_systems(EguiPrimaryContextPass, ui);
    }
}

pub fn ui(
    mut contexts: EguiContexts,
    grid: Option<ResMut<Grid>>,
    shapes: Option<Res<Shapes>>,
    diagnostics: Res<DiagnosticsStore>,
) -> Result {
    let Some(mut grid) = grid else { return Ok(()); };
    let Some(shapes) = shapes else { return Ok(()); };
    egui::Window::new("Lenia").show(contexts.ctx_mut()?, |ui| {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            ui.label(format!("FPS: {value:.2}"));
        }

        ui.heading("Simulation");
        ui.horizontal(|ui| {
            if ui.button("Pause").clicked() {
                grid.pause();
            }
            if ui.button("Reset").clicked() {
                grid.init();
            }
            if ui.button("Clear").clicked() {
                grid.clear();
            }
        });
        egui::ComboBox::from_label("Init generation")
            .selected_text(format!("{:?}", grid.generation_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut grid.generation_type, GenerationType::EMPTY, "Empty");
                ui.selectable_value(&mut grid.generation_type, GenerationType::RANDOM, "Random");
                ui.selectable_value(&mut grid.generation_type, GenerationType::BLOB, "Blob");
            });

        ui.add_space(20.0);
        ui.heading("Colors");
        for i in 1..grid.grid_coloration.color_range.len() + 1 {
            ui.horizontal(|ui| {
                color_picker(ui, &mut grid.grid_coloration.color_range[i - 1]);
                ui.label(format!("Color {i}"));
            });
        }

        ui.add_space(20.0);
        ui.heading("Rules");
        ui.add(egui::Slider::new(&mut grid.rule.micro, 0. ..=5.).text("Micro"));
        ui.add(egui::Slider::new(&mut grid.rule.sigma, 0. ..=5.).text("Sigma"));
        ui.add(egui::Slider::new(&mut grid.rule.radius, 1..=15).text("Radius"));
        ui.add(egui::Slider::new(&mut grid.rule.delta, 0.0..=1.).text("Delta"));

        ui.add_space(20.0);
        ui.heading("Shapes");
        for i in 0..shapes.0.len() {
            let name: String = shapes.0[i].name.clone();
            if ui.button(format!("{name}")).clicked() {
                grid.spawn_shape(name, shapes.0.clone());
            }
        }
    });
    Ok(())
}

fn color_picker(ui: &mut egui::Ui, color: &mut LinearRgba) {
    let mut c = [
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ];
    egui::color_picker::color_edit_button_srgb(ui, &mut c);
    *color = LinearRgba::new(
        c[0] as f32 / 255.,
        c[1] as f32 / 255.,
        c[2] as f32 / 255.,
        1.0, // <-- was 0.0, making everything transparent!
    );
}
