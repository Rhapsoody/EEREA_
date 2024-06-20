use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::map::Map;
use crate::tile::TileContent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Robot {
    pub id: usize,
    pub position: (usize, usize),
    pub energy: u32,
    pub module: Module,
    pub behavior: Behavior,
    pub known_tiles: Vec<(usize, usize)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Module {
    Analysis,
    Mining,
    Imaging,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Behavior {
    Exploration,
    ResourceCollection,
    ScientificInterest,
}

impl Robot {
    pub fn new(id: usize, position: (usize, usize), energy: u32, module: Module, behavior: Behavior) -> Self {
        Self {
            id,
            position,
            energy,
            module,
            behavior,
            known_tiles: vec![]
        }
    }
    // on gere les comportements des petits robots
    pub fn perform_action(&mut self, map: &mut Map, station_position: (usize, usize)) {
        if self.energy == 0 {
            println!("Robot {} has no energy and needs to recharge", self.id);
            self.go_back_to_station(station_position, map);
            if self.position == station_position {
                self.refill_energy();
            }
            return;
        }

        match self.behavior {
            Behavior::Exploration => self.explore(map),
            Behavior::ResourceCollection => self.collect_resource(map),
            Behavior::ScientificInterest => self.investigate(map),
        }

        self.energy -= 1; // et bim on consomme de l'energie
    }

    fn explore(&mut self, map: &mut Map) {
        println!("Robot {} exploring at position {:?}", self.id, self.position);
        self.move_randomly(map);
        self.mark_explored(map);
    }

    fn collect_resource(&mut self, map: &mut Map) {
        println!("Robot {} collecting resources at position {:?}", self.id, self.position);
        if !self.known_tiles.contains(&self.position) {
            self.mine(map);
        }
        self.move_randomly(map);
    }

    fn investigate(&mut self, map: &mut Map) {
        println!("Robot {} investigating at position {:?}", self.id, self.position);
        self.move_randomly(map);
        // self.analyze();
    }
    
    
    // fn analyze(&self) {
    //     if let Module::Analysis = self.module {
    //         println!("Robot {} analyzing at position {:?}", self.id, self.position);
    //         
    //     }
    // }

    fn mine(&self, map: &mut Map) {
        if let Module::Mining = self.module {
            println!("Robot {} mining at position {:?}", self.id, self.position);
            if let Some(resource) = map.retrieve_resource_at(self.position.0, self.position.1) {
                println!("Collected resource: {:?}", resource);
            }
        }
    }

    // fn capture_image(&self) {
    //     if let Module::Imaging = self.module {
    //         println!("Robot {} capturing image at position {:?}", self.id, self.position);
    //         
    //     }
    // }

    fn go_back_to_station(&mut self, target: (usize, usize), map: &Map) {
        let dx = target.0 as isize - self.position.0 as isize;
        let dy = target.1 as isize - self.position.1 as isize;

        let step_x = if dx != 0 { dx / dx.abs() } else { 0 };
        let step_y = if dy != 0 { dy / dy.abs() } else { 0 };

        let new_x = (self.position.0 as isize + step_x).clamp(0, (map.width - 1) as isize) as usize;
        let new_y = (self.position.1 as isize + step_y).clamp(0, (map.height - 1) as isize) as usize;

        if map.tiles[new_y][new_x].content != TileContent::Obstacle {
            self.position = (new_x, new_y);
        }
    }

    
    fn move_randomly(&mut self, map: &Map) {
        let mut rng = rand::thread_rng();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for _ in 0..4 {  
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let new_x = (self.position.0 as isize + dx).clamp(0, (map.width - 1) as isize) as usize;
            let new_y = (self.position.1 as isize + dy).clamp(0, (map.height - 1) as isize) as usize;

            if map.tiles[new_y][new_x].content != TileContent::Obstacle {
                self.position = (new_x, new_y);
                break;
            }
        }
    }

    fn mark_explored(&mut self, map: &mut Map) {
        if let Some(tile) = map.tile_at_mut(self.position.0, self.position.1) {
            tile.explored = true;
            tile.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.mark_tile_as_known(self.position.0, self.position.1);
        }
    }

    pub fn mark_tile_as_known(&mut self, x: usize, y: usize) {
        if !self.known_tiles.contains(&(x, y)) {
            self.known_tiles.push((x, y));
        }
    }

    pub fn refill_energy(&mut self) {
        self.energy = 80; 
        println!("Robot {} is recharged", self.id);
    }
}