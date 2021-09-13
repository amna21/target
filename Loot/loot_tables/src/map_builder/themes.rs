use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

//START: dungeon
impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            // START_HIGHLIGHT
            TileType::Exit => to_cp437('>'),
            // END_HIGHLIGHT
        }
    }
}
//END: dungeon

pub struct ForestTheme {}

//START: forest
impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            // START_HIGHLIGHT
            TileType::Exit => to_cp437('>'),
            // END_HIGHLIGHT
        }
    }
}
//END: forest

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}
