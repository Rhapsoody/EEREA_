use eerea::map::Map;
use eerea::robot::{Behavior, Module, Robot};
use eerea::station::{KnownTile, Station};
use eerea::tile::{TileContent, Resource};

use ggez::event::{self};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;


struct MapMainState {
    map: Map,
    station: Station,
    obstacle_image: Image,
    ore_image: Image,
    energy_image: Image,
    place_of_interest_image: Image,
    empty_image: Image,
    robot_image: Image,
    station_image: Image,
}

impl MapMainState {
    fn new(ctx: &mut Context) -> GameResult<MapMainState> {
        let map = Map::new(40, 40, 14);
        let obstacle_image = Image::new(ctx, "/obstacle.png")?;
        let ore_image = Image::new(ctx, "/ore.png")?;
        let energy_image = Image::new(ctx, "/energy.png")?;
        let place_of_interest_image = Image::new(ctx, "/scientific_place.png")?;
        let empty_image = Image::new(ctx, "/empty.png")?;
        let robot_image = Image::new(ctx, "/robot.png")?;
        let station_image = Image::new(ctx, "/station.png")?;

        let tile_with_nothing = find_free_tile(&map).expect("Bruh no free tile bro");

        let mut station = Station::new(tile_with_nothing);

        // Ajouter 3 robots avec des rôles différents
        let robot1 = Robot::new(1, station.position, 100, Module::Analysis, Behavior::Exploration);
        let robot2 = Robot::new(2, station.position, 100, Module::Mining, Behavior::ResourceCollection);
        let robot3 = Robot::new(3, station.position, 100, Module::Mining, Behavior::ResourceCollection);

        station.robots.push(robot1);
        station.robots.push(robot2);
        station.robots.push(robot3);
        
        let state = MapMainState { 
            map, 
            station ,
            obstacle_image, 
            ore_image, 
            energy_image, 
            place_of_interest_image, 
            empty_image, 
            robot_image, 
            station_image,
        };

        Ok(state)
    }

    

    fn update_robots(&mut self) {
        let mut robots_to_refill = vec![];

        for robot in &mut self.station.robots {
            robot.perform_action(&mut self.map, self.station.position);
            if robot.position == self.station.position {
                robots_to_refill.push(robot.id);
            }
        }

        self.collect_and_refill_robots(robots_to_refill);
    }

    fn collect_and_refill_robots(&mut self, robots_to_refill: Vec<usize>) {
        let map = &self.map;

        for robot_id in robots_to_refill {
            if let Some(robot) = self.station.robots.iter_mut().find(|r| r.id == robot_id) {
                for &(x, y) in &robot.known_tiles {
                    if let Some(tile) = map.tile_at(x, y) {
                        if tile.explored {
                            let known_tile = self.station.known_tiles.iter_mut().find(|t| t.x == x && t.y == y);
                            match known_tile {
                                Some(existing_tile) => {
                                    if tile.timestamp > existing_tile.timestamp {
                                        existing_tile.timestamp = tile.timestamp;
                                    }
                                }
                                None => {
                                    self.station.known_tiles.push(KnownTile { x, y, timestamp: tile.timestamp });
                                }
                            }
                        }
                    }
                }
                robot.refill_energy();
            }
        }
    }

    fn create_robot_if_needed(&mut self) {
        if self.station.energy >= 100 {
            let new_robot = self.station.create_robot(self.station.robots.len() + 1, self.station.position, Module::Imaging, Behavior::Exploration);
            self.station.energy -= 100;
            self.station.robots.push(new_robot);
        }
    }
}

impl event::EventHandler<ggez::GameError> for MapMainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_robots();
        self.create_robot_if_needed();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // carte 
        let tile_size = 32.0;
        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let image = match tile.content {
                    TileContent::Obstacle => &self.obstacle_image,
                    TileContent::Resource(Resource::Energy) => &self.energy_image,
                    TileContent::Resource(Resource::Ore) => &self.ore_image,
                    TileContent::Resource(Resource::PlaceOfInterest) => &self.place_of_interest_image,
                    TileContent::Empty => &self.empty_image,
                };
                let draw_params = DrawParam::default()
                    .dest([x as f32 * tile_size, y as f32 * tile_size]);
                graphics::draw(ctx, image, draw_params)?;
            }
        }

        // station
        let station_draw_params = DrawParam::default().dest([self.station.position.0 as f32 * tile_size, self.station.position.1 as f32 * tile_size]);
        graphics::draw(ctx, &self.station_image, station_draw_params)?;

        //robots
        for robot in &self.station.robots {
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