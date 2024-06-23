use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::map::Map;
use crate::tile::{Resource, TileContent};

use std::collections::VecDeque;


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
            self.move_towards_goal(map, station_position);
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

        self.energy -= 1; 
    }

    fn find_random_goal(&self, map: &Map) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..map.width);
            let y = rng.gen_range(0..map.height);
            if map.tiles[y][x].content != TileContent::Obstacle {
                return (x, y);
            }
        }
    }

    pub fn move_towards_goal(&mut self, map: &Map, goal: (usize, usize)) {
        if let Some(path) = move_using_bfs(map, self.position, goal) {
            if path.len() > 1 {
                let next_step = path[1];
                self.position = next_step;
            }
        } else {
            self.move_randomly(map);
        }
    }

    pub fn find_resource(&self, map: &Map) -> Option<(usize, usize)> {
        for y in 0..map.height {
            for x in 0..map.width {
                if let TileContent::Resource(_) = map.tiles[y][x].content {
                    if !self.known_tiles.contains(&(x, y)) {
                        return Some((x, y));
                    }
                }
            }
        }
        None 
    }

    fn find_point_of_interest(&self, map: &Map) -> (usize, usize) {
        for y in 0..map.height {
            for x in 0..map.width {
                if let TileContent::Resource(Resource::PlaceOfInterest) = map.tiles[y][x].content {
                    return (x, y);
                }
            }
        }
        self.position
    }

    fn explore(&mut self, map: &mut Map) {
        println!("Robot {} exploring at position {:?}", self.id, self.position);
        let goal = self.find_random_goal(map);
        self.move_towards_goal(map, goal);
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
        let goal = self.find_point_of_interest(map);
        self.move_towards_goal(map, goal);
        // self.analyze(); 
    }

    fn mine(&self, map: &mut Map) {
        if let Module::Mining = self.module {
            println!("Robot {} mining at position {:?}", self.id, self.position);
            if let Some(resource) = map.retrieve_resource_at(self.position.0, self.position.1) {
                println!("Collected resource: {:?}", resource);
            }
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

    pub fn mark_explored(&mut self, map: &mut Map) {
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

 fn move_using_bfs(map: &Map, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    let mut came_from = std::collections::HashMap::new();

    queue.push_back(start);
    came_from.insert(start, None);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = Vec::new();
            let mut current = current;
            while let Some(&Some(prev)) = came_from.get(&current) {
                path.push(current);
                current = prev;
            }
            path.reverse();
            return Some(path);
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for &direction in &directions {
            let next = (
                (current.0 as isize + direction.0) as usize,
                (current.1 as isize + direction.1) as usize,
            );

            if map.check_bounds(next.0, next.1)
                && map.tiles[next.1][next.0].content != TileContent::Obstacle
                && !came_from.contains_key(&next)
            {
                queue.push_back(next);
                came_from.insert(next, Some(current));
            }
        }
    }

    None
}