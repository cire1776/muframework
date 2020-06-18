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
pub use facility_commands::{ActivateTreeCommand, OpenChestCommand};

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
        player: &mut Player,
        map: &mut TileMap,
        obstacles: &mut BlockingMap,
        facilities: &mut FacilityList,
        inventories: &mut InventoryList,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let (dx, dy) = get_deltas_from_direction(direction);

        let command = if mode == MoveCommandMode::Normal || mode == MoveCommandMode::Sneak {
            attempt_to_enter(direction, dx, dy, player, obstacles)
        } else {
            attempt_to_use(
                direction,
                dx,
                dy,
                player,
                map,
                obstacles,
                facilities,
                inventories,
            )
        };
        if let Some(mut val) = command {
            val.deref_mut().execute(update_tx, command_tx)
        }
    }

    pub fn spawn_item(
        inventory_id: u64,
        class: ItemClass,
        description: &String,
        inventories: &mut InventoryList,
        items: &mut ItemList,
        _update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        _command_tx: Option<&CommandSender>,
    ) {
        let inventory = inventories
            .get_mut(&inventory_id)
            .expect("unable to find inventory");

        inventory.spawn_item(class, description, items);
    }

    pub fn pickup_item(
        item_index: u64,
        player: &mut Player,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        let x = player.x;
        let y = player.y;

        let item = items.find_nth_at(x, y, (item_index - 1) as i32).cloned();

        match item {
            None => println!("item not found"), // println so that play can continue.
            Some(ItemState::Bundle(item, _x, _y)) => {
                let inventory = &mut inventories
                    .get_mut(&player.inventory_id())
                    .expect("unable to find inventory");
                let mut command = PickupCommand::new(item.id, inventory, items);
                command.execute(update_tx, command_tx);
            }
            Some(_) => panic!("unbundled item found"),
        }
    }
    pub fn drop_item(
        item_index: u64,
        player: &mut Player,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        let item = &items[item_index].clone();
        let x = player.x;
        let y = player.y;
        match item {
            ItemState::Bundle(_, _, _) => panic!("can't drop something not held"),
            ItemState::Stored(item, inventory_id) => {
                let inventory = &mut inventories
                    .get_mut(&inventory_id)
                    .expect("unable to find inventory");
                let mut command = DropCommand::new(item, x, y, inventory, items);
                command.execute(update_tx, command_tx);
            }
            ItemState::Equipped(_item_id, _inventory_id) => panic!("can't drop an equipped item!"),
        }
    }

    pub fn equip_item(
        item_index: u64,
        player: &mut Player,
        item_class_specifiers: &mut ItemClassSpecifierList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&CommandSender>,
    ) {
        let item = &items.clone()[item_index];
        match item {
            ItemState::Bundle(_, _, _) => panic!("can't equip something not held"),
            ItemState::Stored(item, inventory_id) => {
                let command = process_equip_item(
                    item,
                    player,
                    inventory_id,
                    item_class_specifiers,
                    items,
                    inventories,
                );
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
        player: &mut Player,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let item_state = &items.clone()[item_index];
        match item_state {
            ItemState::Bundle(_, _, _) => panic!("can't unequip something not held"),
            ItemState::Stored(_, _) => panic!("cant unequip an already unequipped item."),
            ItemState::Equipped(item, inventory_id) => {
                let command = process_unequip_item(item, inventory_id, player, items, inventories);
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
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let item_state = &items[item_id];
        let item = ItemState::extract_item(item_state);

        let mut command =
            TransferItemCommand::new(&item, source_id, destination_id, inventories, items);
        command.execute(update_tx, command_tx);
    }

    pub fn transfer_all_items(
        source_id: u64,
        destination_id: u64,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&CommandSender>,
    ) {
        let inventories = inventories;

        let mut command = TransferAllCommand::new(source_id, destination_id, inventories, items);
        command.execute(update_tx, None);
    }

    pub fn close_external_inventory(update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>) {
        GameUpdate::send(update_tx, GameUpdate::ExternalInventoryClosed);
    }
}

fn process_equip_item<'a>(
    item: &'a Item,
    player: &'a mut Player,
    inventory_id: &'a u64,
    item_class_specifiers: &'a ItemClassSpecifierList,
    items: &'a mut ItemList,
    inventories: &'a mut InventoryList,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let item_class_specifiers = &item_class_specifiers;

    let inventory = inventories
        .get_mut(&inventory_id)
        .expect("unable to find inventory");

    let items = items;

    Some(std::boxed::Box::new(EquipCommand::new(
        item,
        player,
        item_class_specifiers,
        inventory,
        items,
    )))
}

fn process_unequip_item<'a>(
    item: &'a Item,
    inventory_id: &'a u64,
    player: &'a mut Player,
    items: &'a mut ItemList,
    inventories: &'a mut InventoryList,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let inventory = inventories.get_mut(inventory_id).unwrap();

    Some(std::boxed::Box::new(UnequipCommand::new(
        item.id, inventory, player, items,
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
    player: &'a mut Player,
    obstacles: &'a mut BlockingMap,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let new_x = player.x + dx;
    let new_y = player.y + dy;

    if obstacles.is_blocked_at(new_x, new_y) {
        if facing == player.facing {
            return None;
        } else {
            return Some(std::boxed::Box::new(ChangeFacingCommand::new(
                player, facing,
            )));
        }
    }

    Some(std::boxed::Box::new(MoveCommand::new(
        player, facing, new_x, new_y, obstacles,
    )))
}

fn attempt_to_use<'a>(
    facing: Direction,
    dx: i32,
    dy: i32,
    player: &'a mut Player,
    map: &'a mut TileMap,
    obstacles: &'a mut BlockingMap,
    facilities: &'a mut FacilityList,
    inventories: &'a mut InventoryList,
) -> Option<Box<dyn CommandHandler + 'a>> {
    let target_x = player.x + dx;
    let target_y = player.y + dy;

    if can_use_at(target_x, target_y, map, player, facilities) {
        use_at(
            facing,
            target_x,
            target_y,
            player,
            map,
            obstacles,
            facilities,
            inventories,
        )
    } else {
        attempt_to_enter(facing, dx, dy, player, obstacles)
    }
}

fn can_use_at(x: i32, y: i32, map: &TileMap, player: &Player, facilities: &FacilityList) -> bool {
    match map.at(x, y) {
        tile_map::Tile::ClosedDoor | tile_map::Tile::OpenDoor => true,
        tile_map::Tile::Facility(facility_id) => {
            let facility = facilities.get(facility_id).expect("facility not found");
            match facility.class {
                FacilityClass::ClosedChest => !facility.is_in_use(),
                FacilityClass::AppleTree => ActivateTreeCommand::can_perform(player, facility),
                _ => false,
            }
        }
        _ => false,
    }
}

fn use_at<'a>(
    __facing: Direction,
    x: i32,
    y: i32,
    player: &'a mut Player,
    map: &'a mut TileMap,
    obstacles: &'a mut BlockingMap,
    facilities: &'a mut FacilityList,
    inventories: &'a mut InventoryList,
) -> Option<Box<dyn CommandHandler + 'a>> {
    match map.at(x, y) {
        tile_map::Tile::ClosedDoor => Some(Box::new(OpenDoorCommand::new(x, y, obstacles, map))),
        tile_map::Tile::OpenDoor => Some(Box::new(CloseDoorCommand::new(x, y, obstacles, map))),
        tile_map::Tile::Facility(facility_id) => {
            let facility = facilities.get(facility_id).expect("missing facility");

            match facility.class {
                FacilityClass::ClosedChest => Some(Box::new(OpenChestCommand::new(
                    x,
                    y,
                    player,
                    facility_id,
                    facilities,
                    inventories,
                ))),
                FacilityClass::AppleTree => {
                    Some(Box::new(ActivateTreeCommand::new(player, facility_id)))
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
    ) {
    }

    /// announce the results through GameUpdates
    fn announce(&self, _update_tx: &std::sync::mpsc::Sender<GameUpdate>) {}
}

pub trait Activity {
    fn description(&self) -> String {
        "Activity".into()
    }

    fn start(&self, update_tx: &GameUpdateSender);

    fn complete(&mut self, facilities: &mut FacilityList);

    fn on_completion(
        &self,
        player_inventory_id: u64,
        facility: &mut Facility,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    );

    fn clear_guard(&mut self) {}
}

pub struct NullCommand {}

impl NullCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandHandler for NullCommand {}

pub mod test_transfer_items;
