use crate::gameboard::grid::Grid;
use bevy::prelude::*;

pub enum UnitType {
    Squirrel,
}

#[derive(Component)]
pub struct Unit {
    pub position: Vec2,
    unit_type: UnitType,
}

impl Unit {
    fn new(position: Vec2, unit_type: UnitType) -> Self {
        Self { position, unit_type }
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
