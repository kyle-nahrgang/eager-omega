use macroquad::math::Vec2;

use crate::world_map::{layer::Layer, tileset::OceanTile};

pub struct Ocean {
    tiles: Vec<Vec<Option<u32>>>,
}

impl Layer for Ocean {
    fn is_collision(&self, _position: Vec2, _size: Vec2) -> bool {
        false
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<u32> {
        self.tiles[y][x]
    }
}

impl Ocean {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles_map = [
            [
                OceanTile::Ocean1,
                OceanTile::Ocean2,
                OceanTile::Ocean3,
                OceanTile::Ocean4,
            ],
            [
                OceanTile::Ocean5,
                OceanTile::Ocean6,
                OceanTile::Ocean7,
                OceanTile::Ocean8,
            ],
            [
                OceanTile::Ocean9,
                OceanTile::Ocean10,
                OceanTile::Ocean11,
                OceanTile::Ocean12,
            ],
            [
                OceanTile::Ocean13,
                OceanTile::Ocean14,
                OceanTile::Ocean15,
                OceanTile::Ocean16,
            ],
        ];

        let mut tiles = vec![vec![None; width]; height];
        for y in 0..height {
            for x in 0..width {
                tiles[y][x] = Some(tiles_map[y % 4][x % 4] as u32);
            }
        }

        Self { tiles }
    }
}
