use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::map::Map;
use crate::tile::{TileContent, Resource};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Robot {
    pub id: usize,
    pub position: (usize, usize),
    pub energy: u32,
    pub module: Module,
    pub behavior: Behavior,
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
        }
    }

    pub fn perform_action(&mut self, map: &mut Map) {
        if self.energy == 0 {
            println!("Robot {} has no energy and needs to recharge", self.id);
            // Ajouter la logique pour retourner à la station pour recharger
            return;
        }

        match self.behavior {
            Behavior::Exploration => self.explore(map),
            Behavior::ResourceCollection => self.collect_resource(map),
            Behavior::ScientificInterest => self.investigate(map),
        }

        self.energy -= 1; // Consommer de l'énergie pour chaque action
    }

    fn explore(&mut self, map: &mut Map) {
        println!("Robot {} exploring at position {:?}", self.id, self.position);
        self.move_randomly(map);
        self.mark_explored(map);
    }

    fn collect_resource(&mut self, map: &mut Map) {
        println!("Robot {} collecting resources at position {:?}", self.id, self.position);
        self.move_randomly(map);
        self.mine(map);
    }

    fn investigate(&mut self, map: &mut Map) {
        println!("Robot {} investigating at position {:?}", self.id, self.position);
        self.move_randomly(map);
        self.analyze();
    }
    
    // Fonctions spécifiques aux modules
    fn analyze(&self) {
        if let Module::Analysis = self.module {
            println!("Robot {} analyzing at position {:?}", self.id, self.position);
            // Logique d'analyse spécifique
        }
    }

    fn mine(&self, map: &mut Map) {
        if let Module::Mining = self.module {
            println!("Robot {} mining at position {:?}", self.id, self.position);
            if let Some(resource) = map.retrieve_resource_at(self.position.0, self.position.1) {
                println!("Collected resource: {:?}", resource);
            }
        }
    }

    fn capture_image(&self) {
        if let Module::Imaging = self.module {
            println!("Robot {} capturing image at position {:?}", self.id, self.position);
            // Logique de capture d'image spécifique
        }
    }

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

        for _ in 0..4 {  // Essayer jusqu'à 4 directions
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
        }
    }

    pub fn refill_energy(&mut self) {
        self.energy = 30; 
        println!("Robot {} is recharged", self.id);
    }
}