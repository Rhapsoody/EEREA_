use eerea::map::Map;
use eerea::robot::{Behavior, Module, Robot};
use eerea::station::Station;

#[test]
fn test_station_initialization() {
    let station = Station::new((5, 5));
    assert_eq!(station.position, (5, 5));
    assert_eq!(station.energy, 0);
    assert!(station.robots.is_empty());
    assert!(station.known_tiles.is_empty());
}

#[test]
fn test_station_collect_data() {
    let mut map = Map::new(10, 10, 1);
    let mut robot = Robot::new(1, (1, 1), 100, Module::Analysis, Behavior::Exploration);
    robot.mark_explored(&mut map);
    let mut station = Station::new((5, 5));
    station.collect_data(&robot, &map);
    assert!(station.known_tiles.iter().any(|t| t.x == 1 && t.y == 1));
}

#[test]
fn test_station_create_robot() {
    let mut station = Station::new((5, 5));
    let robot = station.create_robot(1, (5, 5), Module::Mining, Behavior::ResourceCollection);
    assert_eq!(robot.id, 1);
    assert_eq!(robot.position, (5, 5));
    assert_eq!(robot.energy, 100);
    assert_eq!(station.robots.len(), 1);
}

#[test]
fn test_station_add_energy() {
    let mut station = Station::new((5, 5));
    station.add_energy(50);
    assert_eq!(station.energy, 50);
}