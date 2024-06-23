use eerea::map::Map;
use eerea::robot::{Behavior, Module, Robot};
use eerea::station::{KnownTile, Station};
use eerea::tile::{TileContent, Resource};

use ggez::event::{self};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

struct MapMainState {
    map: Arc<Mutex<Map>>,
    station: Arc<Mutex<Station>>,
    obstacle_image: Image,
    ore_image: Image,
    energy_image: Image,
    place_of_interest_image: Image,
    empty_image: Image,
    robot_image: Image,
    station_image: Image,
    robot_tx: Sender<(usize, (usize, usize))>,
    robot_rx: Receiver<(usize, (usize, usize))>,
}

impl MapMainState {
    fn new(ctx: &mut Context) -> GameResult<MapMainState> {
        let map = Arc::new(Mutex::new(Map::new(40, 40, 14)));
        
        let map_locked = map.lock().unwrap();
        let station_position = find_free_tile(&map_locked).unwrap_or((0, 0));
        drop(map_locked);

        let station = Arc::new(Mutex::new(Station::new(station_position)));

        let (robot_tx, robot_rx) = mpsc::channel();

        let obstacle_image = Image::new(ctx, "/obstacle.png")?;
        let ore_image = Image::new(ctx, "/ore.png")?;
        let energy_image = Image::new(ctx, "/energy.png")?;
        let place_of_interest_image = Image::new(ctx, "/scientific_place.png")?;
        let empty_image = Image::new(ctx, "/empty.png")?;
        let robot_image = Image::new(ctx, "/robot.png")?;
        let station_image = Image::new(ctx, "/station.png")?;

        let station_pos;
        {
            let station_locked = station.lock().unwrap();
            station_pos = station_locked.position;
        }

        {
            let mut station_locked = station.lock().unwrap();
            
            let robot1 = station_locked.create_robot(1, station_pos, Module::Analysis, Behavior::Exploration, robot_tx.clone());
            let robot2 = station_locked.create_robot(2, station_pos, Module::Mining, Behavior::ResourceCollection, robot_tx.clone());
            let robot3 = station_locked.create_robot(3, station_pos, Module::Imaging, Behavior::ScientificInterest, robot_tx.clone());

            station_locked.robots.push(robot1);
            station_locked.robots.push(robot2);
            station_locked.robots.push(robot3);
        }

        let state = MapMainState { 
            map, 
            station,
            obstacle_image, 
            ore_image, 
            energy_image, 
            place_of_interest_image, 
            empty_image, 
            robot_image, 
            station_image,
            robot_tx,
            robot_rx,
        };

        Ok(state)
    }

    fn update_robots(&mut self) {
        let station = Arc::clone(&self.station);
        let map = Arc::clone(&self.map);
        let tx = self.robot_tx.clone();
    
        let station_position;
        let robots: Vec<Robot>;
    
        {
            let station_locked = station.lock().unwrap();
            station_position = station_locked.position;
            robots = station_locked.robots.clone();
        }
    
        let mut robots_to_refill = vec![];
    
        for robot in robots {
            let robot_clone = robot.clone();
            let map = Arc::clone(&map);
            let tx = tx.clone();
    
            if robot.position == station_position {
                robots_to_refill.push(robot.id);
            }
    
            thread::spawn(move || {
                let mut robot = robot_clone;
                robot.perform_action(&mut map.lock().unwrap(), station_position);
                tx.send((robot.id, robot.position)).unwrap();
            });
        }
    
        self.collect_and_refill_robots(robots_to_refill);
    }

    fn collect_and_refill_robots(&mut self, robots_to_refill: Vec<usize>) {
        let map = Arc::clone(&self.map);
        let station = Arc::clone(&self.station);
    
        let map_locked = map.lock().unwrap();
        let  station_locked = station.lock().unwrap();
    
        let mut updates: Vec<(usize, Vec<(usize, usize, u64)>)> = vec![];
        let mut refills: Vec<usize> = vec![];
    
        for robot_id in robots_to_refill {
            if let Some(robot) = station_locked.robots.iter().find(|r| r.id == robot_id) {
                let mut known_tiles_updates = vec![];
                for &(x, y) in &robot.known_tiles {
                    if let Some(tile) = map_locked.tile_at(x, y) {
                        if tile.explored {
                            known_tiles_updates.push((x, y, tile.timestamp));
                        }
                    }
                }
                updates.push((robot_id, known_tiles_updates));
                refills.push(robot_id);
            }
        }
    
        drop(map_locked);
        drop(station_locked);
    
        let mut station_locked = station.lock().unwrap();
        for (_robot_id, known_tiles_updates) in updates {
            for (x, y, timestamp) in known_tiles_updates {
                let known_tile = station_locked.known_tiles.iter_mut().find(|t| t.x == x && t.y == y);
                match known_tile {
                    Some(existing_tile) => {
                        if timestamp > existing_tile.timestamp {
                            existing_tile.timestamp = timestamp;
                        }
                    }
                    None => {
                        station_locked.known_tiles.push(KnownTile { x, y, timestamp });
                    }
                }
            }
        }
    
        for robot_id in refills {
            if let Some(robot) = station_locked.robots.iter_mut().find(|r| r.id == robot_id) {
                robot.refill_energy();
            }
        }
    }

    fn create_robot_if_needed(&mut self) {
        let station = Arc::clone(&self.station);
        let tx = self.robot_tx.clone();
    
        let mut station_locked = station.lock().unwrap();
    
        let robot_id = station_locked.robots.len() + 1;
        let station_position = station_locked.position;
    
        if station_locked.energy >= 100 {
            let new_robot = station_locked.create_robot(robot_id, station_position, Module::Imaging, Behavior::Exploration, tx);
            station_locked.energy -= 100;
            station_locked.robots.push(new_robot);
        }
    }
}

impl event::EventHandler<ggez::GameError> for MapMainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        while let Ok((id, pos)) = self.robot_rx.try_recv() {
            if let Some(robot) = self.station.lock().unwrap().robots.iter_mut().find(|r| r.id == id) {
                robot.position = pos;
            }
        }

        self.update_robots();
        self.create_robot_if_needed();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let tile_size = 32.0;
        let map = self.map.lock().unwrap();
        for (y, row) in map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let image = match tile.content {
                    TileContent::Obstacle => &self.obstacle_image,
                    TileContent::Resource(Resource::Energy) => &self.energy_image,
                    TileContent::Resource(Resource::Ore) => &self.ore_image,
                    TileContent::Resource(Resource::PlaceOfInterest) => &self.place_of_interest_image,
                    TileContent::Empty => &self.empty_image,
                };
                let draw_params = DrawParam::default().dest([x as f32 * tile_size, y as f32 * tile_size]);
                graphics::draw(ctx, image, draw_params)?;
            }
        }

        let station_draw_params = DrawParam::default().dest([self.station.lock().unwrap().position.0 as f32 * tile_size, self.station.lock().unwrap().position.1 as f32 * tile_size]);
        graphics::draw(ctx, &self.station_image, station_draw_params)?;

        for robot in &self.station.lock().unwrap().robots {
            let draw_params = DrawParam::default().dest([robot.position.0 as f32 * tile_size, robot.position.1 as f32 * tile_size]);
            graphics::draw(ctx, &self.robot_image, draw_params)?;

            let robot_info = format!("Energy: {}, Module: {:?},", robot.energy, robot.module);
            let text = graphics::Text::new((robot_info, graphics::Font::default(), 20.0));
            let position = [robot.position.0 as f32 * tile_size, robot.position.1 as f32 * tile_size + 32.0];
            graphics::draw(ctx, &text, (position, 0.0, graphics::Color::WHITE))?;
        }

        graphics::present(ctx)
    }
}

fn find_free_tile(map: &Map) -> Option<(usize, usize)> {
    for y in 0..map.height {
        for x in 0..map.width {
            if let TileContent::Empty = map.tiles[y][x].content {
                return Some((x, y));
            }
        }
    }
    None
}

fn main() -> GameResult<()> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("map_game", "Author")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("EEREA Game :)"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1400.0, 1400.0))
        .build()?;

    let game = MapMainState::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}