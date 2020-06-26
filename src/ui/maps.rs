use super::*;
use std::collections::HashSet;
use ui::Sprite;

#[derive(Debug)]
pub struct BackgroundMap {
    pub width: usize,
    pub height: usize,
    pub map: Vec<SpriteStyle>,
}

impl BackgroundMap {
    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            map: vec![],
        }
    }

    pub fn at(&self, x: i32, y: i32) -> SpriteStyle {
        // check for bounds
        self.map[y as usize * self.width + x as usize]
    }

    pub fn set_at(&mut self, x: i32, y: i32, style: SpriteStyle) {
        // check for bounds
        self.map[y as usize * self.width + x as usize] = style;
    }
}

pub struct SparseMap {
    pub sprites: HashMap<u64, Sprite>,
    positions: HashMap<(i32, i32), OneMany<u64>>,
}

#[derive(Debug, Clone)]
pub enum OneMany<T> {
    One(T),
    Many(HashSet<u64>),
}

impl SparseMap {
    pub fn new() -> SparseMap {
        SparseMap {
            sprites: HashMap::new(),   // indexed on id
            positions: HashMap::new(), // indexed on position
        }
    }

    pub fn lookup_sprite_for_type(character_type: CharacterType) -> Sprite {
        match character_type {
            CharacterType::Rat => ui::Sprite {
                style: SpriteStyle {
                    glyph: 'R' as u8,
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            CharacterType::Spider => ui::Sprite {
                style: SpriteStyle {
                    glyph: 'S' as u8,
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            CharacterType::Player => ui::Sprite {
                style: SpriteStyle {
                    glyph: '@' as u8,
                    fg: RGB::named(rltk::YELLOW),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
        }
    }

    fn lookup_sprite_for_item_class(class: ItemClass) -> ui::Sprite {
        match class {
            ItemClass::Dagger => ui::Sprite {
                style: SpriteStyle {
                    glyph: 25 as u8, // '↓'
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            ItemClass::Headwear => ui::Sprite {
                style: SpriteStyle {
                    glyph: 94 as u8, // ^
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            ItemClass::BladeWeapon => ui::Sprite {
                style: SpriteStyle {
                    glyph: 33 as u8, // !
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            ItemClass::Potion => ui::Sprite {
                style: SpriteStyle {
                    glyph: 173 as u8, // ¡
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            _ => ui::Sprite {
                style: SpriteStyle {
                    glyph: '?' as u8,
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
        }
    }

    fn lookup_sprite_for_facility_class(class: FacilityClass, variant: u8) -> ui::Sprite {
        match class {
            FacilityClass::ClosedChest => ui::Sprite {
                style: SpriteStyle {
                    glyph: 0xF0 as u8, // '≡'
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::AppleTree => ui::Sprite {
                style: SpriteStyle {
                    glyph: 0x1E as u8, // '▲'
                    fg: RGB::named(rltk::PINK),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::OliveTree => ui::Sprite {
                style: SpriteStyle {
                    glyph: 0x1E as u8, // '▲'
                    fg: RGB::named(rltk::OLIVE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::PineTree => ui::Sprite {
                style: SpriteStyle {
                    glyph: 0x1E as u8, // '▲'
                    fg: RGB::named(rltk::DARK_GREEN),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::OakTree => ui::Sprite {
                style: SpriteStyle {
                    glyph: 0x1E as u8, // '▲'
                    fg: RGB::named(rltk::TAN),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::FruitPress => ui::Sprite {
                style: SpriteStyle {
                    glyph: '#' as u8, // '#'
                    fg: RGB::named(rltk::BLUE),
                    bg: RGB::named(rltk::YELLOW),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::Lumbermill => ui::Sprite {
                style: SpriteStyle {
                    glyph: '*' as u8, // '*'
                    fg: RGB::named(rltk::BLUE),
                    bg: RGB::named(rltk::LIGHTBLUE),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            FacilityClass::Well => {
                [
                    ui::Sprite {
                        style: SpriteStyle {
                            glyph: 31 as u8, // ▼
                            fg: RGB::named(rltk::BLACK),
                            bg: RGB::named(rltk::SADDLE_BROWN),
                        },
                        x: 0,
                        y: 0,
                        facing: Direction::Up,
                    },
                    ui::Sprite {
                        style: SpriteStyle {
                            glyph: 31 as u8, // ▼
                            fg: RGB::named(rltk::BLACK),
                            bg: RGB::named(rltk::BLUE),
                        },
                        x: 0,
                        y: 0,
                        facing: Direction::Up,
                    },
                    ui::Sprite {
                        style: SpriteStyle {
                            glyph: 31 as u8, // ▼
                            fg: RGB::named(rltk::BLACK),
                            bg: RGB::named(rltk::LIGHTGRAY),
                        },
                        x: 0,
                        y: 0,
                        facing: Direction::Up,
                    },
                ][variant as usize]
            }
            FacilityClass::Vein => [
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: '#' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::named(rltk::SADDLEBROWN),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: '#' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::named(rltk::WHEAT),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: '#' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::named(rltk::BLACK),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: '%' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::named(rltk::SILVER),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: '%' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::from_hex("#B87333").ok().unwrap(),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
            ][variant as usize],
            FacilityClass::FishingSpot => [
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: ' ' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::named(rltk::LIGHTBLUE),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
                ui::Sprite {
                    style: SpriteStyle {
                        glyph: '#' as u8, // #
                        fg: RGB::named(rltk::WHITE),
                        bg: RGB::named(rltk::BLUE),
                    },
                    x: 0,
                    y: 0,
                    facing: Direction::Up,
                },
            ][variant as usize],
            FacilityClass::Smeltery => ui::Sprite {
                style: SpriteStyle {
                    glyph: '+' as u8, // '*'
                    fg: RGB::named(rltk::BLUE),
                    bg: RGB::named(rltk::ORANGE),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
            _ => ui::Sprite {
                style: SpriteStyle {
                    glyph: '?' as u8,
                    fg: RGB::named(rltk::WHITE),
                    bg: RGB::named(rltk::BLACK),
                },
                x: 0,
                y: 0,
                facing: Direction::Up,
            },
        }
    }

    pub fn add_character(&mut self, id: u64, character_type: CharacterType, x: i32, y: i32) {
        let mut sprite = Self::lookup_sprite_for_type(character_type);

        sprite.x = x;
        sprite.y = y;

        self.sprites.insert(id, sprite);
        self.insert_at(x, y, id)
    }

    pub fn add_item(
        &mut self,
        id: u64,
        _description: &String,
        item_class: ItemClass,
        x: i32,
        y: i32,
    ) {
        let mut sprite = Self::lookup_sprite_for_item_class(item_class);
        sprite.x = x;
        sprite.y = y;

        self.sprites.insert(id, sprite);
        self.insert_at(x, y, id)
    }

    pub fn add_facility(
        &mut self,
        id: u64,
        x: i32,
        y: i32,
        class: FacilityClass,
        _description: String,
        variant: u8,
    ) {
        let mut sprite = Self::lookup_sprite_for_facility_class(class, variant);
        sprite.x = x;
        sprite.y = y;

        self.sprites.insert(id, sprite);
        self.insert_at(x, y, id)
    }
    pub fn lookup(&self, id: u64) -> Option<&Sprite> {
        self.sprites.get(&id)
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&OneMany<u64>> {
        self.positions.get(&(x, y))
    }

    pub fn insert_at(&mut self, x: i32, y: i32, id: u64) {
        let entry = self.positions.entry((x, y)).or_insert(OneMany::One(id));
        let new_entry = match entry {
            OneMany::One(id_found) if *id_found == id => OneMany::One(id), // do nothing
            OneMany::One(existing) => {
                let mut set = HashSet::new();
                set.insert(*existing);
                OneMany::Many(set)
            }
            OneMany::Many(set) => {
                set.insert(id);
                OneMany::Many(set.to_owned())
            }
        };
        *entry = new_entry;
    }

    pub fn remove(&mut self, id: u64) {
        println!("removing: {}", id);
        let sprite = self.sprites.remove(&id).expect("sprite not found");

        let entry = self
            .positions
            .get(&(sprite.x, sprite.y))
            .expect("nothing found at position");
        let new_entry = match entry.clone() {
            OneMany::One(id) if id == id => None, // leave nothing
            OneMany::One(_) => panic!("mismatched positions found"),
            OneMany::Many(mut set) => {
                set.remove(&id);
                Some(OneMany::Many(set.to_owned()))
            }
        };

        match new_entry {
            None => {
                self.positions.remove(&(sprite.x, sprite.y));
            }
            Some(entry) => {
                self.positions.insert((sprite.x, sprite.y), entry);
            }
        }
    }

    pub fn reposition(&mut self, id: u64, x: i32, y: i32) {
        let origin = self.sprites.get(&id).unwrap();
        let one_many = self.positions.get_mut(&(origin.x, origin.y));

        match one_many {
            Some(OneMany::One(found)) if found.to_owned() == id => {
                self.positions.remove(&(origin.x, origin.y));
            }
            Some(OneMany::One(_)) => panic!("id not found at position"),
            Some(OneMany::Many(set)) => {
                set.remove(&id);
            }
            None => {}
        }
        self.insert_at(x, y, id);

        let sprite = self.sprites.get_mut(&id).unwrap();
        sprite.x = x;
        sprite.y = y;
    }

    pub fn change_facing(&mut self, id: u64, facing: Direction) {
        self.sprites
            .get_mut(&id)
            .expect("character not found")
            .facing = facing
    }
}
