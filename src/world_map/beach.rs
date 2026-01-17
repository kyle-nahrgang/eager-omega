use macroquad::prelude::*;

use crate::world_map::{
    layer::{
        TerrainCenterTileSet, TerrainCornerTileSet, TerrainEdgeTileSet, TerrainLayerGenerator,
    },
    tileset::TileIndex,
};

#[derive(Debug, Clone)]
pub struct BeachLayer {
    pub tiles: Vec<Vec<Option<TileIndex>>>,
    pub center: Vec2,
}

impl BeachLayer {
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

    pub fn new(width: usize, height: usize) -> Self
    where
        Self: Sized,
    {
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
