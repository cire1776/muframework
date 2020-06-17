// #[macro_use]
// extern crate lazy_static;
// use mut_static::MutStatic;

extern crate chrono;

use bracket_lib::prelude::*;
use std::collections::HashMap;
pub mod common;

pub mod game;
use game::character::CharacterType;
use game::tile_map::TileMap;

use game::facility::FacilityClass;
use game::items::{Item, ItemClass};
pub mod ui;
use ui::window::{BasicWindow, InventoryWindow, MapWindow, MouseReceiver, Window};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Copy, Clone)]
pub enum MapLayer {
    Base,
    Item,
    Facility,
    Character,
    Player,
}

#[derive(Debug)]
pub enum Command {
    None,
    QuitGame,
    Move(Direction, MoveCommandMode),
    Teleport(u64, i32, i32),
    SpawnItem(u64, ItemClass, String),
    TakeItem(u64),
    DropItem(u64),
    EquipItem(u64),
    UnequipItem(u64),
    TransferItem(u64, u64, u64), // (item_id, src_inventory_id, dest_inventory_id)
    TransferAllItems(u64, u64),  // (src_inventory_id, dest_inventory_id )
    CloseExternalInventory,
    RefreshInventory,
    ActivityAbort,
    ActivityComplete,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoveCommandMode {
    Normal,
    Sneak,
    Use,
}

#[derive(Debug, Clone)]
pub enum GameUpdate {
    Message(String),
    SetBackground(TileMap),
    TileChangedAt(i32, i32, game::tile_map::Tile),
    CharacterEntered {
        id: u64,
        x: i32,
        y: i32,
        character_type: CharacterType,
    },
    CharacterMoved(u64, i32, i32),      // Normal move to a new position
    CharacterTeleported(u64, i32, i32), // Instantaneous move to a new position
    CharacterFacingChanged(u64, Direction),
    ItemAdded {
        id: u64,
        x: i32,
        y: i32,
        description: String,
        class: ItemClass,
    },
    ItemRemoved(u64),
    FacilityAdded {
        id: u64,
        x: i32,
        y: i32,
        description: String,
        class: FacilityClass,
    },
    FacilityUpdated {
        id: u64,
        description: String,
        class: FacilityClass,
    },
    EquipmentUpdated(Vec<Item>),
    InventoryUpdated(Vec<Item>),
    ExternalInventoryOpened(Vec<Item>, u64),
    ExternalInventoryUpdated(Vec<Item>),
    ExternalInventoryClosed,
    ActivityStarted(u32),
    ActivityExpired(),
    ActivityAborted(),
    Exit,
}
impl GameUpdate {
    /// All GameUpdates are to be sent through this method.
    pub fn send(update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>, update: GameUpdate) {
        if let None = update_tx {
            return;
        }
        update_tx
            .unwrap()
            .send(update)
            .expect("unable to send update")
    }
}
use GameUpdate::*;
