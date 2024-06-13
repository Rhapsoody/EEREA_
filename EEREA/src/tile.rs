use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Resource {
    Energy,
    Ore,
    PlaceOfInterest,
    Empty,
}

impl Resource {
    // Generate a random resource using the provided RNG
    pub fn random_resource<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..4) {
            0 => Resource::Energy,
            1 => Resource::Ore,
            2 => Resource::PlaceOfInterest,
            _ => Resource::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile{
    pub explored: bool,
    pub has_obstacle: bool,
    pub resource: Option<Resource>,
}

impl Tile {
    pub fn new(explored: bool, has_obstacle: bool, resource: Option<Resource>) -> Self {
        Tile {
            explored,
            has_obstacle,
            resource,
        }
    }
}
