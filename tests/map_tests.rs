use eerea::map::Map;


#[test]
fn test_good_map_dimensions() {
    let map = Map::new(10, 20, 12345);
    assert_eq!(map.width, 10);
    assert_eq!(map.height, 20);
}

#[test]
fn test_good_map_tiles() {
    let map = Map::new(10, 20, 12345);
    assert_eq!(map.tiles.len(), 20);
    assert_eq!(map.tiles[0].len(), 10);
}

#[test]
fn test_fn_check_bounds() {
    let map = Map::new(10, 20, 12345);
    assert_eq!(map.check_bounds(0, 0), true);
    assert_eq!(map.check_bounds(10, 20), false);
    assert_eq!(map.check_bounds(10, 0), false);
    assert_eq!(map.check_bounds(0, 20), false);
}
