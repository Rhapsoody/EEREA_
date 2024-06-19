use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Resource {
    Energy,
    Ore,
    PlaceOfInterest,
}

impl Resource {
    // on va generer des ressources aleatoirement hop
    pub fn random_resource<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..3) {
            0 => Resource::Energy,
            1 => Resource::Ore,
            _ => Resource::PlaceOfInterest,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TileContent {
    Empty,
    Obstacle,
    Resource(Resource),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Tile {
    pub explored: bool,
    pub content: TileContent,
    pub timestamp: u64,
}

impl Tile {
    pub fn new(explored: bool, content: TileContent) -> Self {
        Tile {
            explored,
            content,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}