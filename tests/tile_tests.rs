use eerea::tile::{Tile, TileContent, Resource};

#[test]
fn test_tile_initialization() {
    let tile = Tile::new(false, TileContent::Empty);
    assert_eq!(tile.explored, false);
    assert_eq!(tile.content, TileContent::Empty);
}

#[test]
fn test_tile_with_resource() {
    let tile = Tile::new(false, TileContent::Resource(Resource::Energy));
    if let TileContent::Resource(resource) = tile.content {
        assert_eq!(resource, Resource::Energy);
    } else {
        panic!("Expected Resource::Energy");
    }
}

#[test]
fn test_tile_exploration() {
    let mut tile = Tile::new(false, TileContent::Empty);
    tile.explored = true;
    assert!(tile.explored);
}