use macroquad::math::Vec2;

use crate::world_map::{layer::Layer, tileset::TileIndex};

pub struct Ocean {
    tiles: Vec<Vec<Option<TileIndex>>>,
}

impl Layer for Ocean {
    fn is_collision(&self, _position: Vec2, _size: Vec2) -> bool {
        false
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<TileIndex> {
        self.tiles[y][x]
    }

    fn new(_seed: &mut u64, width: usize, height: usize) -> Self
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
