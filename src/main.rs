mod gameboard;

use bevy::prelude::*;
use gameboard::grid::{Grid, spawn_grid};
use gameboard::tile_cursor::{setup_tile_cursor, move_tile_cursor};

const TILE_SIZE: f32 = 32.0; // Size of each tile in pixels
const GRID_WIDTH: i32 = 20; // Number of tiles horizontally
const GRID_HEIGHT: i32 = 20; // Number of tiles vertically

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy engine plugins
        .insert_resource(Grid::new(
            IVec2::new(GRID_WIDTH, GRID_HEIGHT),
            Vec2::new(TILE_SIZE, TILE_SIZE),
        ))
        .add_systems(Startup, (setup, spawn_grid, setup_tile_cursor, spawn_unit)) // Initial setup system
        .add_systems(Update, move_tile_cursor)
        .run();
}

fn setup(mut commands: Commands) {
    let grid_center = Vec2::new(
        (GRID_WIDTH as f32 * TILE_SIZE) / 2.0,
        (GRID_HEIGHT as f32 * TILE_SIZE) / 2.0,
    );
    // Spawn a 2D camera
    commands.spawn((
        Camera2d,
        Transform::from_xyz(grid_center.x, grid_center.y, 1000.0), // Z value is set to ensure proper rendering
    ));
}


fn spawn_unit(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the unit texture
    let unit_texture = asset_server.load("units/squirrel.png"); // Assuming you have a unit texture

    // Position the unit slightly above the grid at the first tile (position (0, 0))
    let unit_position = Vec2::new(0.0 * TILE_SIZE, 0.0 * TILE_SIZE + TILE_SIZE / 3.0); // Adjust for elevation above the grid

    commands.spawn((
        Sprite {
            image: unit_texture,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(unit_position.x, unit_position.y, 1.0), // Slightly above the grid
    ));
}
