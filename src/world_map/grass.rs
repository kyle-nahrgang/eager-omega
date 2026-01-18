use macroquad::prelude::*;

use crate::world_map::{
    layer::{
        LayerType, TerrainCenterTileSet, TerrainCornerTileSet, TerrainEdgeTileSet,
        TerrainLayerGenerator,
    },
    tileset::TileIndex,
};

#[derive(Debug, Clone)]
pub struct GrassLayer {
    pub tiles: Vec<Vec<Option<TileIndex>>>,
    pub center: Vec2,
    altitude: usize,
}

impl GrassLayer {
    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileIndex> {
        self.tiles[y][x]
    }

    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        return (
            vec2(0.0, 0.0),
            vec2(
                self.tiles[0].len() as f32 * 16.0,
                self.tiles.len() as f32 * 16.0,
            ),
        );
    }

    pub fn new(width: usize, height: usize, prev_layer: Option<&LayerType>) -> Self
    where
        Self: Sized,
    {
        let altitude = if let Some(LayerType::Grass(grass_layer)) = prev_layer {
            grass_layer.altitude + 1
        } else {
            0
        };

        let (center, tiles) = Self::generate_layer(width, height);
        Self {
            tiles,
            center,
            altitude,
        }
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
