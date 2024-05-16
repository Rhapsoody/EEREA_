use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Resource {
    Energy,
    Ore,
    PlaceOfInterest,
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
