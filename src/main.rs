use bevy::prelude::*;
use grid::Grid;

mod cell;
mod grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, grid::spawn.after(setup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let grid: Grid = Grid::new(200, 50);
    commands.insert_resource(grid);
}