use rand::{Rng, seq::IndexedRandom};

use crate::world_map::{layer::Layer, tileset::GrassTile};

pub struct Island {
    pub layer: Layer,
}

impl Island {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![0; width]; height];
        let mut rng = rand::thread_rng();

        // Max island size (1/6th of grid in tiles)
        let min_island_width = (width / 3).max(1);
        let min_island_height = (height / 3).max(1);
        let max_island_width = (width / 2).max(1);
        let max_island_height = (height / 2).max(1);

        // Random island bounding box (in tile coordinates)
        let island_width = rng.gen_range(min_island_width..=max_island_width);
        let island_height = rng.gen_range(min_island_height..=max_island_height);

        let start_x = rng.gen_range(0..=(width - island_width));
        let start_y = rng.gen_range(0..=(height - island_height));

        // Random walk to create blob-shaped island
        let mut land_tiles = vec![];
        let center_x = start_x + island_width / 2;
        let center_y = start_y + island_height / 2;

        land_tiles.push((center_x, center_y));
        tiles[center_y][center_x] = GrassTile::GrassLight as u32;

        let options = vec![
            GrassTile::GrassSpottedLight1,
            GrassTile::GrassSpottedLight2,
            GrassTile::GrassSpottedLight3,
            GrassTile::GrassSpottedLight4,
        ];

        // Number of steps proportional to bounding box size
        let num_steps =
            rng.gen_range((island_width * island_height / 2)..=(island_width * island_height));

        for _ in 0..num_steps {
            if let Some(&(x, y)) = land_tiles.choose(&mut rng) {
                // Random neighbor tile
                let (nx, ny) = match rng.gen_range(0..4) {
                    0 => (x.saturating_sub(1), y),     // left
                    1 => ((x + 1).min(width - 1), y),  // right
                    2 => (x, y.saturating_sub(1)),     // up
                    _ => (x, (y + 1).min(height - 1)), // down
                };

                // Only add tile if it's inside the island bounding box and empty
                if nx >= start_x
                    && nx < start_x + island_width
                    && ny >= start_y
                    && ny < start_y + island_height
                    && tiles[ny][nx] == 0
                {
                    // Assign a random grass type
                    tiles[ny][nx] = options.choose(&mut rng).unwrap().clone() as u32;
                    land_tiles.push((nx, ny));
                }
            }
        }

        Self {
            layer: Layer::new(width, height, tiles),
        }
    }
}
