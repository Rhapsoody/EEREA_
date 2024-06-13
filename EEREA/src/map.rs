
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use noise::{NoiseFn, Perlin};


use crate::tile::Tile;
use crate::tile::Resource;


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
        let mut tiles = vec![vec![Tile::new(false, false, None); width as usize]; height as usize];

        let perlin = Perlin::new(1);

        for x in 0..width {
            tiles[0][x].has_obstacle = true;
            tiles[height - 1][x].has_obstacle = true;
        }

        for y in 0..height {
            tiles[y][0].has_obstacle = true;
            tiles[y][width - 1].has_obstacle = true;
        }

        for y in 0..height {
            for x in 0..width {
                let perlin_noise = perlin.get([x as f64 / width as f64, y as f64 / height as f64]);
                if perlin_noise > 0.5 {
                    tiles[y][x].has_obstacle = true;
                }
                    
            }
        }

        // Randomly distribute resources
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if !tiles[y][x].has_obstacle {
                    tiles[y][x].resource = Some(Resource::random_resource(&mut rand));
                }
            }
        }
        tiles
    }

    pub fn display_map(&self) {
        let mut output = String::new();

        for row in &self.tiles {
            for tile in row {
                let content = if tile.has_obstacle {
                    " 🚧 " 
                } else {
                    match tile.resource {
                        Some(Resource::Energy) => " 🔥 ", 
                        Some(Resource::Ore) => " 💎 ", 
                        Some(Resource::PlaceOfInterest) => " 🛰️ ", 
                        Some(Resource::Empty) => "     ",
                        None => " - ",
                    }
                };
                output.push_str(content);
            }
            output.push('\n');
        }
        print!("{}", output);
    }

    pub fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        if !self.check_bounds(x, y) {
            return false; 
        }
        !self.tiles[y][x].has_obstacle && self.tiles[y][x].resource.is_none()
    }

    pub fn throw_resource_at(&mut self, x: usize, y: usize, resource: Resource) {
        if let Some(tile) = self.tile_at_mut(x, y) {
            tile.resource = Some(resource);
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
            let resource = tile.resource;
            tile.resource = None;
            resource
        } else {
            None
        }
    }    

}