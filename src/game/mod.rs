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

lazy_static! {
    static ref GAME_DATA: MutStatic<GameData> = MutStatic::from(GameData::new());
}

fn LIBRARIES() -> &'static mut InventoryList {
    let game_data = GAME_DATA.read().unwrap();
    &mut game_data.inventories
}

pub trait StaticData: 'static {}

impl StaticData for GameData {}

impl GameData {
    pub fn new() -> Self {
        Self {
            player: Player {
                id: 1,
                x: 0,
                y: 0,
                facing: Direction::Up,
                character_type: CharacterType::Player,
                mounting_points: MountingPointMap::new(),
                external_inventory: None,
                endorsements: HashMap::new(),
            },

            map: TileMap::new(),
            obstacles: BlockingMap::new(),
            characters: CharacterList::new(),

            item_class_specifiers: HashMap::new(),
            items: ItemList::new(),
            facilities: FacilityList::new(),
            inventories: HashMap::new(),
        }
    }
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

pub struct GameState {
    pub activity_guard: Option<Guard>,
    pub activity_timer: Option<timer::Timer>,

    pub game_data: &'static GameData,
}

/// Interface methods
impl GameState {
    pub fn get_player_inventory<'a>(&'a mut self) -> &'a mut Inventory {
        self.game_data
            .inventories
            .get_mut(&self.game_data.player.inventory_id())
            .unwrap()
    }
}

impl GameState {
    pub fn new<S: ToString>(level_path: S) -> GameState {
        let (tile_map, mut characters, mut items, mut facilities, mut stored_items) =
            TileMap::load_from_file(level_path.to_string());

        let mut blocking_map = BlockingMap::new();
        blocking_map.refresh(&tile_map);

        let mut game_state = GameState::initialize(
            &mut characters,
            &mut items,
            &mut facilities,
            &mut stored_items,
        );

        game_state.game_data.item_class_specifiers = ItemClassSpecifier::initialize();

        game_state.game_data.map = tile_map;
        game_state.game_data.obstacles = blocking_map;

        game_state
    }

    fn initialize<'a>(
        characters: &'a mut Vec<String>,
        items: &'a mut Vec<String>,
        facilities: &'a mut Vec<String>,
        stored_items: &'a mut Vec<String>,
    ) -> GameState {
        let (player_x, player_y) =
            common::geometry::Point::read_coordinates(characters[0].to_owned());
        let character_vec = Character::read_in_characters(characters);
        let item_vec = Item::read_in_items(items);
        let mut game_state = GameState {
            activity_guard: None,
            activity_timer: None,

            game_data: GET_GAME_DATA(),
        };
        game_state.game_data.player.x = player_x;
        game_state.game_data.player.y = player_y;

        game_state.game_data.player.x = player_x;
        game_state.game_data.player.y = player_y;

        game_state.game_data.characters = character_vec.clone();
        game_state.game_data.characters = character_vec;

        game_state.game_data.items = item_vec.clone();
        game_state.game_data.items = item_vec;

        let (facilities, aliases) =
            Facility::read_in_facilities(facilities, &mut game_state.game_data.inventories);

        game_state.game_data.facilities = facilities;

        let inventories = &mut game_state.game_data.inventories;

        // create the player's inventory
        Inventory::new_into_inventory_list(game_state.game_data.player.id, inventories);

        Item::read_in_stored_items(
            stored_items,
            aliases,
            &mut game_state.game_data.items,
            inventories,
        );

        game_state
    }

    pub fn game_loop(
        update_tx: GameUpdateSender,
        command_rx: std::sync::mpsc::Receiver<Command>,
        command_tx: CommandSender,
    ) {
        let game_state = &mut Self::initialize_game_loop(&update_tx);

        loop {
            let command = command_rx.recv();

            if let Ok(command) = command {
                game_state.game_loop_iteration(&command, Some(&update_tx), Some(&command_tx));
            } else {
                // if receiver is broken, we just bail, ending the game.
                //   eventually, we need to save the game, probably whenever
                //   leaving this loop.
                return;
            }
        }
    }

    fn initialize_game_loop(update_tx: &GameUpdateSender) -> GameState {
        let mut game_state = GameState::new("maps/level1.map");
        Level::new(&mut game_state, Some(update_tx));
        game_state
    }

    /// pub for testing purposes only
    pub fn game_loop_iteration(
        &mut self,
        command: &Command,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        self.abort_activity_if_necessary(command, update_tx);

        match command {
            Command::QuitGame => {
                GameUpdate::send(update_tx, Exit);
                return;
            }
            Command::Move(direction, mode) => {
                Command::move_player(*direction, *mode, self, update_tx, command_tx)
            }
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
                    self,
                    update_tx,
                    command_tx,
                );
            }
            Command::TakeItem(item_index) => {
                Command::pickup_item(*item_index, self, update_tx, command_tx)
            }
            Command::DropItem(item_index) => {
                Command::drop_item(*item_index, self, update_tx, command_tx)
            }
            Command::EquipItem(item_id) => {
                Command::equip_item(*item_id, self, update_tx, command_tx)
            }
            Command::UnequipItem(item_id) => {
                Command::unequip_item(*item_id, self, update_tx, command_tx)
            }
            Command::TransferItem(item_id, src_inventory, dest_inventory) => {
                Command::transfer_item(
                    *item_id,
                    *src_inventory,
                    *dest_inventory,
                    self,
                    update_tx,
                    command_tx,
                );
            }
            Command::TransferAllItems(src_inventory, dest_inventory) => {
                Command::transfer_all_items(
                    *src_inventory,
                    *dest_inventory,
                    self,
                    update_tx,
                    command_tx,
                )
            }
            Command::CloseExternalInventory => Command::close_external_inventory(self, update_tx),
            Command::RefreshInventory => Self::refresh_inventory(self, update_tx),
            Command::AbortActivity | Command::None => {}
        }
    }

    fn abort_activity_if_necessary(
        &mut self,
        command: &Command,
        update_tx: Option<&GameUpdateSender>,
    ) {
        if let None = self.activity_guard {
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
                if let Some(_) = self.activity_guard {
                    self.activity_guard = None;
                }

                GameUpdate::send(update_tx, GameUpdate::ActivityAborted());
            }
        };
    }

    pub fn refresh_inventory(game_state: &game::GameState, update_tx: Option<&GameUpdateSender>) {
        GameUpdate::send(
            update_tx,
            GameUpdate::InventoryUpdated(
                game_state
                    .game_data
                    .inventories
                    .get(&game_state.game_data.player.inventory_id())
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
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        use crate::game::command::CommandHandler;

        let character = &mut self.game_data.player;
        let facing = character.facing;

        let obstacles = &mut self.game_data.obstacles;
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
