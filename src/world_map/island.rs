use macroquad::{prelude::*, rand::ChooseRandom};

use crate::world_map::{layer::Layer, tileset::TileIndex};

pub struct Island {
    pub tiles: Vec<Vec<Option<TileIndex>>>,
    pub center: Vec2,
}

impl Layer for Island {
    fn is_collision(&self, position: Vec2, _size: Vec2) -> bool {
        let min_x = (position.x / 16.0).floor() as i32;
        let min_y = (position.y / 16.0).floor() as i32;

        if self.tiles[min_y as usize][min_x as usize].is_none() {
            return true;
        }
        false
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<TileIndex> {
        self.tiles[y][x]
    }
}

impl Island {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![None; width]; height];

        rand::srand(1);

        // Random island bounding box (in tile coordinates)
        let island_width = width; // rng.gen_range(min_island_width..=max_island_width);
        let island_height = width; // rng.gen_range(min_island_height..=max_island_height);

        let start_x = rand::RandomRange::gen_range(0, (width - island_width) as i32) as usize;
        let start_y = rand::RandomRange::gen_range(0, (height - island_height) as i32) as usize;

        // Random walk to create blob-shaped island
        let mut land_tiles = vec![];
        let center_x = start_x + island_width / 2;
        let center_y = start_y + island_height / 2;

        let center = vec2(
            (center_x as f32 + 0.5) * 16.0,
            (center_y as f32 + 0.5) * 16.0,
        );

        land_tiles.push((center_x, center_y));
        tiles[center_y][center_x] = Some(TileIndex::Sand);

        // Number of steps proportional to bounding box size
        let num_steps = rand::RandomRange::gen_range(
            island_width * island_height / 2,
            island_width * island_height,
        );

        let center_options = vec![
            TileIndex::Sand,
            TileIndex::SandSpotted1,
            TileIndex::SandSpotted2,
            TileIndex::SandSpotted3,
        ];

        let rx = island_width as f32 * 0.5;
        let ry = island_height as f32 * 0.5;
        let cx = center_x as i32;
        let cy = center_y as i32;

        for _ in 0..num_steps {
            if let Some(&(x, y)) = land_tiles.choose() {
                let (nx, ny) = match rand::gen_range(0, 4) {
                    0 => (x.saturating_sub(1), y),
                    1 => ((x + 1).min(width - 1), y),
                    2 => (x, y.saturating_sub(1)),
                    _ => (x, (y + 1).min(height - 1)),
                };

                if tiles[ny][nx].is_some() {
                    continue;
                }

                if !Island::inside_radius(cx, cy, nx as i32, ny as i32, rx, ry) {
                    continue;
                }

                tiles[ny][nx] = Some(*center_options.choose().unwrap());
                land_tiles.push((nx, ny));
            }
        }

        Self::fill_single_tile_gaps(&mut tiles);
        Self::add_island_edges(&mut tiles);
        Self::add_island_edge_corners(&mut tiles);

        Self { tiles, center }
    }

    fn inside_radius(cx: i32, cy: i32, x: i32, y: i32, rx: f32, ry: f32) -> bool {
        let dx = (x - cx) as f32 / rx;
        let dy = (y - cy) as f32 / ry;
        dx * dx + dy * dy <= 32.0
    }

    fn fill_single_tile_gaps(grid: &mut Vec<Vec<Option<TileIndex>>>) {
        let height = grid.len();
        let width = grid[0].len();

        loop {
            // Work on a copy so we don't interfere while iterating
            let original = grid.clone();
            let mut found_gap = false;

            for y in 0..height {
                for x in 0..width {
                    // Only consider blank tiles
                    if original[y][x].is_some() {
                        continue;
                    }

                    let top: bool = y > 0 && original[y - 1][x].is_some();
                    let bottom: bool = y + 1 < height && original[y + 1][x].is_some();
                    let left: bool = x > 0 && original[y][x - 1].is_some();
                    let right: bool = x + 1 < width && original[y][x + 1].is_some();

                    let tile = match (top, bottom, left, right) {
                        (true, true, true, true)
                        | (true, true, true, false)
                        | (true, true, false, true)
                        | (true, false, true, true)
                        | (false, true, true, true) => {
                            found_gap = true;
                            TileIndex::Sand
                        }
                        _ => continue,
                    };

                    grid[y][x] = Some(tile);
                }
            }

            if !found_gap {
                break;
            }
        }
    }

    // Determine tile type based on neighbors
    fn add_island_edges(grid: &mut Vec<Vec<Option<TileIndex>>>) {
        let height = grid.len();
        let width = grid[0].len();

        // Work on a copy so we don't interfere while iterating
        let original = grid.clone();

        for y in 0..height {
            for x in 0..width {
                // Only consider blank tiles
                if original[y][x].is_some() {
                    continue;
                }

                let top: bool = y > 0 && original[y - 1][x].is_some();
                let bottom: bool = y + 1 < height && original[y + 1][x].is_some();
                let left: bool = x > 0 && original[y][x - 1].is_some();
                let right: bool = x + 1 < width && original[y][x + 1].is_some();

                let tile = match (top, bottom, left, right) {
                    // Corners
                    (false, true, true, true) | (false, true, false, false) => {
                        TileIndex::SandEdgeTop
                    }
                    (true, false, true, true) | (true, false, false, false) => {
                        TileIndex::SandEdgeBottom
                    }
                    (true, true, false, true) | (false, false, false, true) => {
                        TileIndex::SandEdgeLeft
                    }
                    (true, true, true, false) | (false, false, true, false) => {
                        TileIndex::SandEdgeRight
                    }
                    (false, true, false, true) => TileIndex::SandEdgeTopLeft,
                    (false, true, true, false) => TileIndex::SandEdgeTopRight,
                    (true, false, false, true) => TileIndex::SandEdgeBottomLeft,
                    (true, false, true, false) => TileIndex::SandEdgeBottomRight,
                    (false, false, false, false) => continue,
                    _ => {
                        println!(
                            "Unmatched edge case at ({}, {}): top={}, bottom={}, left={}, right={}",
                            x, y, top, bottom, left, right
                        );
                        continue;
                    }
                };

                grid[y][x] = Some(tile);
            }
        }
    }

    fn add_island_edge_corners(grid: &mut Vec<Vec<Option<TileIndex>>>) {
        let height = grid.len();
        let width = grid[0].len();

        // Work on a copy so we don't interfere while iterating
        let original = grid.clone();

        for y in 0..height {
            for x in 0..width {
                // Only consider blank tiles
                if original[y][x].is_some() {
                    continue;
                }

                let top: bool = y > 0 && original[y - 1][x].is_some_and(|t| !t.is_bottom_edge());
                let bottom: bool =
                    y + 1 < height && original[y + 1][x].is_some_and(|t| !t.is_top_edge());
                let left: bool = x > 0 && original[y][x - 1].is_some();
                let right: bool = x + 1 < width && original[y][x + 1].is_some();

                let tile = match (top, bottom, left, right) {
                    (false, true, false, true) => TileIndex::SandCornerTopLeft,
                    (false, true, true, false) => TileIndex::SandCornerTopRight,
                    (true, false, false, true) => TileIndex::SandCornerBottomLeft,
                    (true, false, true, false) => TileIndex::SandCornerBottomRight,
                    _ => {
                        continue;
                    }
                };

                grid[y][x] = Some(tile);
            }
        }
    }
}
