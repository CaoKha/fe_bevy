use crate::gameboard::grid::Grid;
use bevy::prelude::*;

use super::tile_cursor::{CursorState, TileCursor};

#[derive(Debug)]
pub enum UnitType {
    Squirrel,
}

#[derive(Component)]
pub struct Unit {
    pub position: Vec2,
    unit_type: UnitType,
}

#[derive(Resource)]
pub struct SelectedUnit(pub Option<Entity>);

impl Unit {
    fn new(position: Vec2, unit_type: UnitType) -> Self {
        Self {
            position,
            unit_type,
        }
    }
}

pub fn spawn_squirrel(mut commands: Commands, grid: Res<Grid>, asset_server: Res<AssetServer>) {
    // Load the unit texture
    let squirrel_img = asset_server.load("units/squirrel.png"); // Assuming you have a unit texture

    // Position the unit slightly above the grid at the first tile (position (0, 0))
    let squirrel_position = Vec2::new(0.0 * grid.cell_size.x, 0.0 * grid.cell_size.y);

    commands.spawn((
        Sprite {
            image: squirrel_img,
            custom_size: Some(Vec2::new(grid.cell_size.x, grid.cell_size.y)),
            ..Default::default()
        },
        Transform::from_xyz(squirrel_position.x, squirrel_position.y, 1.0), // Slightly above the grid
        Unit::new(squirrel_position, UnitType::Squirrel),
    ));
}

pub fn move_unit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cursor_query: Query<&TileCursor>,
    mut units_query: Query<(&mut Unit, &mut Transform)>,
    mut tile_cursor_state: ResMut<CursorState>,
    grid: Res<Grid>,
    mut selected_unit: ResMut<SelectedUnit>,
) {
    if tile_cursor_state.is_select_a_unit && keyboard_input.just_pressed(KeyCode::KeyX) {
        if let Ok(cursor) = cursor_query.get_single() {
            if let Some(entity) = selected_unit.0 {
                if let Ok((mut unit, mut transform)) = units_query.get_mut(entity) {
                    if cursor.position != unit.position {
                        unit.position = cursor.position;
                        transform.translation.x = cursor.position.x * grid.cell_size.x;
                        transform.translation.y = cursor.position.y * grid.cell_size.y;
                        // Ensure unit's position is updated visually
                        debug!("Unit moved to: ({}, {})", unit.position.x, unit.position.y);
                        tile_cursor_state.is_select_a_unit = false;
                        selected_unit.0 = None;
                        debug!("Auto deselect unit after movement");
                    }
                }
            }
        }
    }
}

pub fn select_unit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tile_cursor_query: Query<&TileCursor>,
    unit_query: Query<(&Unit, Entity)>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut tile_cursor_state: ResMut<CursorState>,
) {
    if tile_cursor_state.is_hover_a_unit && keyboard_input.just_pressed(KeyCode::KeyX) {
        if let Ok(cursor) = tile_cursor_query.get_single() {
            for (unit, entity) in unit_query.iter() {
                if unit.position == cursor.position {
                    tile_cursor_state.is_select_a_unit = true;
                    selected_unit.0 = Some(entity);
                    debug!(
                        "Unit selected is a {:?}: ({}, {})",
                        unit.unit_type, unit.position.x, unit.position.y
                    );
                    break;
                }
            }
        }
    }
}
