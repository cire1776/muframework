use super::*;

use regex::Regex;
use std::fmt;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    StoneWall,
    DirtFloor,
    ClosedDoor,
    OpenDoor,
    Facility(u64),
}

impl Tile {
    pub fn to_str(tile: Tile) -> &'static str {
        match tile {
            Tile::Empty => " ",
            Tile::StoneWall => "#",
            Tile::DirtFloor => ".",
            Tile::ClosedDoor => "|",
            Tile::OpenDoor => "/",
            Tile::Facility(_) => "\u{ff}",
        }
    }
}

#[derive(Clone)]
pub struct TileMap {
    pub map_width: usize,
    pub map_height: usize,
    map: Vec<Tile>,
}

impl fmt::Debug for TileMap {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut index: usize = 0;
        for _y in 0..self.map_height {
            for _x in 0..self.map_width {
                let tile_char = Tile::to_str(self.map[index]);
                index += 1;
                formatter.write_str(tile_char).unwrap();
            }
            formatter.write_str("\n").unwrap();
        }
        formatter.write_str("=")
    }
}

impl TileMap {
    /// Initializes a new zero-sized TileMap.
    ///
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::TileMap;
    /// let subject = TileMap::new();
    /// assert_eq!(subject.map_width,0);
    /// assert_eq!(subject.map_height,0);
    /// ```
    pub fn new() -> Self {
        Self {
            map_width: 0,
            map_height: 0,
            map: vec![],
        }
    }

    /// loads a level from a file.
    /// # Arguments:
    /// * `filename` - A string slice that
    pub fn load_from_file<S: ToString> (
        filename: S,
    ) -> (TileMap, Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
        let contents = fs::read_to_string(filename.to_string()).expect("unable to read level file");

        let re = Regex::new(
            r"(?s)(.+)===END OF MAP===\n(.+)===END OF CHARACTERS===\n(.*)===END OF ITEMS===\n(.*)===END OF FACILITIES===\n(.*)===END OF STORED ITEMS===",
        )
        .expect("unable to initialize regex");

        let captures = re.captures(&contents).expect("unable to match contents");
        let capture = captures.get(1).unwrap();
        let map_rows = capture.as_str().lines().collect::<Vec<&str>>();
        let map_height: usize = Self::get_map_row_count(&map_rows);
        let map_width: usize = Self::get_map_column_count(&map_rows);

        let characters = capture_section(&captures, 2);
        let items = capture_section(&captures, 3);
        let facilities: Vec<String> = capture_section(&captures, 4);
        let stored_items = capture_section(&captures, 5);

        (
            Self::load_map_from_vector(&map_rows, map_width, map_height),
            characters,
            items,
            facilities,
            stored_items,
        )
    }

    /// returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// returns an iterator over the map.
    pub fn to_iter(&self) -> std::slice::Iter<'_, Tile> {
        self.map.iter()
    }

    /// adds a tile to an empty space in the map.
    /// # Arguments:
    /// * tile - the new tile to be placed.
    /// * x, y - the coordinates at which the tile should be placed.
    /// # Note:
    /// Panics if the tile at x,y is not already empty.
    pub fn add_tile(&mut self, tile: Tile, x: i32, y: i32) {
        let index = self.xy_to_index(x, y);

        if self.map[index] != Tile::Empty {
            panic!("tile added over non-empty tile");
        }
        self.map[index] = tile;
    }

    /// sets the tile at x,y to empty
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// assert_eq!(subject.at(5,5), Tile::StoneWall);
    /// subject.remove_tile(5,5);
    /// assert_eq!(subject.at(5,5), Tile::Empty);
    /// ```
    pub fn remove_tile(&mut self, x: i32, y: i32) {
        let index = self.xy_to_index(x, y);
        self.map[index] = TileMap::default_tile();
    }

    /// sets the tile at x,y
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// subject.set_tile_at(5,5, Tile::OpenDoor);
    /// assert_eq!(subject.at(5,5), Tile::OpenDoor);
    /// ```
    /// ```should_panic
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// subject.set_tile_at(500,500, Tile::OpenDoor);
    /// ```
    pub fn set_tile_at(&mut self, x: i32, y: i32, tile: Tile) {
        if self.out_of_bounds(x, y) {
            panic!("out of bounds");
        }
        let index = self.xy_to_index(x, y);
        self.map[index] = tile;
    }

    /// converts x,y to a direct single-dimension index into the map.
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// let result = subject.xy_to_index(5,5);
    /// assert_eq!(result, 55);
    /// ```
    #[inline]
    pub fn xy_to_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.map_width + x as usize
    }

    #[inline]
    fn get_map_row_count(rows: &Vec<&str>) -> usize {
        rows.len()
    }

    #[inline]
    fn get_map_column_count(rows: &Vec<&str>) -> usize {
        rows.iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len()
    }

    fn load_map_from_vector(rows: &Vec<&str>, width: usize, height: usize) -> TileMap {
        let mut map = TileMap {
            map_width: width as usize,
            map_height: height as usize,
            map: vec![Tile::Empty; width * height],
        };

        let mut index: usize = 0;

        for row in rows {
            for character in row.chars() {
                let tile = match character {
                    '#' => Tile::StoneWall,
                    '.' => Tile::DirtFloor,
                    ' ' => Tile::Empty,
                    '|' => Tile::ClosedDoor,
                    '/' => Tile::OpenDoor,
                    char => panic!("unrecognized character: {}", char),
                };
                map.map[index] = tile;
                index += 1;
            }
            if index % width != 0 {
                index = index + width - index % width;
            }
        }
        map
    }

    /// returns the empty tile
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// assert_eq!(TileMap::default_tile(), Tile::Empty);
    /// ```
    #[inline]
    pub fn default_tile() -> Tile {
        Tile::Empty
    }

    /// return true if x,y is outside of the bounds of the map.  False otherwise.
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// assert!(subject.out_of_bounds(100,5));
    /// assert!(!subject.out_of_bounds(9,9));
    /// ```
    #[inline]
    pub fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= self.map_width as i32 || y < 0 || y >= self.map_height as i32
    }

    /// returns the tile at x,y.  Returns the default tile if out of bounds.
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// assert_eq!(subject.at(9,5), Tile::StoneWall);
    /// assert_eq!(subject.at(100,100), Tile::Empty);
    /// ```
    pub fn at(&self, x: i32, y: i32) -> Tile {
        if self.out_of_bounds(x, y) {
            Self::default_tile()
        } else {
            self.map[y as usize * self.map_width + x as usize]
        }
    }

    /// Expands the dimensions of the map.  Will not reduce the size.
    /// Currently used only for testing
    /// # Arguments:
    /// * width, height - the new dimensions
    /// * default - the tile that will be used for new spaces.
    ///
    /// # Examples:
    /// ```
    /// # use muframework::game::tile_map::{Tile,TileMap};
    /// let mut subject = TileMap::new();
    /// subject.set_dimensions(5,5, Tile::DirtFloor);
    /// subject.set_dimensions(10,10, Tile::StoneWall);
    /// assert_eq!(subject.at(9,9), Tile::StoneWall);
    /// assert_eq!(subject.at(4,4), Tile::DirtFloor);
    /// ```
    pub fn set_dimensions(&mut self, width: i32, height: i32, default: Tile) {
        let old_width = self.map_width;
        let old_height = self.map_height;
        let old_map = &self.map.clone();

        self.map_width = width as usize;
        self.map_height = height as usize;
        self.map = vec![default; (width * height) as usize];

        let old_index = 0;
        for y in 0..old_height {
            for x in 0..old_width {
                self.set_tile_at(x as i32, y as i32, old_map[old_index]);
            }
        }
    }

}
