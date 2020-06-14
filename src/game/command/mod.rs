use super::*;
use game::blocking_map::*;
use std::boxed::Box;
use std::ops::DerefMut;

pub mod move_command;
pub use move_command::{ChangeFacingCommand, MoveCommand};
pub mod door_commands;
pub use door_commands::{CloseDoorCommand, OpenDoorCommand};
pub mod item_commands;
pub use item_commands::{
    DropCommand, EquipCommand, PickupCommand, TransferAllCommand, TransferItemCommand,
    UnequipCommand,
};
pub mod facility_commands;
pub use facility_commands::{ActivateAppleTreeCommand, OpenChestCommand};

pub type GameUpdateSender = std::sync::mpsc::Sender<GameUpdate>;
pub type CommandSender = std::sync::mpsc::Sender<Command>;

impl Command {
    pub fn send(command_tx: Option<&CommandSender>, command: Command) {
        if let None = command_tx {
            return;
        }
        command_tx
            .unwrap()
            .send(command)
            .expect("unable to send command")
    }

    pub fn move_player(
        direction: Direction,
        mode: MoveCommandMode,
        game_state: &mut game::GameState,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let (dx, dy) = get_deltas_from_direction(direction);

        let command = if mode == MoveCommandMode::Normal || mode == MoveCommandMode::Sneak {
            attempt_to_enter(direction, dx, dy, game_state)
        } else {
            attempt_to_use(direction, dx, dy, game_state)
        };
        if let Some(mut val) = command {
            val.deref_mut().execute(update_tx, command_tx)
        }
    }

    pub fn spawn_item(
        inventory_id: u64,
        class: ItemClass,
        description: &String,
        game_state: &mut game::GameState,
        _update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        _command_tx: Option<&CommandSender>,
    ) {
        let game_data = &mut game_state.game_data;

        let inventory = game_data
            .inventories
            .get_mut(&inventory_id)
            .expect("unable to find inventory");

        inventory.spawn_item(class, description, &mut game_data.items);
    }

    pub fn pickup_item(
        item_index: u64,
        game_state: &mut game::GameState,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        let x = game_state.game_data.player.x;
        let y = game_state.game_data.player.y;

        let item = game_state
            .game_data
            .items
            .find_nth_at(x, y, (item_index - 1) as i32)
            .cloned();

        match item {
            None => println!("item not found"),
            Some(ItemState::Bundle(item, _x, _y)) => {
                let inventory = &mut game_state
                    .game_data
                    .inventories
                    .get_mut(&game_state.game_data.player.inventory_id())
                    .expect("unable to find inventory");
                let mut command =
                    PickupCommand::new(item.id, inventory, &mut game_state.game_data.items);
                command.execute(update_tx, command_tx);
            }
            Some(_) => panic!("unbundled item found"),
        }
    }
    pub fn drop_item(
        item_index: u64,
        game_state: &mut game::GameState,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        let item = &game_state.game_data.items[item_index].clone();
        let x = game_state.game_data.player.x;
        let y = game_state.game_data.player.y;
        match item {
            ItemState::Bundle(_, _, _) => panic!("can't drop something not held"),
            ItemState::Stored(item, inventory_id) => {
                let inventory = &mut game_state
                    .game_data
                    .inventories
                    .get_mut(&inventory_id)
                    .expect("unable to find inventory");
                let mut command =
                    DropCommand::new(item, x, y, inventory, &mut game_state.game_data.items);
                command.execute(update_tx, command_tx);
            }
            ItemState::Equipped(_item_id, _inventory_id) => panic!("can't drop an equipped item!"),
        }
    }

    pub fn equip_item(
        item_index: u64,
        game_state: &mut game::GameState,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        let item = &game_state.game_data.items[item_index].clone();
        match item {
            ItemState::Bundle(_, _, _) => panic!("can't equip something not held"),
            ItemState::Stored(item, inventory_id) => {
                let command = process_equip_item(item, inventory_id, game_state);
                if let Some(mut cmd) = command {
                    cmd.execute(update_tx, command_tx);
                }
            }
            ItemState::Equipped(_item_id, _inventory_id) => {
                panic!("cant re-equip an already equipped item.")
            }
        }
    }

    pub fn unequip_item(
        item_index: u64,
        game_state: &mut game::GameState,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let item_state = &game_state.game_data.items[item_index].clone();
        match item_state {
            ItemState::Bundle(_, _, _) => panic!("can't unequip something not held"),
            ItemState::Stored(_, _) => panic!("cant unequip an already unequipped item."),
            ItemState::Equipped(item, inventory_id) => {
                let command =
                    process_unequip_item(item, inventory_id, game_state, &mut game_state.game_data);
                if let Some(mut cmd) = command {
                    cmd.execute(update_tx, command_tx);
                }
            }
        }
    }

    pub fn transfer_item(
        item_id: u64,
        source_id: u64,
        destination_id: u64,
        game_state: &mut game::GameState,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let item_state = &game_state.game_data.items[item_id];
        let item = ItemState::extract_item(item_state);

        let inventories = &mut game_state.game_data.inventories;
        let items = &mut game_state.game_data.items;

        let mut command =
            TransferItemCommand::new(&item, source_id, destination_id, inventories, items);
        command.execute(update_tx, command_tx);
    }

    pub fn transfer_all_items(
        source_id: u64,
        destination_id: u64,
        game_state: &mut game::GameState,
        update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&CommandSender>,
    ) {
        let inventories = &mut game_state.game_data.inventories;
        let items = &mut game_state.game_data.items;

        let mut command = TransferAllCommand::new(source_id, destination_id, inventories, items);
        command.execute(update_tx, None);
    }

    pub fn close_external_inventory(
        __game_state: &mut game::GameState,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
    ) {
        GameUpdate::send(update_tx, GameUpdate::ExternalInventoryClosed);
    }
}

fn process_equip_item<'a>(
    item: &'a Item,
    inventory_id: &'a u64,
    game_state: &'a mut game::GameState,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let item_class_specifiers = &game_state.game_data.item_class_specifiers;

    let player_mounting_points = &mut game_state.game_data.player.mounting_points;

    let inventory = game_state
        .game_data
        .inventories
        .get_mut(&inventory_id)
        .expect("unable to find inventory");

    let items = &mut game_state.game_data.items;

    Some(std::boxed::Box::new(EquipCommand::new(
        item,
        player_mounting_points,
        item_class_specifiers,
        inventory,
        items,
    )))
}

fn process_unequip_item<'a>(
    item: &'a Item,
    inventory_id: &'a u64,
    game_state: &'a mut game::GameState,
    game_data: &'a mut GameData,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let player_mounting_points = &mut game_state.game_data.player.mounting_points;

    let inventory = game_state
        .game_data
        .inventories
        .get_mut(inventory_id)
        .unwrap();

    Some(std::boxed::Box::new(UnequipCommand::new(
        item.id,
        game_data,
        player_mounting_points,
        inventory,
    )))
}

fn get_deltas_from_direction(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::UpRight => (1, -1),
        Direction::DownRight => (1, 1),
        Direction::DownLeft => (-1, 1),
        Direction::UpLeft => (-1, -1),
    }
}

fn attempt_to_enter<'a>(
    facing: Direction,
    dx: i32,
    dy: i32,
    game_state: &'a mut game::GameState,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let new_x = game_state.game_data.player.x + dx;
    let new_y = game_state.game_data.player.y + dy;

    if game_state.game_data.obstacles.is_blocked_at(new_x, new_y) {
        if facing == game_state.game_data.player.facing {
            return None;
        } else {
            return Some(std::boxed::Box::new(ChangeFacingCommand::new(
                &mut game_state.game_data.player,
                facing,
            )));
        }
    }

    Some(std::boxed::Box::new(MoveCommand::new(
        &mut game_state.game_data.player,
        facing,
        new_x,
        new_y,
        &mut game_state.game_data.obstacles,
    )))
}

fn attempt_to_use<'a>(
    facing: Direction,
    dx: i32,
    dy: i32,
    game_state: &'a mut game::GameState,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let target_x = game_state.game_data.player.x + dx;
    let target_y = game_state.game_data.player.y + dy;

    if can_use_at(target_x, target_y, game_state) {
        use_at(game_state, facing, target_x, target_y)
    } else {
        attempt_to_enter(facing, dx, dy, game_state)
    }
}

fn can_use_at(x: i32, y: i32, game_state: &game::GameState) -> bool {
    match game_state.game_data.map.at(x, y) {
        tile_map::Tile::ClosedDoor | tile_map::Tile::OpenDoor => true,
        tile_map::Tile::Facility(facility_id) => {
            let facility = game_state
                .game_data
                .facilities
                .get(facility_id)
                .expect("facility not found");
            match facility.class {
                FacilityClass::ClosedChest | FacilityClass::AppleTree => !facility.is_in_use(),
                _ => false,
            }
        }
        _ => false,
    }
}

fn use_at<'a>(
    game_state: &'a mut game::GameState,
    __facing: Direction,
    x: i32,
    y: i32,
) -> Option<Box<dyn CommandHandler + 'a>> {
    match game_state.game_data.map.at(x, y) {
        tile_map::Tile::ClosedDoor => Some(Box::new(OpenDoorCommand::new(
            x,
            y,
            &mut game_state.game_data.obstacles,
            &mut game_state.game_data.map,
        ))),
        tile_map::Tile::OpenDoor => Some(Box::new(CloseDoorCommand::new(
            x,
            y,
            &mut game_state.game_data.obstacles,
            &mut game_state.game_data.map,
        ))),
        tile_map::Tile::Facility(facility_id) => {
            let facility = game_state
                .game_data
                .facilities
                .get(facility_id)
                .expect("missing facility");

            match facility.class {
                FacilityClass::ClosedChest => Some(Box::new(OpenChestCommand::new(
                    x,
                    y,
                    &mut game_state.game_data.player,
                    facility_id,
                    &mut game_state.game_data.facilities,
                    &game_state.game_data.inventories,
                ))),
                FacilityClass::AppleTree => {
                    Some(Box::new(ActivateAppleTreeCommand::new(game_state)))
                }
                _ => {
                    println!("facility not matched!");
                    None
                }
            }
        }
        _ => None,
    }
}

pub trait CommandHandler {
    fn can_perform(&self, _game_state: &game::GameState) -> bool {
        true
    }

    /// execute and announce the results of the command.
    /// # Arguments
    /// * update_tx - an optional channel to announce upon.  Can be None for testing purposes.
    fn execute(
        &mut self,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        self.perform_execute(update_tx, command_tx);

        if let Some(update_tx) = update_tx {
            self.announce(update_tx);
        }
    }

    /// perform the actions of the command
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    );

    /// announce the results through GameUpdates
    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>);
}

pub mod test_transfer_items;
