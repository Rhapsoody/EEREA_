use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use noise::{NoiseFn, Perlin};
use crate::tile::{Tile, TileContent, Resource};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        let tiles = Self::generate_tiles(width, height, seed);
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn generate_tiles(width: usize, height: usize, seed: u64) -> Vec<Vec<Tile>> {
        let mut rand = rand::rngs::StdRng::seed_from_u64(seed);
        let mut tiles = vec![vec![Tile::new(false, TileContent::Empty); width]; height];

        let perlin = Perlin::new(8);

        // Bordures comme obstacles
        for x in 0..width {
            tiles[0][x] = Tile::new(false, TileContent::Obstacle);
            tiles[height - 1][x] = Tile::new(false, TileContent::Obstacle);
        }

        for y in 0..height {
            tiles[y][0] = Tile::new(false, TileContent::Obstacle);
            tiles[y][width - 1] = Tile::new(false, TileContent::Obstacle);
        }

        
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let perlin_noise = perlin.get([x as f64 / 15.0 as f64, y as f64 / 12.0 as f64]);
                if perlin_noise > 0.5 {
                    tiles[y][x] = Tile::new(false, TileContent::Obstacle);
                } else {
                    let resource_probability: f64 = rand.gen();
                    if resource_probability < 0.3 {
                        let resource = Resource::random_resource(&mut rand);
                        tiles[y][x] = Tile::new(false, TileContent::Resource(resource));
                    }
                }
            }
        }

        tiles
    }

    pub fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        if !self.check_bounds(x, y) {
            return false; 
        }
        matches!(self.tiles[y][x].content, TileContent::Empty)
    }

    pub fn throw_resource_at(&mut self, x: usize, y: usize, resource: Resource) {
        if let Some(tile) = self.tile_at_mut(x, y) {
            tile.content = TileContent::Resource(resource);
        }
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        if self.check_bounds(x, y) {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn tile_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if self.check_bounds(x, y) {
            Some(&mut self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn retrieve_resource_at(&mut self, x: usize, y: usize) -> Option<Resource> {
        if let Some(tile) = self.tile_at_mut(x, y) {
            if let TileContent::Resource(resource) = tile.content {
                tile.content = TileContent::Empty;
                Some(resource)
            } else {
                None
            }
        } else {
            None
        }
    }
}