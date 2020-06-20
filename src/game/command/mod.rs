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
pub use facility_commands::{
    ActivateFruitPressCommand, ActivateFruitPressFillCommand, ActivateTreeLoggingCommand,
    ActivateTreePickingCommand, OpenChestCommand, OpenFruitPressCommand, TreeType,
};

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

    pub fn move_player<'a>(
        direction: Direction,
        mode: MoveCommandMode,
        player: &'a mut Player,
        map: &'a mut TileMap,
        obstacles: &'a mut BlockingMap,
        facilities: &'a mut FacilityList,
        item_types: &'a ItemTypeList,
        items: &'a mut ItemList,
        inventories: &'a mut InventoryList,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&CommandSender>,
    ) {
        let (dx, dy) = get_deltas_from_direction(direction);

        let command = if mode == MoveCommandMode::Normal || mode == MoveCommandMode::Sneak {
            attempt_to_enter(mode, direction, dx, dy, player, obstacles)
        } else {
            attempt_to_use(
                mode,
                direction,
                dx,
                dy,
                player,
                map,
                obstacles,
                facilities,
                item_types,
                items,
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
) -> Option<Box<dyn CommandHandler<'a> + 'a>> {
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
) -> Option<Box<dyn CommandHandler<'a> + 'a>> {
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
    _mode: MoveCommandMode,
    facing: Direction,
    dx: i32,
    dy: i32,
    player: &'a mut Player,
    obstacles: &'a mut BlockingMap,
) -> Option<Box<dyn CommandHandler<'a> + 'a>> {
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
    mode: MoveCommandMode,
    facing: Direction,
    dx: i32,
    dy: i32,
    player: &'a mut Player,
    map: &'a mut TileMap,
    obstacles: &'a mut BlockingMap,
    facilities: &'a mut FacilityList,
    item_types: &'a ItemTypeList,
    items: &'a mut ItemList,
    inventories: &'a mut InventoryList,
) -> Option<Box<dyn CommandHandler<'a> + 'a>> {
    let mut mode = mode;

    let target_x = player.x + dx;
    let target_y = player.y + dy;

    if can_use_at(
        mode,
        target_x,
        target_y,
        map,
        player,
        facilities,
        items,
        inventories,
    ) {
        use_at(
            mode,
            facing,
            target_x,
            target_y,
            player,
            map,
            obstacles,
            facilities,
            item_types,
            items,
            inventories,
        )
    } else {
        if mode == MoveCommandMode::SneakUse {
            mode = MoveCommandMode::Sneak;
        }
        attempt_to_enter(mode, facing, dx, dy, player, obstacles)
    }
}

fn can_use_at(
    mode: MoveCommandMode,
    x: i32,
    y: i32,
    map: &TileMap,
    player: &Player,
    facilities: &FacilityList,
    _items: &ItemList,
    inventories: &mut InventoryList,
) -> bool {
    match map.at(x, y) {
        tile_map::Tile::ClosedDoor | tile_map::Tile::OpenDoor => true,
        tile_map::Tile::Facility(facility_id) => {
            let facility = facilities.get(facility_id).expect("facility not found");
            match facility.class {
                FacilityClass::ClosedChest => !facility.is_in_use(),
                FacilityClass::AppleTree | FacilityClass::OliveTree => {
                    ActivateTreePickingCommand::can_perform(player, facility)
                        || ActivateTreeLoggingCommand::can_perform(player, facility)
                }
                FacilityClass::PineTree | FacilityClass::OakTree => {
                    ActivateTreeLoggingCommand::can_perform(player, facility)
                }
                FacilityClass::FruitPress => {
                    let inventory = inventories
                        .get_mut(&facility.id)
                        .expect("unable to find inventory");

                    if mode == MoveCommandMode::SneakUse {
                        OpenFruitPressCommand::can_perform(player, facility)
                    } else {
                        ActivateFruitPressFillCommand::can_perform(player, facility)
                            || ActivateFruitPressCommand::can_perform(facility, inventory)
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn use_at<'a>(
    mode: MoveCommandMode,
    __facing: Direction,
    x: i32,
    y: i32,
    player: &'a mut Player,
    map: &'a mut TileMap,
    obstacles: &'a mut BlockingMap,
    facilities: &'a mut FacilityList,
    item_types: &'a ItemTypeList,
    items: &'a mut ItemList,
    inventories: &'a mut InventoryList,
) -> Option<Box<dyn CommandHandler<'a> + 'a>> {
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
                FacilityClass::AppleTree | FacilityClass::OliveTree => {
                    let tree_type = TreeType::from_facility_class(facility.class);

                    // this code cannot be extracted to a separate method
                    //   because of some weird lifetime issue.
                    if ActivateTreePickingCommand::can_perform(player, facility) {
                        Some(Box::new(ActivateTreePickingCommand::new(
                            tree_type,
                            player,
                            facility_id,
                        )))
                    } else if ActivateTreeLoggingCommand::can_perform(player, facility) {
                        Some(Box::new(ActivateTreeLoggingCommand::new(
                            player,
                            facility_id,
                            facilities,
                        )))
                    } else {
                        panic!("Player cannot pick or log!")
                    }
                }
                FacilityClass::PineTree | FacilityClass::OakTree => Some(Box::new(
                    ActivateTreeLoggingCommand::new(player, facility_id, facilities),
                )),
                FacilityClass::FruitPress => match mode {
                    MoveCommandMode::Use => {
                        if ActivateFruitPressFillCommand::can_perform(player, facility) {
                            Some(Box::new(ActivateFruitPressFillCommand::new(
                                player,
                                facility_id,
                                facilities,
                                items,
                                inventories,
                            )))
                        } else {
                            Some(Box::new(ActivateFruitPressCommand::new(
                                player,
                                facility_id,
                                facilities,
                                items,
                                inventories,
                            )))
                        }
                    }
                    MoveCommandMode::SneakUse => Some(Box::new(OpenFruitPressCommand::new(
                        player,
                        facility_id,
                        facilities,
                        item_types,
                        inventories,
                    ))),
                    _ => None,
                },
                _ => None,
            }
        }
        _ => None,
    }
}

pub trait CommandHandler<'a> {
    fn expiration(&self) -> u32 {
        60
    }

    fn create_activity(
        &self,
        _timer: timer::Timer,
        _guard: Guard,
        _update_sender: GameUpdateSender,
        _command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        None
    }

    /// callback to set the activity in the player.
    fn set_activity(&mut self, _activity: Option<Box<dyn Activity>>) {}

    /// get ready to execute the command
    fn prepare_to_execute(&mut self) {}

    /// execute and announce the results of the command.
    /// # Arguments
    /// * update_tx - an optional channel to announce upon.  Can be None for testing purposes.
    fn execute(
        &mut self,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        self.prepare_to_execute();
        let activity = self.perform_execute(update_tx, command_tx);
        self.set_activity(activity);

        if let Some(update_tx) = update_tx {
            self.announce(update_tx);
        }
    }

    /// perform the actions of the command
    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();
        let command_tx = command_tx.unwrap().clone();

        let guard = timer.schedule_repeating(
            chrono::Duration::seconds(self.expiration() as i64),
            move || {
                Command::send(Some(&command_sender), Command::ActivityComplete);
            },
        );

        self.create_activity(timer, guard, update_sender, command_tx)
    }

    /// announce the results through GameUpdates
    fn announce(&self, _update_tx: &std::sync::mpsc::Sender<GameUpdate>) {}
}

pub trait Activity {
    fn description(&self) -> String {
        "Activity".into()
    }

    fn start(&self, update_tx: &GameUpdateSender);

    fn complete(
        &mut self,
        facilities: &mut FacilityList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
    );

    fn on_completion(
        &self,
        player_inventory_id: u64,
        facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
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

impl<'a> CommandHandler<'a> for NullCommand {}

pub mod test_transfer_items;
