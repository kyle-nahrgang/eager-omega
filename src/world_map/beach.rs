use macroquad::prelude::*;

use crate::world_map::{
    layer::{
        TerrainCenterTileSet, TerrainCornerTileSet, TerrainEdgeTileSet, TerrainLayerGenerator,
    },
    tileset::TileIndex,
};

pub struct BeachLayer {
    pub tiles: Vec<Vec<Option<TileIndex>>>,
    pub center: Vec2,
}

impl BeachLayer {
    pub fn is_walkable(&self, position: Vec2, _size: Vec2) -> bool {
        let min_x = (position.x / 16.0).floor() as i32;
        let min_y = (position.y / 16.0).floor() as i32;

        if min_x >= self.tiles[0].len() as i32 || min_y >= self.tiles.len() as i32 {
            return true;
        }

        if self.tiles[min_y as usize][min_x as usize].is_none() {
            return true;
        }

        false
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileIndex> {
        self.tiles[y][x]
    }

    pub fn new(seed: &mut u64, width: usize, height: usize) -> Self
    where
        Self: Sized,
    {
        *seed = *seed + 1;

        let (center, tiles) = Self::generate_layer(width, height);
        Self { tiles, center }
    }
}

impl TerrainEdgeTileSet for BeachLayer {
    fn top_edge() -> TileIndex {
        TileIndex::SandEdgeTop
    }
    fn top_right_edge() -> TileIndex {
        TileIndex::SandEdgeTopRight
    }
    fn right_edge() -> TileIndex {
        TileIndex::SandEdgeRight
    }
    fn bottom_right_edge() -> TileIndex {
        TileIndex::SandEdgeBottomRight
    }
    fn bottom_edge() -> TileIndex {
        TileIndex::SandEdgeBottom
    }
    fn bottom_left_edge() -> TileIndex {
        TileIndex::SandEdgeBottomLeft
    }
    fn left_edge() -> TileIndex {
        TileIndex::SandEdgeLeft
    }
    fn top_left_edge() -> TileIndex {
        TileIndex::SandEdgeTopLeft
    }
}

impl TerrainCornerTileSet for BeachLayer {
    fn top_right_corner() -> TileIndex {
        TileIndex::SandCornerTopRight
    }
    fn top_left_corner() -> TileIndex {
        TileIndex::SandCornerTopLeft
    }
    fn bottom_right_corner() -> TileIndex {
        TileIndex::SandCornerBottomRight
    }
    fn bottom_left_corner() -> TileIndex {
        TileIndex::SandCornerBottomLeft
    }
}

impl TerrainCenterTileSet for BeachLayer {
    fn center_tiles() -> &'static [TileIndex] {
        &[
            TileIndex::Sand,
            TileIndex::SandSpotted1,
            TileIndex::SandSpotted2,
            TileIndex::SandSpotted3,
        ]
    }
}

impl TerrainLayerGenerator for BeachLayer {}
