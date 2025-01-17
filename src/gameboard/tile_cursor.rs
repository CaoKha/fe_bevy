use super::grid::Grid;
use super::unit::Unit;
use bevy::prelude::*;

// Cursor component that holds its position
#[derive(Component)]
pub struct TileCursor {
    pub position: Vec2,
}

#[derive(Resource)]
pub struct MovementTimer {
    delay_timer: Timer,
    repeat_timer: Timer,
}

#[derive(Resource)]
pub struct CursorState {
    pub is_hover_a_unit: bool,
    pub is_select_a_unit: bool,
}

impl TileCursor {
    fn new(start_pos: Vec2) -> Self {
        Self {
            position: start_pos,
        }
    }
}

// Setup the cursor entity with a texture and initial position
pub fn setup_tile_cursor(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<Grid>) {
    spawn_tile_cursor(&mut commands, asset_server, grid);
    setup_tile_cursor_timer(&mut commands);
}

fn spawn_tile_cursor(commands: &mut Commands, asset_server: Res<AssetServer>, grid: Res<Grid>) {
    let cursor_start_pos = Vec2::new(0.0, 0.0); // Start at the top-left corner of the grid

    // Load the grid texture (same as the grid tile texture)
    let tile_cursor_img = asset_server.load("cursors/tile_cursor.png"); // Path to your texture

    commands
        .spawn((
            Sprite {
                image: tile_cursor_img.clone(),
                custom_size: Some(Vec2::new(grid.cell_size.x, grid.cell_size.y)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                cursor_start_pos.x * grid.cell_size.x,
                cursor_start_pos.y * grid.cell_size.y,
                0.5,
            )),
        ))
        .insert(TileCursor::new(cursor_start_pos));
}

fn setup_tile_cursor_timer(commands: &mut Commands) {
    commands.insert_resource(MovementTimer {
        delay_timer: Timer::from_seconds(0.5, TimerMode::Once),
        repeat_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });
}

// Move the cursor based on arrow key inputs
pub fn move_tile_cursor(
    mut cursor_query: Query<(&mut Transform, &mut TileCursor, &mut Sprite)>,
    unit_query: Query<&Unit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid: Res<Grid>,
    time: Res<Time>,
    mut movement_timer: ResMut<MovementTimer>,
) {
    for (mut transform, mut cursor, mut sprite) in cursor_query.iter_mut() {
        let mut moved = false;
        movement_timer.delay_timer.tick(time.delta());
        movement_timer.repeat_timer.tick(time.delta());
        // Move cursor based on arrow key inputs
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            cursor.position.x -= 1.0;
            moved = true;
            movement_timer.delay_timer.reset();
            movement_timer.repeat_timer.reset();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            cursor.position.x += 1.0;
            moved = true;
            movement_timer.delay_timer.reset();
            movement_timer.repeat_timer.reset();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            cursor.position.y += 1.0;
            moved = true;
            movement_timer.delay_timer.reset();
            movement_timer.repeat_timer.reset();
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            cursor.position.y -= 1.0;
            moved = true;
            movement_timer.delay_timer.reset();
            movement_timer.repeat_timer.reset();
        }

        if movement_timer.delay_timer.finished() && movement_timer.repeat_timer.finished() {
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                cursor.position.x -= 1.0;
                moved = true;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                cursor.position.x += 1.0;
                moved = true;
            } else if keyboard_input.pressed(KeyCode::ArrowUp) {
                cursor.position.y += 1.0;
                moved = true;
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                cursor.position.y -= 1.0;
                moved = true;
            }

            if moved {
                movement_timer.repeat_timer.reset();
            }
        }

        if moved {
            // Clamp the cursor's position within the grid bounds
            cursor.position.x = cursor.position.x.clamp(0.0, grid.size.x as f32 - 1.0);
            cursor.position.y = cursor.position.y.clamp(0.0, grid.size.y as f32 - 1.0);

            // Update the cursor's transform to reflect its new position
            transform.translation = Vec3::new(
                cursor.position.x * grid.cell_size.x,
                cursor.position.y * grid.cell_size.y,
                0.5,
            );

            let mut is_overlapping_with_unit = false;
            for unit in unit_query.iter() {
                if unit.position == cursor.position {
                    is_overlapping_with_unit = true;
                    break;
                }
            }
            // Add a border highlight when selected by changing the cursor color
            sprite.color = if is_overlapping_with_unit {
                Color::srgb(0.9, 0.0, 0.0) // Highlight color (red for example)
            } else {
                Color::WHITE // No highlight (normal state)
            };
        }
    }
}

pub fn check_tile_cursor_state(
    tile_cursor_query: Query<&TileCursor, Changed<TileCursor>>,
    unit_query: Query<&Unit>,
    mut tile_cursor_state: ResMut<CursorState>,
) {
    for cursor in tile_cursor_query.iter() {
        for unit in unit_query.iter() {
            if cursor.position == unit.position {
                tile_cursor_state.is_hover_a_unit = true;
                debug!("cursor is hovering a unit");
            }
        }
    }
}
