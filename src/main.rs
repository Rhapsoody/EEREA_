use eerea::map::Map;
use eerea::robot::{Behavior, Module, Robot};
use eerea::tile::{TileContent, Resource};

use ggez::event::{self};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;


struct MapMainState {
    map: Map,
    robots: Vec<Robot>,
    obstacle_image: Image,
    ore_image: Image,
    energy_image: Image,
    place_of_interest_image: Image,
    empty_image: Image,
    robot_image: Image,
    station_position: (usize, usize),
}

impl MapMainState {
    fn new(ctx: &mut Context) -> GameResult<MapMainState> {
        let map = Map::new(25, 25, 10);
        let obstacle_image = Image::new(ctx, "/obstacle.png")?;
        let ore_image = Image::new(ctx, "/ore.png")?;
        let energy_image = Image::new(ctx, "/energy.png")?;
        let place_of_interest_image = Image::new(ctx, "/scientific_place.png")?;
        let empty_image = Image::new(ctx, "/empty.png")?;
        let robot_image = Image::new(ctx, "/robot.png")?;

        let robot1 = Robot::new(1, (5, 5), 100, Module::Imaging, Behavior::Exploration);

        let station_position = (0, 0);
        
        let state = MapMainState { map, obstacle_image, 
            ore_image, 
            energy_image, 
            place_of_interest_image, 
            empty_image, robot_image, 
            robots: vec![robot1], 
            station_position};

        
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MapMainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for robot in &mut self.robots {
            robot.perform_action(&mut self.map);
            println!("Robot {} moved to position {:?}", robot.id, robot.position);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Dessiner la carte
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

        
        for robot in &self.robots {
            let draw_params = DrawParam::default()
                .dest([robot.position.0 as f32 * tile_size, robot.position.1 as f32 * tile_size]);
            graphics::draw(ctx, &self.robot_image, draw_params)?;
        }

        graphics::present(ctx)
    }
}

fn main() -> GameResult<()> {
    // Charger les ressources (optionnel)
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
        .window_mode(ggez::conf::WindowMode::default().dimensions(900.0, 900.0))
        .build()?;

    let game = MapMainState::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}