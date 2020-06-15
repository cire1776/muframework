use super::*;
use std::convert::*;
use timer::Guard;

use character::CharacterList;
// use geometry::Point;
pub mod blocking_map;
use blocking_map::BlockingMap;

pub mod command;
pub use command::{CommandSender, GameUpdateSender};

pub mod character;
pub use character::{Character, CharacterType, Player};

pub mod level;
pub use level::Level;

pub mod items;
pub use items::{Item, ItemClass, ItemList, ItemState};

pub mod inventory;
pub use inventory::{AliasList, Inventory, InventoryList};

pub mod facility;
pub use facility::{Facility, FacilityList};

pub mod equipment;
pub use equipment::{ItemClassSpecifier, ItemClassSpecifierList, MountingPoint, MountingPointMap};

pub mod tile_map;
pub use tile_map::TileMap;

pub mod parsing;
pub use parsing::*;

use std::sync::atomic::{AtomicU64, Ordering};

pub struct GameData {
    pub player: Player,
    pub map: TileMap,
    pub obstacles: BlockingMap,
    pub characters: CharacterList,

    pub item_class_specifiers: ItemClassSpecifierList,
    pub items: ItemList,
    pub facilities: FacilityList,

    pub inventories: InventoryList,
}

// starts at two to reserve one for the player.
//  this is temporary
static GLOBAL_NEXT_ID: AtomicU64 = AtomicU64::new(2);
static GLOBAL_NEXT_ITEM_ID: AtomicU64 = AtomicU64::new(1);

#[allow(non_snake_case)]
fn NEXT_ID() -> u64 {
    GLOBAL_NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

#[allow(non_snake_case)]
fn NEXT_ITEM_ID() -> u64 {
    GLOBAL_NEXT_ITEM_ID.fetch_add(1, Ordering::SeqCst)
}

pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn game_loop(
        update_tx: GameUpdateSender,
        command_rx: std::sync::mpsc::Receiver<Command>,
        command_tx: CommandSender,
    ) {
        let (
            player,
            map,
            obstacles,
            characters,
            item_class_specifiers,
            items,
            facilities,
            inventories,
        ) = &mut Self::initialize_game("maps/level1.map", Some(&update_tx));

        let game_state = &mut GameState::new();
        loop {
            let command = command_rx.recv();

            if let Ok(command) = command {
                game_state.game_loop_iteration(
                    player,
                    map,
                    obstacles,
                    characters,
                    item_class_specifiers,
                    items,
                    facilities,
                    inventories,
                    &command,
                    Some(&update_tx),
                    Some(&command_tx),
                );
            } else {
                // if receiver is broken, we just bail, ending the game.
                //   eventually, we need to save the game, probably whenever
                //   leaving this loop.
                return;
            }
        }
    }

    /// public for testing purposes
    pub fn initialize_game<S: ToString>(
        level_path: S,
        update_tx: Option<&GameUpdateSender>,
    ) -> (
        Player,
        TileMap,
        BlockingMap,
        CharacterList,
        ItemClassSpecifierList,
        ItemList,
        FacilityList,
        InventoryList,
    ) {
        let (mut map, mut character_vec, item_vec, facility_vec, stored_item_vec) =
            TileMap::load_from_file(level_path.to_string());

        let mut obstacles = BlockingMap::new();
        obstacles.refresh(&map);

        let (player_x, player_y) =
            common::geometry::Point::read_coordinates(character_vec[0].to_owned());
        let characters = Character::read_in_characters(&mut character_vec);
        let mut items = Item::read_in_items(&item_vec);

        // find home for activity guard and activity timer

        let mut player = Player::new();

        player.x = player_x;
        player.y = player_y;

        let inventories = &mut InventoryList::new();

        let (facilities, aliases) = Facility::read_in_facilities(&facility_vec, inventories);

        // create the player's inventory
        Inventory::new_into_inventory_list(player.id, inventories);

        Item::read_in_stored_items(&stored_item_vec, aliases, &mut items, inventories);

        let item_class_specifiers = ItemClassSpecifier::initialize();

        // consider adding a function to level to do these things
        Level::introduce_player(&player, inventories, update_tx);
        Level::introduce_other_characters(&characters, &mut obstacles, update_tx);
        Level::introduce_items(&items, update_tx);
        Level::introduce_facilities(&facilities, &mut map, &mut obstacles, update_tx);

        // consider moving this to a function
        GameUpdate::send(update_tx, SetBackground(map.clone()));

        (
            player,
            map,
            obstacles,
            characters,
            item_class_specifiers,
            items,
            facilities,
            inventories.clone(),
        )
    }

    /// pub for testing purposes only
    pub fn game_loop_iteration(
        &mut self,
        player: &mut Player,
        map: &mut TileMap,
        obstacles: &mut BlockingMap,
        _characters: &mut CharacterList,
        item_class_specifiers: &mut ItemClassSpecifierList,
        items: &mut ItemList,
        facilities: &mut FacilityList,
        inventories: &mut InventoryList,
        command: &Command,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        self.abort_activity_if_necessary(player, command, update_tx);

        match command {
            Command::QuitGame => {
                GameUpdate::send(update_tx, Exit);
                return;
            }
            Command::Move(direction, mode) => Command::move_player(
                *direction,
                *mode,
                player,
                map,
                obstacles,
                facilities,
                inventories,
                update_tx,
                command_tx,
            ),
            Command::Teleport(id, _new_x, _new_y) => {
                if *id != 1 {
                    todo!()
                }
                todo!()
                // Command::teleport_character(*id, *new_x, *new_y, self, &update_tx);
            }
            Command::SpawnItem(inventory_id, class, description) => {
                Command::spawn_item(
                    *inventory_id,
                    *class,
                    description,
                    inventories,
                    items,
                    update_tx,
                    command_tx,
                );
            }
            Command::TakeItem(item_index) => Command::pickup_item(
                *item_index,
                player,
                items,
                inventories,
                update_tx,
                command_tx,
            ),
            Command::DropItem(item_index) => Command::drop_item(
                *item_index,
                player,
                items,
                inventories,
                update_tx,
                command_tx,
            ),
            Command::EquipItem(item_id) => Command::equip_item(
                *item_id,
                player,
                item_class_specifiers,
                items,
                inventories,
                update_tx,
                command_tx,
            ),
            Command::UnequipItem(item_id) => {
                Command::unequip_item(*item_id, player, items, inventories, update_tx, command_tx)
            }
            Command::TransferItem(item_id, src_inventory, dest_inventory) => {
                Command::transfer_item(
                    *item_id,
                    *src_inventory,
                    *dest_inventory,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                );
            }
            Command::TransferAllItems(src_inventory, dest_inventory) => {
                Command::transfer_all_items(
                    *src_inventory,
                    *dest_inventory,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                )
            }
            Command::CloseExternalInventory => Command::close_external_inventory(update_tx),
            Command::RefreshInventory => Self::refresh_inventory(player, inventories, update_tx),
            Command::AbortActivity | Command::None => {}
        }
    }

    fn abort_activity_if_necessary(
        &mut self,
        player: &mut Player,
        command: &Command,
        update_tx: Option<&GameUpdateSender>,
    ) {
        if let None = player.activity_guard {
            return;
        }

        match command {
            // list commands that do not abort activities here
            Command::None
            | Command::SpawnItem(_, _, _)
            | Command::RefreshInventory
            | Command::TakeItem(_)
            | Command::DropItem(_) => {}

            _ => {
                if let Some(_) = player.activity_guard {
                    player.activity_guard = None;
                }

                GameUpdate::send(update_tx, GameUpdate::ActivityAborted());
            }
        };
    }

    pub fn refresh_inventory(
        player: &Player,
        inventories: &InventoryList,
        update_tx: Option<&GameUpdateSender>,
    ) {
        GameUpdate::send(
            update_tx,
            GameUpdate::InventoryUpdated(
                inventories
                    .get(&player.inventory_id())
                    .expect("unable to find player inventory")
                    .to_vec(),
            ),
        );
    }

    // for testing purposes
    pub fn teleport_player<U: TryInto<i32>>(
        &mut self,
        new_x: U,
        new_y: U,
        player: &mut Player,
        obstacles: &mut BlockingMap,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        use crate::game::command::CommandHandler;

        let character = player;
        let facing = character.facing;

        let obstacles = obstacles;
        let mut command = game::command::MoveCommand::new(
            character,
            facing,
            new_x
                .try_into()
                .ok()
                .expect("Must be able to convert to i32"),
            new_y
                .try_into()
                .ok()
                .expect("Must be able to convert to i32"),
            obstacles,
        );
        command.execute(update_tx, command_tx);
    }
}
#[cfg(test)]
mod test_external_inventories;
