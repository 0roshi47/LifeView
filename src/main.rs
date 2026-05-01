use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::grid::update_generation;
use crate::instancing::CellMaterialPlugin;
use crate::instancing::update_instance_data;
use crate::interface::UiPlugin;
use crate::shapes::add_shapes;
use crate::shapes::insert_shapes;
use bevy_egui::EguiPlugin;

mod cell;
mod grid;
mod grid_coloration;
mod instancing;
mod interface;
mod rule;
mod shapes;

const DRAW_STRENGHT: f32 = 0.05;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EguiPlugin {
            ..EguiPlugin::default()
        })
        .add_plugins(CellMaterialPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, insert_shapes)
        .add_systems(Startup, add_shapes.after(insert_shapes))
        .add_systems(FixedUpdate, update_generation)
        .add_systems(
            FixedUpdate,
            update_instance_data.after(crate::grid::update_generation),
        )
        .add_systems(FixedUpdate, mouse_click.after(update_generation))
        .run();
}

fn mouse_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    grid: Option<ResMut<grid::Grid>>,
) {
    let Some(grid) = grid else { return };
    let window = windows.single().unwrap();
    if mouse.pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            draw(position, grid, 1.0, window);
        }
    } else if mouse.pressed(MouseButton::Right) {
        if let Some(position) = window.cursor_position() {
            draw(position, grid, -1.0, window);
        }
    }
}

fn draw(position: Vec2, mut grid: ResMut<grid::Grid>, pressure: f32, window: &Window) {
    let world_x = position.x;
    let world_y = window.height() - position.y;
    let gx = (world_x / grid.cell_size).floor() as i32;
    let gy = (world_y / grid.cell_size).floor() as i32;
    let true_pos = grid.wrap_pos(IVec2::new(gx, gy));
    let idx: usize = grid.vector_to_idx(true_pos) as usize;
    grid.cells[idx].state += DRAW_STRENGHT * pressure * grid.rule.delta;
    grid.cells[idx].state = grid.cells[idx].state.clamp(0., 1.);
}
