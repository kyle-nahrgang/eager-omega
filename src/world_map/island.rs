use rand::{Rng, seq::IndexedRandom};

use crate::world_map::{
    layer::Layer,
    tileset::{GrassTile, IslandTile},
};

pub struct Island {
    pub layer: Layer,
}

impl Island {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![0; width]; height];
        let mut rng = rand::thread_rng();

        // Max island size (1/6th of grid in tiles)
        let min_island_width = (width / 3).max(1);
        let min_island_height = (height / 3).max(1);
        let max_island_width = (width / 2).max(1);
        let max_island_height = (height / 2).max(1);

        // Random island bounding box (in tile coordinates)
        let island_width = rng.gen_range(min_island_width..=max_island_width);
        let island_height = rng.gen_range(min_island_height..=max_island_height);

        let start_x = rng.gen_range(0..=(width - island_width));
        let start_y = rng.gen_range(0..=(height - island_height));

        // Random walk to create blob-shaped island
        let mut land_tiles = vec![];
        let center_x = start_x + island_width / 2;
        let center_y = start_y + island_height / 2;

        land_tiles.push((center_x, center_y));
        tiles[center_y][center_x] = IslandTile::Sand as u32;

        // Number of steps proportional to bounding box size
        let num_steps =
            rng.gen_range((island_width * island_height / 2)..=(island_width * island_height));

        let center_options = vec![
            IslandTile::Sand,
            IslandTile::SandSpotted1,
            IslandTile::SandSpotted2,
            IslandTile::SandSpotted3,
        ];
        let rx = island_width as f32 * 0.5;
        let ry = island_height as f32 * 0.5;
        let cx = center_x as i32;
        let cy = center_y as i32;

        for _ in 0..num_steps {
            if let Some(&(x, y)) = land_tiles.choose(&mut rng) {
                let (nx, ny) = match rng.gen_range(0..4) {
                    0 => (x.saturating_sub(1), y),
                    1 => ((x + 1).min(width - 1), y),
                    2 => (x, y.saturating_sub(1)),
                    _ => (x, (y + 1).min(height - 1)),
                };

                if tiles[ny][nx] != 0 {
                    continue;
                }

                // NEW: roundness constraint
                if !Island::inside_radius(cx, cy, nx as i32, ny as i32, rx, ry) {
                    continue;
                }

                tiles[ny][nx] = *center_options.choose(&mut rng).unwrap() as u32;
                land_tiles.push((nx, ny));
            }
        }

        Island::add_island_edges(&mut tiles);

        Self {
            layer: Layer::new(width, height, tiles),
        }
    }

    fn inside_radius(cx: i32, cy: i32, x: i32, y: i32, rx: f32, ry: f32) -> bool {
        let dx = (x - cx) as f32 / rx;
        let dy = (y - cy) as f32 / ry;
        dx * dx + dy * dy <= 1.0
    }

    // Determine tile type based on neighbors
    fn add_island_edges(grid: &mut Vec<Vec<u32>>) {
        let height = grid.len();
        let width = grid[0].len();

        // Work on a copy so we don't interfere while iterating
        let original = grid.clone();

        for y in 0..height {
            for x in 0..width {
                // Only consider blank tiles
                if original[y][x] != 0 {
                    continue;
                }

                let top = y > 0 && original[y - 1][x] != 0;
                let bottom = y + 1 < height && original[y + 1][x] != 0;
                let left = x > 0 && original[y][x - 1] != 0;
                let right = x + 1 < width && original[y][x + 1] != 0;

                let tile = match (top, bottom, left, right) {
                    // Corners
                    (false, true, true, true) => IslandTile::SandEdgeTop,
                    (true, false, true, true) => IslandTile::SandEdgeBottom,
                    (true, true, false, true) => IslandTile::SandEdgeLeft,
                    (true, true, true, false) => IslandTile::SandEdgeRight,
                    (false, true, false, true) => IslandTile::SandCornerTopLeft,
                    (false, true, true, false) => IslandTile::SandCornerTopRight,
                    (true, false, false, true) => IslandTile::SandCornerBottomLeft,
                    (true, false, true, false) => IslandTile::SandCornerBottomRight,
                    _ => continue,
                };

                grid[y][x] = tile as u32;
            }
        }
    }
}
