use super::*;
use std::sync::mpsc;

pub mod maps;

pub mod input;
use input::{InputState, MouseState};

use game::items::Item;

use game::tile_map::{Tile, TileMap};
use maps::{BackgroundMap, SparseMap};
pub mod window;
use window::MapWindowMode;

pub mod pane;
pub use pane::*;

#[derive(Debug, Copy, Clone)]
pub struct SpriteStyle {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub style: SpriteStyle,
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
}

pub struct Map {}

/// The player info for UIState.  Hopefully not to be confused ith game::Player.

#[derive(Debug, Copy, Clone)]
pub struct UIPlayer {
    facing: Direction,
    inventory_id: u64,
    x: i32,
    y: i32,
}

impl UIPlayer {
    pub fn new() -> Self {
        Self {
            facing: Direction::Up,
            inventory_id: 1,
            x: 0,
            y: 0,
        }
    }

    pub fn locate(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub struct UIState {
    pub update_rx: std::sync::mpsc::Receiver<GameUpdate>,
    pub command_tx: std::sync::mpsc::Sender<Command>,

    pub map_window: MapWindow,
    pub message_window: Window,
    pub inventory_window: InventoryWindow,
    pub info_window: Window,

    pub characters: SparseMap,
    pub items: SparseMap,
    pub facilities: SparseMap,

    pub player: UIPlayer,

    pub inventory: Vec<Item>,
    pub equipment: Vec<Item>,
    pub external_inventory: Option<Vec<Item>>,
    pub external_inventory_id: Option<u64>,

    pub background: BackgroundMap,

    pub input_state: InputState,
    pub mouse_state: MouseState,

    pub activity_time: Option<u64>,
}

impl GameState for UIState {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.perform_tick(Some(ctx));
    }
}

impl UIState {
    /// public for testing purposes
    pub fn perform_tick(&mut self, context: Option<&mut BTerm>) {
        let received = self.update_rx.try_recv();

        match received {
            Ok(Exit) => {
                println!("exiting the game");
                std::process::exit(0)
            }
            Ok(SetBackground(tile_map)) => self.set_background(&tile_map),
            Ok(TileChangedAt(x, y, new_tile)) => {
                let new_style = Self::style_from_tile(new_tile);
                self.background.set_at(x, y, new_style);
            }
            Ok(Message(m)) => println!("Message: {}", m),
            Ok(CharacterTeleported(id, new_x, new_y)) => {
                self.player.locate(new_x, new_y);
                self.characters.reposition(id, new_x, new_y);
                if id == 1 {
                    self.focus_on_player(new_x, new_y);
                }
            }
            Ok(CharacterEntered {
                id,
                x,
                y,
                character_type,
            }) => {
                self.player.locate(x, y);
                self.characters.add_character(id, character_type, x, y);
                if id == 1 {
                    self.focus_on_player(x, y);
                }
            }
            Ok(CharacterMoved(id, new_x, new_y)) => {
                self.player.locate(new_x, new_y);
                self.characters.reposition(id, new_x, new_y);
                if id == 1 {
                    self.focus_on_player(new_x, new_y);
                }
            }
            Ok(CharacterFacingChanged(id, facing)) => {
                self.characters.change_facing(id, facing);
                if id == 1 {
                    self.player.facing = facing;
                }
            }
            Ok(ItemAdded {
                id,
                x,
                y,
                description,
                class,
            }) => self.items.add_item(id, &description, class, x, y),
            Ok(ItemRemoved(item_id)) => self.items.remove(item_id),
            Ok(FacilityAdded {
                id,
                x,
                y,
                description,
                class,
            }) => self.facilities.add_facility(id, x, y, class, description),
            Ok(FacilityUpdated {
                id: _,
                description: _,
                class: _,
            }) => {}
            Ok(EquipmentUpdated(items)) => {
                self.inventory_window.max_selection_equipment = items.len() as u8;
                self.inventory_window
                    .set_max_equipment_selection(items.len() as u8);
                self.update_equipment(items);
            }
            Ok(InventoryUpdated(items)) => {
                self.inventory = items;
                self.inventory
                    .sort_by(|a, b| a.description().cmp(&b.description()));

                self.inventory_window
                    .set_max_item_selection(self.inventory.len() as u8);
            }
            Ok(ExternalInventoryOpened(external_inventory, external_inventory_id)) => {
                self.input_state = InputState::ExternalInventoryOpen;
                self.map_window.window_mode = MapWindowMode::ExternalInventory;
                self.map_window.active_pane =
                    Some(Pane::new(10, 5, 25, 28, external_inventory.len() as u8));
                self.external_inventory = Some(external_inventory);
                self.external_inventory_id = Some(external_inventory_id);
            }
            Ok(ExternalInventoryClosed) => {
                self.input_state = InputState::Normal;
                self.map_window.window_mode = MapWindowMode::Normal;
                self.map_window.active_pane = None;
                self.external_inventory = None;
                self.external_inventory_id = None;
            }
            Ok(ExternalInventoryUpdated(external_inventory)) => {
                if self.input_state == InputState::ExternalInventoryOpen {
                    let mut pane = self.map_window.active_pane().unwrap();
                    pane.set_max_selection(external_inventory.len() as u8);
                    self.map_window.active_pane = Some(pane);
                    self.external_inventory = Some(external_inventory);
                }
            }
            Ok(ActivityStarted(duration)) => {
                self.activity_time = Some(time_in_millis() + duration as u64);
                self.map_window.active_pane =
                    Some(Pane::new(self.player.x + 2, self.player.y + 2, 11, 3, 0));
                self.input_state = InputState::Activity;
            }
            Ok(ActivityExpired()) => {
                self.input_state = InputState::Normal;
                self.activity_time = None;
            }
            Ok(ActivityAborted()) => {
                self.input_state = InputState::Normal;
                self.activity_time = None;
            }
            Err(_) => {}
        }

        if let Some(context) = context {
            self.draw_gui(context);
            self.draw_map(context);
            self.process_input(context);
        }
    }

    pub fn new(update_rx: mpsc::Receiver<GameUpdate>, command_tx: mpsc::Sender<Command>) -> Self {
        Self {
            update_rx,
            command_tx,

            map_window: MapWindow::new(21, 0, 41, 35),
            message_window: Window::new(21, 35, 41, 24),
            inventory_window: InventoryWindow::new(62, 0, 21, 59),
            info_window: Window::new(0, 0, 21, 59),

            characters: SparseMap::new(),
            items: SparseMap::new(),
            facilities: SparseMap::new(),
            background: BackgroundMap::empty(),

            player: UIPlayer::new(),

            inventory: vec![],
            equipment: vec![],
            external_inventory: None,
            external_inventory_id: None,

            input_state: InputState::Normal,
            mouse_state: MouseState::LeftButtonUp,

            activity_time: None,
        }
    }

    fn update_equipment(&mut self, items: Vec<Item>) {
        self.equipment = items;
    }

    fn set_background(&mut self, tile_map: &TileMap) {
        self.background.width = tile_map.map_width;
        self.background.height = tile_map.map_height;

        self.map_window
            .set_max_scroll(tile_map.map_width as i32, tile_map.map_height as i32);

        self.background = BackgroundMap {
            width: tile_map.map_width,
            height: tile_map.map_height,
            map: vec![
                SpriteStyle {
                    glyph: ' ' as u8,
                    fg: RGB::named(rltk::BLACK),
                    bg: RGB::named(rltk::BLACK),
                };
                tile_map.map_width * tile_map.map_height
            ],
        };

        for y in 0..self.background.height as i32 {
            for x in 0..self.background.width as i32 {
                let style = Self::style_from_tile(tile_map.at(x, y));
                self.background.set_at(x, y, style);
            }
        }
    }
    fn style_from_tile(tile: Tile) -> SpriteStyle {
        let style = match tile {
            Tile::Empty => SpriteStyle {
                glyph: ' ' as u8,
                fg: RGB::named(rltk::BLACK),
                bg: RGB::named(rltk::BLACK),
            },
            Tile::StoneWall => SpriteStyle {
                glyph: '#' as u8,
                fg: RGB::named(rltk::WHITE),
                bg: RGB::named(rltk::BLACK),
            },
            Tile::DirtFloor => SpriteStyle {
                glyph: '.' as u8,
                fg: RGB::named(rltk::LIGHT_GRAY),
                bg: RGB::named(rltk::BLACK),
            },
            Tile::ClosedDoor => SpriteStyle {
                glyph: '|' as u8,
                fg: RGB::named(rltk::WHITE),
                bg: RGB::named(rltk::BLACK),
            },
            Tile::OpenDoor => SpriteStyle {
                glyph: '/' as u8,
                fg: RGB::named(rltk::WHITE),
                bg: RGB::named(rltk::BLACK),
            },
            Tile::Facility(_) => SpriteStyle {
                glyph: '\u{ff}' as u8,
                fg: RGB::named(rltk::BLACK),
                bg: RGB::named(rltk::WHITE),
            },
        };
        style
    }

    fn focus_on_player(&mut self, x: i32, y: i32) {
        if self.map_window.is_in_central_region(x, y) {
            return;
        }

        self.map_window.move_focus_towards(x, y);
    }

    fn draw_gui(&mut self, context: &mut BTerm) {
        context.cls();
        self.message_window.draw_frame(context, "messages");

        self.inventory_window.draw_frame(context, "inventory");

        let items: Vec<String> = self.inventory.iter().map(|i| i.description()).collect();
        let equipment: Vec<String> = self.equipment.iter().map(|i| i.description()).collect();

        self.inventory_window.set_inventory_items(items, equipment);

        self.info_window.draw_frame(context, "info");

        let mut message = "".to_string();
        if self.characters.sprites.len() > 0 {
            let player = self.characters.sprites[&1];
            message = format!("({},{})", player.x, player.y);
        }
        self.map_window.draw_frame(context, &message[..]);
    }

    fn draw_background(&self, context: &mut BTerm) {
        for y in 0..self.background.height as i32 {
            for x in 0..self.background.width as i32 {
                let style = self.background.at(x, y);
                self.map_window
                    .set(context, x, y, style.fg, style.bg, style.glyph);
            }
        }
    }

    fn draw_items(&self, window: &dyn BasicWindow, context: &mut BTerm) {
        for (_key, sprite) in self.items.sprites.iter() {
            window.set(
                context,
                sprite.x,
                sprite.y,
                sprite.style.fg,
                sprite.style.bg,
                sprite.style.glyph,
            );
        }
    }

    fn draw_characters(&self, window: &dyn BasicWindow, context: &mut BTerm) {
        for (_key, sprite) in self.characters.sprites.iter() {
            window.set(
                context,
                sprite.x,
                sprite.y,
                sprite.style.fg,
                sprite.style.bg,
                sprite.style.glyph,
            )
        }
    }

    fn draw_facilities(&self, window: &dyn BasicWindow, context: &mut BTerm) {
        for (_key, sprite) in self.facilities.sprites.iter() {
            window.set(
                context,
                sprite.x,
                sprite.y,
                sprite.style.fg,
                sprite.style.bg,
                sprite.style.glyph,
            )
        }
    }

    fn draw_map(&mut self, context: &mut BTerm) {
        {
            {
                let window = &self.map_window;

                self.draw_background(context);
                self.draw_items(window, context);
                self.draw_facilities(window, context);
                self.draw_characters(window, context);
                self.draw_facing_indicator(&self.player.facing, window, context);
                self.draw_activity_pane(window, context);
                self.draw_external_inventory_pane(window, context);
            }
            {}
        }

        let window = &self.map_window;
        if window.window_mode == MapWindowMode::ExternalInventory {}
    }

    fn draw_activity_pane(&self, window: &dyn BasicWindow, context: &mut BTerm) {
        if let Some(expiration) = self.activity_time {
            let seconds = (expiration - time_in_millis()) / 1000;
            let pane = self.map_window.active_pane.unwrap();

            pane.draw_frame("ESC: Abort", window, context);
            pane.draw_text(format!("{}", seconds), 5, 1, window, context)
        }
    }

    fn draw_external_inventory_pane(&self, window: &dyn BasicWindow, context: &mut BTerm) {
        if let Some(items) = &self.external_inventory {
            let strings: Vec<String> = items.iter().map(|i| i.description()).collect();

            let mut pane = window.active_pane().unwrap();

            pane.set_max_selection(strings.len() as u8);

            self.perform_draw_external_inventory_window(strings, window, context);
        }
    }

    fn perform_draw_external_inventory_window(
        &self,
        items: Vec<String>,
        window: &dyn BasicWindow,
        context: &mut BTerm,
    ) {
        if let Some(pane) = window.active_pane() {
            pane.draw_frame("ESC: Close   A: Take All", window, context);

            for (i, item) in items.iter().enumerate() {
                if Some((i + 1) as u8) == pane.selection {
                    pane.draw_inverted_text(item, 2, i as i32 + 1, window, context)
                } else {
                    pane.draw_text(item, 2, i as i32 + 1, window, context)
                }
            }
        } else {
            panic!("{:?}", window.active_pane());
        }
    }

    fn draw_facing_indicator(
        &self,
        facing: &Direction,
        window: &dyn BasicWindow,
        context: &mut BTerm,
    ) {
        context.set(
            window.width() / 2 - 1 + window.x(),
            window.height() - 2 + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::UpLeft {
                '\\' as u8
            } else {
                ' ' as u8
            },
        );
        context.set(
            window.width() / 2 + window.x(),
            window.height() - 2 + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::Up {
                '|' as u8
            } else {
                ' ' as u8
            },
        );
        context.set(
            window.width() / 2 + 1 + window.x(),
            window.height() - 2 + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::UpRight {
                '/' as u8
            } else {
                ' ' as u8
            },
        );

        context.set(
            window.width() / 2 - 1 + window.x(),
            window.height() - 1 + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::Left {
                '-' as u8
            } else {
                ' ' as u8
            },
        );
        context.set(
            window.width() / 2 + window.x(),
            window.height() - 1 + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            15 as u8,
        );
        context.set(
            window.width() / 2 + 1 + window.x(),
            window.height() - 1 + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::Right {
                '-' as u8
            } else {
                ' ' as u8
            },
        );

        context.set(
            window.width() / 2 - 1 + window.x(),
            window.height() + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::DownLeft {
                '/' as u8
            } else {
                ' ' as u8
            },
        );
        context.set(
            window.width() / 2 + window.x(),
            window.height() + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::Down {
                '|' as u8
            } else {
                ' ' as u8
            },
        );
        context.set(
            window.width() / 2 + 1 + window.x(),
            window.height() + window.y(),
            RGB::named(BLUE),
            RGB::named(WHITE),
            if *facing == Direction::DownRight {
                '\\' as u8
            } else {
                ' ' as u8
            },
        );
    }
}

pub fn time_in_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("No time")
        .as_millis() as u64
}
