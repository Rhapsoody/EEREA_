// Importer les modules du crate
use eerea::tile::{Tile, TileContent, Resource};
use eerea::map::Map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_content() {
        let obstacle_tile = Tile::new(false, TileContent::Obstacle);
        let resource_tile = Tile::new(false, TileContent::Resource(Resource::Energy));
        let empty_tile = Tile::new(false, TileContent::Empty);

        assert!(matches!(obstacle_tile.content, TileContent::Obstacle));
        assert!(matches!(resource_tile.content, TileContent::Resource(Resource::Energy)));
        assert!(matches!(empty_tile.content, TileContent::Empty));
    }

    #[test]
    fn test_map_generation() {
        let map = Map::new(10, 10, 42);

        // Vérifier les bordures
        for x in 0..10 {
            assert!(matches!(map.tiles[0][x].content, TileContent::Obstacle));
            assert!(matches!(map.tiles[9][x].content, TileContent::Obstacle));
        }

        for y in 0..10 {
            assert!(matches!(map.tiles[y][0].content, TileContent::Obstacle));
            assert!(matches!(map.tiles[y][9].content, TileContent::Obstacle));
        }

        // Vérifier les tuiles internes
        for y in 1..9 {
            for x in 1..9 {
                let content = &map.tiles[y][x].content;
                match content {
                    TileContent::Obstacle => {},
                    TileContent::Resource(resource) => match resource {
                        Resource::Energy | Resource::Ore | Resource::PlaceOfInterest => {},
                    },
                    TileContent::Empty => {},
                }
            }
        }
    }
}