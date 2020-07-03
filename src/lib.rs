#[macro_use]
extern crate lazy_static;
// use mut_static::MutStatic;
// extern crate extern_rand;

extern crate chrono;

use bracket_lib::prelude::*;
use std::collections::HashMap;
pub mod common;
pub use common::timer::{Guard, Timer};

pub mod game;

use game::character::CharacterType;
use game::tile_map::TileMap;

use game::facility::FacilityClass;
use game::{
    equipment::MountingPoint,
    items::{Item, ItemClass},
};
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ActionContinuation {
    Smeltery,
    Firepit,
    Smithy,
    CraftingStation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    None,
    QuitGame,
    DumpPlayer,
    Move(Direction, MoveCommandMode),
    ChangeFacing(Direction),
    Teleport(u64, i32, i32),
    SpawnItem(u64, ItemClass, String), // (inventory_id, ItemClass, Description)
    TakeItem(u64),                     // (item_index[not id])
    DropItem(u64),
    EquipItem(u64),
    UnequipItem(u64),
    TransferItem(u64, u64, u64), // (item_id, src_inventory_id, dest_inventory_id)
    TransferAllItems(u64, u64),  // (src_inventory_id, dest_inventory_id )
    UseItem,
    TransferEquipmentToInventory(MountingPoint, u64), // (mounting point, dest_inventory_id)
    CloseExternalInventory,
    RefreshInventory,
    ActivityAbort,
    ActivityComplete,
    DestroyFacility(u64), // (facility_id)
    ChoiceSelected(u8, ActionContinuation, u64),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoveCommandMode {
    Normal,
    Sneak,
    Use,
    SneakUse,
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
        variant: u8,
    },
    FacilityUpdated {
        id: u64,
        description: String,
        class: FacilityClass,
        variant: u8,
    },
    FacilityRemoved {
        id: u64,
    },
    EquipmentUpdated(Vec<Item>),
    InventoryUpdated(Vec<Item>),
    ExternalInventoryOpened(Vec<Item>, u64), // (Contents, inventory_id?)
    ExternalInventoryUpdated(Vec<Item>),
    ExternalInventoryClosed,
    ActivityStarted(u32, ui::pane::PaneTitle), // (millis, title)
    ActivityExpired(),
    ActivityAborted(),
    DisplayOptions(Vec<String>, ActionContinuation, u64),
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

#[cfg(test)]
pub mod test_support {
    use super::*;
    use game::inventory::*;
    use game::items::{Item, ItemList, ItemType};
    use game::NEXT_ITEM_ID;
    use std::convert::TryInto;

    pub fn test_item<S: ToString, N: TryInto<u8>>(
        class: ItemClass,
        description: S,
        quantity: N,
    ) -> Item {
        Item::new(
            NEXT_ITEM_ID(),
            ItemType::new(class, description.to_string()),
            quantity,
        )
    }

    pub fn spawn_item_into_inventory<S: ToString, N: TryInto<u8>>(
        class: ItemClass,
        description: S,
        quantity: N,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) -> Item {
        let description = description.to_string();
        {
            let item_types = &mut items.item_types;
            item_types.lookup_or_insert(&description, class, &description);
        }

        inventory.spawn_by_type(&description, quantity, &items.item_types.clone(), items)
    }
}
