use super::*;
use extern_timer::Guard;
use std::convert::*;

use character::CharacterList;
// use geometry::Point;
pub mod blocking_map;
use blocking_map::BlockingMap;

pub mod command;
pub use command::{Activity, CommandHandler, CommandSender, GameUpdateSender};

pub mod attributes;
pub use attributes::*;

pub mod character;
pub use character::{Character, CharacterType, Player};

pub mod level;
pub use level::Level;

pub mod items;
pub use items::{Item, ItemClass, ItemList, ItemState, ItemType, ItemTypeList};

pub mod inventory;
pub use inventory::{AliasList, Inventory, InventoryList};

pub mod skills;
pub use skills::*;

pub mod facility;
pub use facility::{Facility, FacilityList};

pub mod equipment;
pub use equipment::{ItemClassSpecifier, ItemClassSpecifierList, MountingPoint, MountingPointMap};

pub mod tile_map;
pub use tile_map::TileMap;

pub mod parsing;
pub use parsing::*;

use std::sync::atomic::{AtomicU64, Ordering};

// starts at two to reserve one for the player.
//  this is temporary
static GLOBAL_NEXT_ID: AtomicU64 = AtomicU64::new(2);
static GLOBAL_NEXT_ITEM_ID: AtomicU64 = AtomicU64::new(1);

#[allow(non_snake_case)]
pub fn NEXT_ID() -> u64 {
    GLOBAL_NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

#[allow(non_snake_case)]
pub fn NEXT_ITEM_ID() -> u64 {
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
        let mut activity: Option<Box<dyn Activity>> = None;

        loop {
            let command = command_rx.recv();

            if let Ok(command) = command {
                activity = game_state.game_loop_iteration(
                    player,
                    map,
                    obstacles,
                    characters,
                    item_class_specifiers,
                    items,
                    facilities,
                    inventories,
                    activity,
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
        let (
            mut map,
            mut character_vec,
            mut item_type_vec,
            item_vec,
            facility_vec,
            stored_item_vec,
        ) = TileMap::load_from_file(level_path.to_string());

        let mut obstacles = BlockingMap::new();
        obstacles.refresh(&map);

        let (player_x, player_y) =
            common::geometry::Point::read_coordinates(character_vec[0].to_owned());
        let characters = Character::read_in_characters(&mut character_vec);

        let item_types = ItemType::read_in_item_types(&mut item_type_vec);

        let mut items = Item::read_in_items(&item_vec, item_types.clone());

        // find home for activity guard and activity timer

        let mut player = Player::new();

        player.x = player_x;
        player.y = player_y;

        let inventories = &mut InventoryList::new();

        let (mut facilities, aliases) = Facility::read_in_facilities(&facility_vec, inventories);

        // create the player's inventory
        Inventory::new_into_inventory_list(player.id, inventories);

        Item::read_in_stored_items(&stored_item_vec, aliases, &mut items, inventories);

        let item_class_specifiers = ItemClassSpecifier::initialize();

        let players_inventory = inventories.get_mut(&1).unwrap();
        players_inventory.spawn_by_type("apple", 63, &item_types.clone(), &mut items);
        players_inventory.spawn_by_type("olive", 64, &item_types.clone(), &mut items);
        players_inventory.spawn_by_type("apple", 16, &item_types.clone(), &mut items);
        players_inventory.spawn_by_type("glass_bottle", 64, &item_types.clone(), &mut items);

        // consider adding a function to level to do these things
        Level::introduce_player(&player, inventories, update_tx);
        Level::introduce_other_characters(&characters, &mut obstacles, update_tx);
        Level::introduce_items(&items, update_tx);
        Level::introduce_facilities(&mut facilities, &mut map, &mut obstacles, update_tx);

        // TODO: consider moving this to a function
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
        activity: Option<Box<dyn Activity>>,
        command: &Command,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        let mut activity = activity;
        activity = self.abort_activity_if_necessary(activity, command, update_tx);

        match command {
            Command::QuitGame => {
                GameUpdate::send(update_tx, Exit);
                return None;
            }
            Command::DumpPlayer => {
                println!("{:?}", player);
                None
            }
            Command::Move(direction, mode) => Command::move_player(
                *direction,
                *mode,
                player,
                map,
                obstacles,
                facilities,
                &items.item_types.clone(),
                items,
                inventories,
                activity,
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
                activity
            }
            Command::TakeItem(item_index) => {
                Command::pickup_item(
                    *item_index,
                    player,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                );
                None
            }
            Command::DropItem(item_index) => {
                Command::drop_item(
                    *item_index,
                    player,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                );
                None
            }
            Command::EquipItem(item_id) => {
                Command::equip_item(
                    *item_id,
                    player,
                    item_class_specifiers,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                );
                None
            }
            Command::UnequipItem(item_id) => {
                Command::unequip_item(*item_id, player, items, inventories, update_tx, command_tx);
                None
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
                None
            }
            Command::TransferAllItems(src_inventory, dest_inventory) => {
                Command::transfer_all_items(
                    *src_inventory,
                    *dest_inventory,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                );
                None
            }

            Command::TransferEquipmentToInventory(mounting_point, inventory_id) => {
                Command::transfer_equipment_to_inventory(
                    mounting_point,
                    *inventory_id,
                    player,
                    items,
                    inventories,
                    update_tx,
                    command_tx,
                );
                None
            }
            Command::CloseExternalInventory => {
                Command::close_external_inventory(update_tx);
                None
            }
            Command::RefreshInventory => {
                Self::refresh_inventory(player, inventories, update_tx);
                activity
            }
            Command::DestroyFacility(facility_id) => {
                let facility = facilities
                    .get(*facility_id)
                    .expect("cannot locate facility.");
                map.set_tile_at(facility.x, facility.y, facility.background_tile);
                GameUpdate::send(
                    update_tx,
                    GameUpdate::TileChangedAt(facility.x, facility.y, facility.background_tile),
                );
                facilities.remove(*facility_id);
                GameUpdate::send(update_tx, GameUpdate::FacilityRemoved { id: *facility_id });
                activity
            }
            Command::None => activity,
            Command::ActivityComplete => self.complete_activity(
                player,
                activity,
                facilities,
                items,
                inventories,
                update_tx,
                command_tx,
            ),
            Command::ActivityAbort => None,
            Command::ChoiceSelected(selection, continuation, facility_id) => match continuation {
                ActionContinuation::Smeltery => {
                    command::facility_commands::smeltery_commands::ActivateSmelteryCommand::new(
                        player,
                        *facility_id,
                        *selection,
                        inventories,
                    )
                    .execute(update_tx, command_tx)
                }
                _ => panic!("unknown continuation"),
            },
        }
    }

    fn complete_activity(
        &self,
        player: &mut Player,
        activity: Option<Box<dyn Activity>>,
        facilities: &mut FacilityList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        let mut activity = activity;
        if let Some(ref mut activity) = &mut activity {
            activity.complete(
                player,
                facilities,
                items,
                inventories,
                &update_tx.expect("update_tx is None."),
                &command_tx.expect("command_tx is None"),
            );
        }

        activity
    }

    fn abort_activity_if_necessary(
        &mut self,
        activity: Option<Box<dyn Activity>>,
        command: &Command,
        update_tx: Option<&GameUpdateSender>,
    ) -> Option<Box<dyn Activity>> {
        let mut activity = activity;
        let mut clear_activity = false;
        if let Some(ref mut activity) = activity {
            match command {
                // list commands that do not abort activities here
                Command::None
                | Command::SpawnItem(_, _, _)
                | Command::RefreshInventory
                | Command::TakeItem(_)
                | Command::DropItem(_)
                | Command::ActivityComplete => {}

                _ => {
                    activity.clear_guard();
                    clear_activity = true;
                    GameUpdate::send(update_tx, GameUpdate::ActivityAborted());
                }
            };
        }
        if clear_activity {
            None
        } else {
            activity
        }
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
        _inventories: &InventoryList,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
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

#[cfg(test)]
mod test_item_type;

#[cfg(test)]
mod test_facility_properties;

#[cfg(test)]
mod tests;
