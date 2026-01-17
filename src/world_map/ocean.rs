use macroquad::math::{Vec2, vec2};

use crate::world_map::tileset::TileIndex;

#[derive(Debug, Clone)]
pub struct OceanLayer {
    tiles: Vec<Vec<Option<TileIndex>>>,
}

impl OceanLayer {
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
        let tiles_map = [
            [
                TileIndex::Ocean1,
                TileIndex::Ocean2,
                TileIndex::Ocean3,
                TileIndex::Ocean4,
            ],
            [
                TileIndex::Ocean5,
                TileIndex::Ocean6,
                TileIndex::Ocean7,
                TileIndex::Ocean8,
            ],
            [
                TileIndex::Ocean9,
                TileIndex::Ocean10,
                TileIndex::Ocean11,
                TileIndex::Ocean12,
            ],
            [
                TileIndex::Ocean13,
                TileIndex::Ocean14,
                TileIndex::Ocean15,
                TileIndex::Ocean16,
            ],
        ];

        let mut tiles = vec![vec![None; width]; height];
        for y in 0..height {
            for x in 0..width {
                tiles[y][x] = Some(tiles_map[y % 4][x % 4]);
            }
        }

        Self { tiles }
    }
}
