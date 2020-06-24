use super::tile_map::Tile::*;
use super::TileMap;
use std::fmt;

/*
When TileMap is created         // Some tiles block others do not
When a tile glyph is changed    // tiles aren't removed.  They become Empty.

When a character moves  // All characters block.  What about small rats?
When a character enters
When a character exits

                        // items do not block
When an item is placed  // items don't move.  They are removed and re-placed
When an item is removed

                            // All facilities block
When a facility is placed   // facilities don't move.  They are removed and re-placed
When a facility is removed

*/
#[derive(Clone)]
pub struct BlockingMap {
    pub width: usize,
    pub height: usize,
    pub map: Vec<bool>,
}

impl fmt::Debug for BlockingMap {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // self.map[..].fmt(formatter)
        let mut index: usize = 0;
        for _y in 0..self.height {
            for _x in 0..self.width {
                let tile_char = match self.map[index] {
                    false => ".",
                    true => "+",
                };
                index += 1;
                formatter.write_str(tile_char).unwrap();
            }
            formatter.write_str("\n").unwrap();
        }
        formatter.write_str("=")
    }
}

impl BlockingMap {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            map: vec![],
        }
    }

    pub fn refresh(&mut self, tiles: &TileMap) {
        self.map = vec![false; tiles.len()];
        self.width = tiles.map_width;
        self.height = tiles.map_height;

        for (index, tile) in tiles.to_iter().enumerate() {
            match tile {
                StoneWall | Empty | ClosedDoor | DeepWater | Coastline => self.map[index] = true,
                _ => {}
            }
        }
    }

    #[inline]
    pub fn is_blocked_at(&self, x: i32, y: i32) -> bool {
        self.map[y as usize * self.width + x as usize]
    }

    #[inline]
    pub fn block_at(&mut self, x: i32, y: i32) {
        self.map[y as usize * self.width + x as usize] = true
    }

    #[inline]
    pub fn unblock_at(&mut self, x: i32, y: i32) {
        self.map[y as usize * self.width + x as usize] = false
    }
}
