use eerea::map::Map;
use eerea::robot::{Behavior, Module, Robot};

#[test]
fn test_robot_creation() {
    let robot = Robot::new(1, (0, 0), 100, Module::Analysis, Behavior::Exploration);
    assert_eq!(robot.id, 1);
    assert_eq!(robot.position, (0, 0));
    assert_eq!(robot.energy, 100);
}

#[test]
fn test_robot_exploration() {
    let mut map = Map::new(10, 10, 1);
    let mut robot = Robot::new(1, (1, 1), 100, Module::Analysis, Behavior::Exploration);
    robot.perform_action(&mut map, (0, 0));
    assert!(robot.energy < 100);
    assert!(map.tile_at(robot.position.0, robot.position.1).unwrap().explored);
}


#[test]
fn test_robot_move_towards_goal() {
    let map = Map::new(10, 10, 1);
    let mut robot = Robot::new(1, (1, 1), 100, Module::Mining, Behavior::ResourceCollection);
    let goal = (3, 3);
    robot.move_towards_goal(&map, goal);
    assert_ne!(robot.position, (1, 1));
}

#[test]
fn test_robot_mark_tile_as_known() {
    let mut robot = Robot::new(1, (0, 0), 100, Module::Analysis, Behavior::Exploration);
    robot.mark_tile_as_known(2, 2);
    assert!(robot.known_tiles.contains(&(2, 2)));
}