use macroquad::{prelude::*, rand::srand};

use crate::world_map::{
    layer::{
        Layer, TerrainCenterTileSet, TerrainCornerTileSet, TerrainEdgeTileSet,
        TerrainLayerGenerator,
    },
    tileset::TileIndex,
};

pub struct Grass {
    pub tiles: Vec<Vec<Option<TileIndex>>>,
    pub center: Vec2,
}

impl Layer for Grass {
    fn is_collision(&self, position: Vec2, _size: Vec2) -> bool {
        let min_x = (position.x / 16.0).floor() as i32;
        let min_y = (position.y / 16.0).floor() as i32;

        false
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<TileIndex> {
        self.tiles[y][x]
    }

    fn new(seed: &mut u64, width: usize, height: usize) -> Self
    where
        Self: Sized,
    {
        *seed = *seed + 1;

        let (center, tiles) = Self::generate_layer(width, height);
        Self { tiles, center }
    }
}

impl TerrainEdgeTileSet for Grass {
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

impl TerrainCornerTileSet for Grass {
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

impl TerrainCenterTileSet for Grass {
    fn center_tiles() -> &'static [TileIndex] {
        &[
            TileIndex::_GrassLight,
            TileIndex::_GrassSpottedLight1,
            TileIndex::_GrassSpottedLight2,
            TileIndex::_GrassSpottedLight3,
        ]
    }
}

impl TerrainLayerGenerator for Grass {}
