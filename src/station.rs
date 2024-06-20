use crate::map::Map;
use crate::robot::{Robot, Module, Behavior};


#[derive(Debug)]
pub struct Station {
    pub position: (usize, usize),
    pub energy: u32,
    pub robots: Vec<Robot>,
    pub known_tiles: Vec<KnownTile>, 
}

#[derive(Debug)]
pub struct KnownTile {
    pub x: usize,
    pub y: usize,
    pub timestamp: u64,
}

impl Station {
    pub fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            energy: 0,
            robots: vec![],
            known_tiles: vec![],
        }
    }

    // petite fonction de collecte des données des robots qu reviennent à la station
    pub fn collect_data(&mut self, robot: &Robot, map: &Map) {
        for &(x, y) in &robot.known_tiles {
            if let Some(tile) = map.tile_at(x, y) {
                if tile.explored {
                    let known_tile = self.known_tiles.iter_mut().find(|t| t.x == x && t.y == y);
                    match known_tile {
                        Some(existing_tile) => {
                            if tile.timestamp > existing_tile.timestamp {
                                existing_tile.timestamp = tile.timestamp;
                            }
                        }
                        None => {
                            self.known_tiles.push(KnownTile { x, y, timestamp: tile.timestamp });
                        }
                    }
                }
            }
        }
    }

    

    // la station doit pouvoir créer des robots 
    pub fn create_robot(&mut self, id: usize, position: (usize, usize), module: Module, behavior: Behavior) -> Robot {
        let robot = Robot::new(id, position, 100, module, behavior);
        self.robots.push(robot.clone());
        robot
    }

    // la c'est pour la fonction de partages des données des points connus avec les robots
    pub fn share_data(&self, robot: &mut Robot) {
        for tile in &self.known_tiles {
            robot.mark_tile_as_known(tile.x, tile.y);
        }
    }

    // La on ajoute de l'energie à la station
    pub fn add_energy(&mut self, amount: u32) {
        self.energy += amount;
    }
}