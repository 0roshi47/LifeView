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
    let Some(mut grid) = grid else {
        return Ok(());
    };
    let Some(shapes) = shapes else {
        return Ok(());
    };

    // ── Top bar ───────────────────────────────────────────────────
    egui::TopBottomPanel::top("topbar")
        .exact_height(32.0)
        .show(contexts.ctx_mut()?, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(egui::RichText::new("LifeView").strong());

                ui.separator();

                if ui.button("⏸ Pause").clicked() {
                    grid.pause();
                }
                if ui.button("↺ Reset").clicked() {
                    grid.init();
                }
                if ui.button("✕ Clear").clicked() {
                    grid.clear();
                }

                ui.separator();

                if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
                    && let Some(value) = fps.smoothed()
                {
                    ui.label(format!("FPS: {value:.0}"));
                }
            });
        });

    // ── Side panel ────────────────────────────────────────────────
    egui::SidePanel::left("main_panel")
        .resizable(false)
        .exact_width(220.0)
        .show(contexts.ctx_mut()?, |ui| {
            // ── System ────────────────────────────────────────────
            ui.add_space(8.0);
            ui.label(egui::RichText::new("System").heading());
            ui.add_space(4.0);

            for (label, tag) in [
                ("Lenia", "continuous"),
                ("Game of Life", "discrete"),
                ("SmoothLife", "continuous"),
                ("Brian's Brain", "discrete"),
            ] {
                let selected = label == "Lenia"; // placeholder
                if ui
                    .add_sized(
                        [204.0, 24.0],
                        egui::SelectableLabel::new(selected, format!("{label}  {tag}")),
                    )
                    .clicked()
                { /* TODO: switch system */ }
            }

            ui.separator();

            // ── Grid ──────────────────────────────────────────────
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Grid").heading());
            ui.add_space(4.0);

            egui::ComboBox::from_label("Topology")
                .selected_text("Toroidal (loop)")
                .show_ui(ui, |ui| {
                    ui.selectable_label(true, "Toroidal (loop)");
                    ui.selectable_label(false, "Finite");
                });

            egui::ComboBox::from_label("Init")
                .selected_text(format!("{:?}", grid.generation_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut grid.generation_type, GenerationType::EMPTY, "Empty");
                    ui.selectable_value(
                        &mut grid.generation_type,
                        GenerationType::RANDOM,
                        "Random",
                    );
                    ui.selectable_value(&mut grid.generation_type, GenerationType::BLOB, "Blob");
                });

            ui.separator();

            // ── Rules ─────────────────────────────────────────────
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Rules").heading());
            ui.add_space(4.0);

            ui.add(egui::Slider::new(&mut grid.rule.micro, 0.0..=1.0).text("μ micro"));
            ui.add(egui::Slider::new(&mut grid.rule.sigma, 0.0..=1.0).text("σ sigma"));
            ui.add(egui::Slider::new(&mut grid.rule.radius, 1..=15).text("Radius"));
            ui.add(egui::Slider::new(&mut grid.rule.delta, 0.0..=1.0).text("Δt"));

            ui.separator();

            // ── Color map ─────────────────────────────────────────
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Color map").heading());
            ui.add_space(4.0);

            for i in 0..grid.grid_coloration.color_range.len() {
                ui.horizontal(|ui| {
                    color_picker(ui, &mut grid.grid_coloration.color_range[i]);
                    ui.label(format!("Color {}", i + 1));
                });
            }

            ui.separator();

            // ── Shapes ────────────────────────────────────────────
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Shapes").heading());
            ui.add_space(4.0);

            egui::Grid::new("shapes_grid")
                .num_columns(2)
                .spacing([6.0, 6.0])
                .show(ui, |ui| {
                    for (i, shape) in shapes.0.iter().enumerate() {
                        let name = shape.name.clone();
                        if ui.button(&name).clicked() {
                            grid.spawn_shape(name, shapes.0.clone());
                        }
                        if i % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });
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
        1.0,
    );
}
