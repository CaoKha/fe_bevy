use super::grid::Grid;
use bevy::prelude::*;

// Cursor component that holds its position
#[derive(Component)]
pub struct TileCursor {
    pub position: Vec2,
}

impl TileCursor {
    pub fn new(start_pos: Vec2) -> Self {
        Self {
            position: start_pos,
        }
    }
}

// Setup the cursor entity with a texture and initial position
pub fn setup_tile_cursor(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<Grid>) {
    let cursor_start_pos = Vec2::new(0.0, 0.0); // Start at the top-left corner of the grid

    // Load the grid texture (same as the grid tile texture)
    let texture_handle = asset_server.load("tiles/grid_tile.png"); // Path to your texture

    commands
        .spawn((
            Sprite {
                image: texture_handle.clone(),
                custom_size: Some(Vec2::new(grid.cell_size.x, grid.cell_size.y)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                cursor_start_pos.x * grid.cell_size.x,
                cursor_start_pos.y * grid.cell_size.y,
                1.0,
            )),
        ))
        .insert(TileCursor::new(cursor_start_pos));
}

// Move the cursor based on arrow key inputs
pub fn move_tile_cursor(
    mut cursor_query: Query<(&mut Transform, &mut TileCursor, &mut Sprite)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid: Res<Grid>,
) {
    for (mut transform, mut cursor, mut sprite) in cursor_query.iter_mut() {
        // Move cursor based on arrow key inputs
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            cursor.position.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            cursor.position.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            cursor.position.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            cursor.position.y -= 1.0;
        }

        // Clamp the cursor's position within the grid bounds
        cursor.position.x = cursor.position.x.clamp(0.0, grid.size.x as f32 - 1.0);
        cursor.position.y = cursor.position.y.clamp(0.0, grid.size.y as f32 - 1.0);

        // Update the cursor's transform to reflect its new position
        transform.translation = Vec3::new(
            cursor.position.x * grid.cell_size.x,
            cursor.position.y * grid.cell_size.y,
            1.0,
        );

        // Add a border highlight when selected by changing the cursor color
        sprite.color = if cursor.position.x % 2.0 == 0.0 && cursor.position.y % 2.0 == 0.0 {
            Color::srgb(0.9, 0.0, 0.0) // Highlight color (red for example)
        } else {
            Color::WHITE // No highlight (normal state)
        };
    }
}
