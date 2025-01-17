mod gameboard;

use bevy::prelude::*;
use gameboard::grid::{spawn_grid, Grid};
use gameboard::tile_cursor::{
    check_tile_cursor_state, move_tile_cursor, setup_tile_cursor, CursorState,
};
use gameboard::unit::{move_unit, select_unit, spawn_squirrel, SelectedUnit};

use bevy::log::LogPlugin;

const TILE_SIZE: f32 = 32.0; // Size of each tile in pixels
const GRID_WIDTH: i32 = 20; // Number of tiles horizontally
const GRID_HEIGHT: i32 = 20; // Number of tiles vertically

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin::default())) // Bevy engine plugins
        .insert_resource(Grid::new(
            IVec2::new(GRID_WIDTH, GRID_HEIGHT),
            Vec2::new(TILE_SIZE, TILE_SIZE),
        ))
        .insert_resource(SelectedUnit(None))
        .insert_resource(CursorState {
            is_hover_a_unit: false,
            is_select_a_unit: false,
        })
        .add_systems(
            Startup,
            (setup_camera, spawn_grid, setup_tile_cursor, spawn_squirrel),
        ) // Initial setup system
        .add_systems(
            Update,
            (
                check_tile_cursor_state,
                move_tile_cursor,
                select_unit.before(move_unit),
                move_unit,
            ),
        );
    app.run();
}

fn setup_camera(mut commands: Commands) {
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
