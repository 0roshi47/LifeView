use bevy::prelude::*;
use crate::grid::Grid;

mod cell;
mod grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_cam)
        .add_systems(Startup, setup_grid)
        .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid() {
    let grid: Grid = Grid::new(100, 100);
    println!("{}", grid.cells.len());
}