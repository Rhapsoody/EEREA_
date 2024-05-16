use rand::Rng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};


use crate::tile::Tile;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
}

