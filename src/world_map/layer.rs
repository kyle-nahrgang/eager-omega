use macroquad::{
    math::{Vec2, vec2},
    prelude::rand,
    rand::ChooseRandom,
};

use crate::world_map::{
    beach::BeachLayer, grass::GrassLayer, ocean::OceanLayer, tileset::TileIndex,
};

#[derive(Debug, Clone)]
pub enum Layer {
    Ocean(OceanLayer),
    Beach(BeachLayer),
    Grass(GrassLayer),
}

macro_rules! delegate_layer {
    ($self:expr, $method:ident($($arg:expr),*)) => {
        match $self {
            Layer::Ocean(l) => l.$method($($arg),*),
            Layer::Beach(l) => l.$method($($arg),*),
            Layer::Grass(l) => l.$method($($arg),*),
        }
    };
}

impl Layer {
    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileIndex> {
        delegate_layer!(self, get_tile(x, y))
    }

    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        delegate_layer!(self, get_bounds())
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Layer::Ocean(_) => "Ocean",
            Layer::Beach(_) => "Beach",
            Layer::Grass(_) => "Grass",
        }
    }
}

pub trait TerrainCenterTileSet {
    fn center_tiles() -> &'static [TileIndex];
}

pub trait TerrainCornerTileSet {
    fn top_right_corner() -> TileIndex;
    fn top_left_corner() -> TileIndex;
    fn bottom_right_corner() -> TileIndex;
    fn bottom_left_corner() -> TileIndex;
}
pub trait TerrainEdgeTileSet {
    fn top_edge() -> TileIndex;
    fn top_right_edge() -> TileIndex;
    fn right_edge() -> TileIndex;
    fn bottom_right_edge() -> TileIndex;
    fn bottom_edge() -> TileIndex;
    fn bottom_left_edge() -> TileIndex;
    fn left_edge() -> TileIndex;
    fn top_left_edge() -> TileIndex;
}

pub trait TerrainLayerGenerator:
    TerrainCenterTileSet + TerrainCornerTileSet + TerrainEdgeTileSet
{
    fn generate_layer(width: usize, height: usize) -> (Vec2, Vec<Vec<Option<TileIndex>>>) {
        // todo: select starting tile based on previous layer
        let mut tiles = vec![vec![None; width]; height];

        let start_x = rand::RandomRange::gen_range(0, width as i32 / 2) as usize;
        let start_y = rand::RandomRange::gen_range(0, height as i32 / 2) as usize;

        // Random walk to create blob-shaped terrain
        let mut land_tiles = vec![];
        let center_x = start_x + width / 2;
        let center_y = start_y + width / 2;

        let center = vec2(
            (center_x as f32 + 0.5) * 16.0,
            (center_y as f32 + 0.5) * 16.0,
        );

        land_tiles.push((center_x, center_y));
        tiles[center_y][center_x] = Some(TileIndex::Sand);

        // Number of steps proportional to bounding box size
        let num_steps = rand::RandomRange::gen_range(width * height / 2, width * height);

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

                tiles[ny][nx] = Some(*Self::center_tiles().choose().unwrap());
                land_tiles.push((nx, ny));
            }
        }

        Self::fill_single_tile_gaps(&mut tiles);
        Self::populate_edges(&mut tiles);
        Self::populate_corners(&mut tiles);
        (center, tiles)
    }

    fn fill_single_tile_gaps(tiles: &mut Vec<Vec<Option<TileIndex>>>) {
        let height = tiles.len();
        let width = tiles[0].len();

        loop {
            // Work on a copy so we don't interfere while iterating
            let original = tiles.clone();
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
                            Some(*Self::center_tiles().choose().unwrap())
                        }
                        _ => continue,
                    };

                    tiles[y][x] = tile;
                }
            }

            if !found_gap {
                break;
            }
        }
    }

    fn populate_edges(tiles: &mut Vec<Vec<Option<TileIndex>>>) {
        let height = tiles.len();
        let width = tiles[0].len();

        // Work on a copy so we don't interfere while iterating
        let original = tiles.clone();

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
                    (false, true, true, true) | (false, true, false, false) => Self::top_edge(),
                    (true, false, true, true) | (true, false, false, false) => Self::bottom_edge(),
                    (true, true, false, true) | (false, false, false, true) => Self::left_edge(),
                    (true, true, true, false) | (false, false, true, false) => Self::right_edge(),
                    (false, true, false, true) => Self::top_left_edge(),
                    (false, true, true, false) => Self::top_right_edge(),
                    (true, false, false, true) => Self::bottom_left_edge(),
                    (true, false, true, false) => Self::bottom_right_edge(),
                    (false, false, false, false) => continue,
                    _ => {
                        println!(
                            "Unmatched edge case at ({}, {}): top={}, bottom={}, left={}, right={}",
                            x, y, top, bottom, left, right
                        );
                        continue;
                    }
                };

                tiles[y][x] = Some(tile);
            }
        }
    }

    fn populate_corners(tiles: &mut Vec<Vec<Option<TileIndex>>>) {
        let height = tiles.len();
        let width = tiles[0].len();

        // Work on a copy so we don't interfere while iterating
        let original = tiles.clone();

        for y in 0..height {
            for x in 0..width {
                // Only consider blank tiles
                if original[y][x].is_some() {
                    continue;
                }

                let top: bool =
                    y > 0 && original[y - 1][x].is_some_and(|t| t != Self::bottom_edge());
                let bottom: bool =
                    y + 1 < height && original[y + 1][x].is_some_and(|t| t != Self::top_edge());
                let left: bool =
                    x > 0 && original[y][x - 1].is_some_and(|t| t != Self::right_edge());
                let right: bool =
                    x + 1 < width && original[y][x + 1].is_some_and(|t| t != Self::left_edge());
                let tile = match (top, bottom, left, right) {
                    (false, true, false, true) => Self::top_left_corner(),
                    (false, true, true, false) => Self::top_right_corner(),
                    (true, false, false, true) => Self::bottom_left_corner(),
                    (true, false, true, false) => Self::bottom_right_corner(),
                    _ => {
                        continue;
                    }
                };

                tiles[y][x] = Some(tile);
            }
        }
    }
}
