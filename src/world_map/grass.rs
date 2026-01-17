use macroquad::prelude::*;

use crate::world_map::{
    layer::{
        TerrainCenterTileSet, TerrainCornerTileSet, TerrainEdgeTileSet, TerrainLayerGenerator,
    },
    tileset::TileIndex,
};

#[derive(Debug, Clone)]
pub struct GrassLayer {
    pub tiles: Vec<Vec<Option<TileIndex>>>,
    pub center: Vec2,
}

impl GrassLayer {
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

impl TerrainEdgeTileSet for GrassLayer {
    fn top_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn top_right_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn right_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn bottom_right_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn bottom_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn bottom_left_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn left_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn top_left_edge() -> TileIndex {
        TileIndex::_GrassDark
    }
}

impl TerrainCornerTileSet for GrassLayer {
    fn top_right_corner() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn top_left_corner() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn bottom_right_corner() -> TileIndex {
        TileIndex::_GrassDark
    }
    fn bottom_left_corner() -> TileIndex {
        TileIndex::_GrassDark
    }
}

impl TerrainCenterTileSet for GrassLayer {
    fn center_tiles() -> &'static [TileIndex] {
        &[
            TileIndex::_GrassLight,
            TileIndex::_GrassSpottedLight1,
            TileIndex::_GrassSpottedLight2,
            TileIndex::_GrassSpottedLight3,
        ]
    }
}

impl TerrainLayerGenerator for GrassLayer {}
