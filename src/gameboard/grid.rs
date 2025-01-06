use bevy::{
    math::{IVec2, Vec2},
    prelude::*,
};

#[derive(Debug, Resource)]
pub struct Grid {
    pub size: IVec2,     // Grid size in rows and columns
    pub cell_size: Vec2, // Size of each cell in pixels
    pub half_cell_size: Vec2,
}

impl Grid {
    pub fn new(size: IVec2, cell_size: Vec2) -> Self {
        Self {
            size,
            cell_size,
            half_cell_size: cell_size / 2.0,
        }
    }

    /// Calculates the position of a cell's center in pixels on the map.
    pub fn calculate_map_position(&self, grid_position: Vec2) -> Vec2 {
        grid_position * self.cell_size + self.half_cell_size
    }

    /// Calculates the grid coordinates from a map position in pixels.
    pub fn calculate_grid_coordinates(&self, map_position: Vec2) -> Vec2 {
        (map_position / self.cell_size).floor()
    }

    /// Checks if the given cell coordinates are within the grid's bounds.
    fn is_within_bounds(&self, cell_coordinates: Vec2) -> bool {
        cell_coordinates.x >= 0.0
            && cell_coordinates.x < self.size.x as f32
            && cell_coordinates.y >= 0.0
            && cell_coordinates.y < self.size.y as f32
    }

    /// Clamps the given grid position to the grid's bounds.
    fn clamp(&self, grid_position: Vec2) -> Vec2 {
        Vec2::new(
            grid_position.x.clamp(0.0, self.size.x as f32 - 1.0),
            grid_position.y.clamp(0.0, self.size.y as f32 - 1.0),
        )
    }

    /// Converts 2D grid coordinates to a 1D index.
    /// Useful for pathfinding or performance optimization.
    fn as_index(&self, cell: Vec2) -> Option<usize> {
        if self.is_within_bounds(cell) {
            Some((cell.x + self.size.x as f32 * cell.y) as usize)
        } else {
            None
        }
    }
}

pub fn spawn_grid(mut commands: Commands, grid: Res<Grid>, asset_server: Res<AssetServer>) {
    for y in 0..grid.size.y {
        for x in 0..grid.size.x {
            let grid_texture = asset_server.load("tiles/grid_tile.png");
            let position = Vec2::new(x as f32 * grid.cell_size.x, y as f32 * grid.cell_size.y);
            commands.spawn((
                Sprite {
                    image: grid_texture,
                    custom_size: Some(Vec2::new(grid.cell_size.x, grid.cell_size.y)),
                    image_mode: SpriteImageMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 0.5,
                    },
                    ..Default::default()
                },
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
}
