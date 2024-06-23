use eerea::map::Map;
use eerea::tile::{TileContent, Resource};

#[test]
fn test_generate_tiles() {
    let map = Map::new(10, 10, 1);
    assert_eq!(map.width, 10);
    assert_eq!(map.height, 10);
}

#[test]
fn test_check_bounds() {
    let map = Map::new(10, 10, 1);
    assert!(map.check_bounds(0, 0));
    assert!(map.check_bounds(9, 9));
    assert!(!map.check_bounds(10, 10));
}

#[test]
fn test_is_empty() {
    let map = Map::new(10, 10, 1);
    assert!(!map.is_empty(0, 0)); 
}

#[test]
fn test_throw_resource_at() {
    let mut map = Map::new(10, 10, 1);
    map.throw_resource_at(1, 1, Resource::Energy);
    if let Some(tile) = map.tile_at(1, 1) {
        assert_eq!(tile.content, TileContent::Resource(Resource::Energy));
    } else {
        panic!("Tile not found");
    }
}

#[test]
fn test_tile_at() {
    let map = Map::new(10, 10, 1);
    if let Some(tile) = map.tile_at(2, 2) {
        assert_eq!(tile.content, TileContent::Empty); 
    } else {
        panic!("Tile not found");
    }
}

#[test]
fn test_tile_at_mut() {
    let mut map = Map::new(10, 10, 1);
    if let Some(tile) = map.tile_at_mut(1, 1) {
        tile.content = TileContent::Obstacle;
    }
    if let Some(tile) = map.tile_at(1, 1) {
        assert_eq!(tile.content, TileContent::Obstacle);
    } else {
        panic!("Tile not found");
    }
}

#[test]
fn test_retrieve_resource_at() {
    let mut map = Map::new(10, 10, 1);
    map.throw_resource_at(1, 1, Resource::Energy);
    let resource = map.retrieve_resource_at(1, 1);
    assert_eq!(resource, Some(Resource::Energy));
    if let Some(tile) = map.tile_at(1, 1) {
        assert_eq!(tile.content, TileContent::Empty);
    } else {
        panic!("Tile not found");
    }
}