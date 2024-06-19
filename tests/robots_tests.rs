#[cfg(test)]
mod tests {
    

    use eerea::{map::Map, robot::{Behavior, Module, Robot}, tile::{Resource, Tile, TileContent}};

    #[test]
    fn test_robot_exploration() {
        let  station_position = (0, 0);
        let mut map = Map::new(10, 10, 1);
        let mut robot = Robot::new(1, (5, 5), 100, Module::Imaging, Behavior::Exploration);
        robot.perform_action(&mut map, station_position);
        assert!(map.tile_at(robot.position.0, robot.position.1).unwrap().explored);
    }

    #[test]
    fn test_robot_collecting_resources() {
        let mut map = Map::new(10, 10, 1);
        map.tiles[5][5] = Tile::new(false, TileContent::Resource(Resource::Energy));
        let mut robot = Robot::new(2, (5, 5), 100, Module::Mining, Behavior::ResourceCollection);
        let station_position = (0, 0);
        robot.perform_action(&mut map, station_position);
        assert_eq!(map.tile_at(robot.position.0, robot.position.1).unwrap().content, TileContent::Empty);
    }

    #[test]
    fn test_robot_investigating() {
        let mut map = Map::new(10, 10, 1);
        let mut robot = Robot::new(3, (5, 5), 100, Module::Analysis, Behavior::ScientificInterest);
        let station_position = (0, 0);
        robot.perform_action(&mut map, station_position);
    }

    #[test]
    fn test_robot_energy_management() {
        let mut map = Map::new(10, 10, 1);
        let mut robot = Robot::new(4, (5, 5), 1, Module::Imaging, Behavior::Exploration);
        let station_position = (0, 0);
        robot.perform_action(&mut map, station_position);
        assert_eq!(robot.energy, 0);
        robot.perform_action(&mut map, station_position);
        robot.refill_energy();
        assert_eq!(robot.energy, 100);
    }

    

    
}