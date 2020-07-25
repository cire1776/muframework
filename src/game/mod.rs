use super::*;
use std::convert::*;

use character::CharacterList;
// use geometry::Point;
pub mod blocking_map;
use blocking_map::BlockingMap;

pub mod random;
pub use random::*;

pub mod command;
pub use command::{Activity, CommandHandler};

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

pub mod saving;
pub use saving::{GameLoader, GameSaver};

pub mod levelling;
pub use levelling::*;

use chrono::Local;

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

pub struct GameData {
    pub auto_save_enabled: bool,
    pub current_tick: u128,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            auto_save_enabled: true,
            current_tick: 0,
        }
    }
}

pub struct GameState {
    ticks: u128,
}

impl GameState {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }

    pub fn game_loop(
        auto_save_enabled: bool,
        update_tx: GameUpdateSender,
        command_rx: std::sync::mpsc::Receiver<Command>,
        command_tx: CommandSender,
    ) {
        #[allow(unused_assignments)]
        let (
            mut player,
            mut map,
            mut obstacles,
            mut characters,
            mut item_class_specifiers,
            mut items,
            mut facilities,
            mut inventories,
            mut timer,
        ) = Self::initialize_game(
            "maps/level1.map",
            Some(&update_tx),
            Some(command_tx.clone()),
        );

        let game_state = &mut GameState::new();

        if let Some(filename) = GameLoader::find_latest_save_file() {
            let (
                new_player,
                new_obstacles,
                new_characters,
                new_items,
                new_facilities,
                new_inventories,
            ) = GameLoader::load_game(filename, &mut map, game_state, &mut timer, Some(&update_tx));

            player = new_player;
            obstacles = new_obstacles;
            characters = new_characters;
            items = new_items;
            facilities = new_facilities;
            inventories = new_inventories;
        }

        let _guard = game_state.start_heartbeat(&mut timer);
        let mut activity: Option<Box<dyn Activity>> = None;

        GameUpdate::send(
            Some(&update_tx),
            GameUpdate::Message {
                message: "Welcome to the World of Mufra!".to_string(),
                message_type: MessageType::System,
                timestamp: Local::now().format("%T").to_string(),
            },
        );

        let mut alarms_set = false;

        let mut game_data = GameData::new();
        game_data.auto_save_enabled = auto_save_enabled;

        loop {
            if timer.current_tick() != 0 && !alarms_set {
                Self::setup_alarms(&mut timer, auto_save_enabled);
                alarms_set = true;
            }

            game_data.current_tick = game_state.ticks;

            let command = command_rx.recv();
            let mut rng = random::Rng::new();

            if command == Ok(Command::LoadGame) {
                let (
                    new_player,
                    new_obstacles,
                    new_characters,
                    new_items,
                    new_facilities,
                    new_inventories,
                ) = GameLoader::load_game(
                    "level1.sav",
                    &mut map,
                    game_state,
                    &mut timer,
                    Some(&update_tx),
                );

                player = new_player;
                obstacles = new_obstacles;
                characters = new_characters;
                items = new_items;
                facilities = new_facilities;
                inventories = new_inventories;
            } else if let Ok(command) = command {
                activity = game_state.game_loop_iteration(
                    &mut player,
                    &mut map,
                    &mut obstacles,
                    &mut characters,
                    &mut item_class_specifiers,
                    &mut items,
                    &mut facilities,
                    &mut inventories,
                    &mut game_data,
                    &mut rng,
                    &mut timer,
                    activity,
                    &command,
                    Some(&update_tx),
                    Some(command_tx.clone()),
                );
            } else {
                // if receiver is broken, we just bail, ending the game.
                //   eventually, we need to save the game, probably whenever
                //   leaving this loop.
                return;
            }
        }
    }

    fn start_heartbeat(&self, timer: &mut Timer) -> Guard {
        let duration = chrono::Duration::microseconds(1_000_000 / 60);
        timer.repeating(duration, Command::NextTick, "Heartbeat")
    }

    pub fn current_tick(&self) -> u128 {
        self.ticks
    }

    #[cfg(test)]
    pub fn test_start_heartbeat(&self, timer: &mut Timer) -> Guard {
        self.start_heartbeat(timer)
    }

    fn setup_alarms(timer: &mut Timer, auto_save_enabled: bool) {
        use chrono::Timelike;

        timer.repeating_by_tick(3600, Command::DisplayTick, "DisplayTick");

        let minute = chrono::Local::now().minute();
        let mut offset: u128 = (10 - minute % 10) as u128;

        if offset > 5 {
            offset -= 5;
        } else {
            offset += 5;
        };

        println!("offset: {}", offset);

        if auto_save_enabled {
            timer.repeating_by_tick_starting_at(
                offset * 3600 as u128,
                10 * 3600,
                Command::SaveGame,
                "Autosave",
            );
        }
    }

    /// public for testing purposes
    pub fn initialize_game<S: ToString>(
        level_path: S,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<CommandSender>,
    ) -> (
        Player,
        TileMap,
        BlockingMap,
        CharacterList,
        ItemClassSpecifierList,
        ItemList,
        FacilityList,
        InventoryList,
        Timer,
    ) {
        let (
            mut map,
            mut character_vec,
            mut item_type_vec,
            item_vec,
            facility_vec,
            stored_item_vec,
        ) = TileMap::load_from_file(level_path.to_string());

        let mut timer = Timer::new(command_tx.clone());

        let mut obstacles = BlockingMap::new();
        obstacles.refresh(&map);

        let (player_x, player_y) =
            common::geometry::Point::read_coordinates(character_vec[0].to_owned());
        let characters = Character::read_in_characters(&mut character_vec);

        let item_types = ItemType::read_in_item_types(&mut item_type_vec);

        let mut items = Item::read_in_items(&item_vec, item_types.clone());

        let mut player = Player::new();

        player.x = player_x;
        player.y = player_y;

        let inventories = &mut InventoryList::new();

        let (mut facilities, aliases) =
            Facility::read_in_facilities(&facility_vec, inventories, &mut timer);

        // create the player's inventory
        Inventory::new_into_inventory_list(player.id, inventories);

        Item::read_in_stored_items(&stored_item_vec, aliases, &mut items, inventories);

        let item_class_specifiers = ItemClassSpecifier::initialize();

        Level::introduce(
            &player,
            &mut map,
            &mut obstacles,
            &characters,
            &mut facilities,
            &items,
            &inventories,
            update_tx,
        );

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
            timer,
        )
    }

    /// pub for testing purposes only
    pub fn game_loop_iteration(
        &mut self,
        player: &mut Player,
        map: &mut TileMap,
        obstacles: &mut BlockingMap,
        characters: &mut CharacterList,
        item_class_specifiers: &mut ItemClassSpecifierList,
        items: &mut ItemList,
        facilities: &mut FacilityList,
        inventories: &mut InventoryList,
        game_data: &mut GameData,
        rng: &mut Rng,
        timer: &mut Timer,
        activity: Option<Box<dyn Activity>>,
        command: &Command,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        let mut activity = activity;
        activity = self.abort_activity_if_necessary(activity, command, update_tx);

        match command {
            Command::LoadGame => activity,
            Command::SaveGame => {
                let save_data = GameSaver::save_game_to_string(
                    player,
                    characters,
                    items,
                    inventories,
                    facilities,
                    self,
                );
                GameSaver::save_to_file(save_data);
                activity
            }
            Command::NextTick => {
                self.ticks += 1;
                timer.tick(self.ticks);
                activity
            }
            Command::DisplayTick => {
                println!("{}", self.ticks);
                activity
            }
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
                game_data,
                timer,
                activity,
                update_tx,
                command_tx,
            ),
            Command::ChangeFacing(direction) => {
                Command::change_facing(player, *direction, update_tx);
                None
            }
            Command::Teleport(id, _new_x, _new_y) => {
                if *id != 1 {
                    todo!()
                }
                todo!()
                // Command::teleport_character(*id, *new_x, *new_y, self, &update_tx);
            }
            Command::SpawnItem(inventory_id, class, description) => {
                Command::spawn_items(
                    *inventory_id,
                    1,
                    *class,
                    description,
                    inventories,
                    items,
                    update_tx,
                    command_tx,
                );
                activity
            }
            Command::SpawnItems(inventory_id, quantity, class, description) => {
                Command::spawn_items(
                    *inventory_id,
                    *quantity,
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

            Command::UseItem => {
                Command::use_item(player, items, timer, activity, update_tx, command_tx)
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
            Command::FacilityMaintenance(facility_id) => {
                let facility = facilities
                    .get_mut(*facility_id)
                    .expect("Unable to find facility");

                facility.maintenance();
                activity
            }
            Command::None => activity,
            Command::ActivityComplete => self.complete_activity(
                player,
                activity,
                facilities,
                items,
                inventories,
                game_data,
                rng,
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
                        timer,
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
        game_data: &mut GameData,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        let mut activity = activity;
        if let Some(ref mut activity) = &mut activity {
            let (new_updater, _urx) = std::sync::mpsc::channel::<GameUpdate>();
            let (new_commander, _crx) = std::sync::mpsc::channel::<Command>();

            let update_sender = match update_tx {
                Some(updater) => updater,
                None => &new_updater,
            };
            let command_sender = match command_tx {
                Some(commander) => commander,
                None => new_commander,
            };

            activity.complete(
                player,
                facilities,
                items,
                inventories,
                game_data,
                rng,
                update_sender,
                command_sender,
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
                | Command::SaveGame
                | Command::NextTick
                | Command::DisplayTick
                | Command::SpawnItem(_, _, _)
                | Command::RefreshInventory
                | Command::TakeItem(_)
                | Command::DropItem(_)
                | Command::FacilityMaintenance(_)
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
        command_tx: Option<CommandSender>,
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
